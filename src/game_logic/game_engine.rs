use std::sync::mpsc;
use server_lib::codec::*;
use server_lib::cards;
use server_lib::cards::*;
use websocket::message::OwnedMessage;
use game_logic::board::BoardStruct;
use game_logic::{self, wordapi};
use rand::Rng;
use rand;
use std;

pub trait GameCon {
    fn tx_send(&self, OwnedMessage);
}
pub trait TheDraft {
    fn player_starting(&self, &mut Player, &[cards::ListCard<BoardStruct>; 180], &mut Vec<usize>);
    fn deck_starting(&self, &[cards::ListCard<BoardStruct>; 180], &Vec<usize>) -> Vec<usize>;
}
pub struct GameEngine<T: GameCon> {
    players: Vec<Player>,
    connections: Vec<T>,
    gamestates: Vec<GameState>,
    turn_index: usize,
}
impl<T> GameEngine<T>
    where T: GameCon
{
    pub fn new(players: Vec<Player>, connections: Vec<T>) -> Self {
        let mut gamestates_v = vec![];
        let mut c = 0;
        for _ in &players {
            if c == 0 {
                gamestates_v.push(GameState::TurnToSubmit);
            } else {
                gamestates_v.push(GameState::Spell);
            }
            c += 1;
        }
        GameEngine {
            turn_index: 0,
            players: players,
            connections: connections,
            gamestates: gamestates_v,
        }
    }
    pub fn run<D: TheDraft>(&mut self, rx: mpsc::Receiver<(usize, GameCommand)>, debug_struct: D) {
        let mut last_update = std::time::Instant::now();
        let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
        println!("ccc");
        let mut owned_deck = give_outstarting(&mut self.players, &cardmeta, &debug_struct);
        let mut remaining_cards = debug_struct.deck_starting(&cardmeta, &owned_deck);
        let (wait_tx, wait_rx) = mpsc::channel();
        let mut wait_for_input: [Option<Vec<Box<Fn(&mut Player, &mut Vec<usize>)>>>; 4] =
            [None, None, None, None];
        let mut wait_for_break = false;
        'game: loop {
            let sixteen_ms = std::time::Duration::from_millis(1000);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);

            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }
            game_logic::draw_card::redraw_cards_to_hand_size(&mut self.players,
                                                             &mut self.gamestates);
            while let Ok((player_id, game_command)) = rx.try_recv() {
                println!("receive from client");
                let mut need_update = false;
                let mut offer_row =
                    (0..7).zip(remaining_cards.iter()).map(|(e, c)| c.clone()).collect();
                let mut temp_board = BoardStruct::new(self.players.clone(),
                                                      self.gamestates.clone(),
                                                      offer_row,
                                                      wait_tx.clone());
                let mut type_is_reply = false;
                let &GameCommand { reply, killserver, .. } = &game_command;
                if let Some(_) = reply {
                    type_is_reply = true;
                }

                if let (&GameCommand { use_ink,
                                       use_remover,
                                       ref arranged,
                                       submit_word,
                                       buyoffer,
                                       buylockup,
                                       .. },
                        ref mut _board,
                        Some(ref con),
                        Some(ref mut _gamestate),
                        &None,
                        false) =
                    (&game_command,
                     &mut temp_board,
                     self.connections.get(player_id),
                     self.gamestates.get_mut(player_id),
                     &wait_for_input[player_id],
                     type_is_reply) {

                    match _gamestate {
                        &mut &mut GameState::Spell => {
                            game_logic::spell::use_ink_or_remover::<T>(_board,
                                                                       player_id,
                                                                       con,
                                                                       use_ink,
                                                                       use_remover);
                            game_logic::spell::arrange(_board, player_id, arranged);
                        }
                        &mut &mut GameState::TurnToSubmit => {
                            game_logic::spell::use_ink_or_remover::<T>(_board,
                                                                       player_id,
                                                                       con,
                                                                       use_ink,
                                                                       use_remover);
                            game_logic::spell::arrange(_board, player_id, arranged);
                            if let Some(true) = game_logic::spell::turn_to_submit(_board,
                                                                                  player_id,
                                                                                  &cardmeta,
                                                                                  submit_word) {
                                game_logic::resolve_cards::resolve_cards(_board,
                                                                         player_id,
                                                                         &cardmeta,
                                                                         wait_tx.clone());
                            }
                        }
                        &mut &mut GameState::SubmitWordWaitForReply => {
                            //uses tempboard

                        }
                        &mut &mut GameState::Buy => {
                            if let Some(z) = buyoffer {
                                //z top of remaining deck
                                game_logic::purchase::buy_card_from(z,
                                                                    &mut remaining_cards,
                                                                    &cardmeta,
                                                                    _board,
                                                                    player_id,
                                                                    wait_tx.clone());

                            }
                            if let Some(z) = buylockup {
                                //z:index of player.lockup
                                game_logic::purchase::buy_card_from_lockup(z,
                                                                           &cardmeta,
                                                                           _board,
                                                                           player_id,
                                                                           wait_tx.clone());
                            }
                        }
                        _ => {}
                    }
                }
                //save temp_board.players to self.players
                for mut it in temp_board.players.iter().zip(self.players.iter_mut()) {
                    let (_tb_p, mut _p) = it;
                    *_p = _tb_p.clone();
                }
                for mut it in temp_board.gamestates.iter().zip(self.gamestates.iter_mut()) {
                    let (_tb_p, mut _p) = it;
                    *_p = _tb_p.clone();
                }
                println!("emp_board.offer_row.len()){}", temp_board.offer_row.len());
                for _missingcard in 0..(7 - temp_board.offer_row.len()) {
                    if let Some(x) = remaining_cards.pop() {
                        temp_board.offer_row.push(x);
                    }
                }
                if let (&GameCommand { reply, .. }, Some(_p), _wait, true) =
                    (&game_command,
                     self.players.get_mut(player_id),
                     &mut wait_for_input[player_id],
                     type_is_reply) {
                    if let (Some(_reply), &&mut Some(ref _wait_vec)) = (reply, &_wait) {
                        if let Some(_closure) = _wait_vec.get(_reply) {
                            (*_closure)(_p, &mut remaining_cards);
                        }
                    }
                    *_wait = None;
                }
                wait_tx.clone().send(None).unwrap();
                println!("killserver P{:?}", killserver.clone());
                if let Some(true) = killserver {
                    wait_for_break = true;
                }
            }
            while let Ok(input_request) = wait_rx.try_recv() {
                println!("recev input_request");
                match input_request {
                    Some((player_id, header, option_vec)) => {
                        // request Input
                        if let Some::<&T>(ref con) = self.connections.get(player_id) {

                            let mut temp_vec: Vec<String> = vec![];
                            let mut temp_wait_for_input: Vec<Box<Fn(&mut Player,&mut Vec<usize>)>> = vec![];
                            for (sz, sb) in option_vec {
                                temp_vec.push(sz);
                                temp_wait_for_input.push(sb);
                            }
                            wait_for_input[player_id] = Some(temp_wait_for_input);
                            let g = json!({
                                              "request": (header, temp_vec)
                                          });
                            con.tx_send(OwnedMessage::Text(g.to_string()));

                        }
                    }
                    None => {
                        //just update boardstae
                        for it in self.connections.iter() {
                            let offer_row = (0..7)
                                .zip(remaining_cards.iter())
                                .map(|(e, c)| c.clone())
                                .collect();
                            let ref con = it;
                            let k: Result<BoardCodec, String> =
                                Ok(BoardCodec {
                                       players: self.players.clone(),
                                       gamestates: self.gamestates.clone(),
                                       offer_row: offer_row,
                                   });
                            let g = json!({
                                              "boardstate": k
                                          });
                            con.tx_send(OwnedMessage::Text(g.to_string()));
                        }
                    }
                }
                if wait_for_break {
                    println!("closing server");
                    break 'game;
                }
            }
            last_update = std::time::Instant::now();
        }
    }
}
pub fn give_outstarting<D: TheDraft>(players: &mut Vec<Player>,
                                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                                     debug_struct: &D)
                                     -> Vec<usize> {
    let mut owned_deck = vec![];
    for mut _p in players {
        debug_struct.player_starting(_p, cardmeta, &mut owned_deck);
    }
    owned_deck
}