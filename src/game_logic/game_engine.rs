use std::sync::mpsc;
use server_lib::codec::*;
use server_lib::cards;
use websocket::message::OwnedMessage;
use game_logic::board::BoardStruct;
use game_logic::{self, wordapi};
use rand::Rng;
use rand;
use std;
#[derive(Clone)]
pub enum GameState {
    SubmitWord,
    Buy,
    DrawCard,
    Spell,
}

pub trait GameCon {
    fn tx_send(&self, OwnedMessage);
}
pub struct GameEngine<T: GameCon> {
    players: Vec<Player>,
    connections: Vec<T>,
    gamestates: Vec<GameState>,
}
impl<T> GameEngine<T>
    where T: GameCon
{
    pub fn new(players: Vec<Player>, connections: Vec<T>) -> Self {
        let mut gamestates_v = vec![];
        for _ in &players {
            gamestates_v.push(GameState::Spell);
        }
        GameEngine {
            players: players,
            connections: connections,
            gamestates: gamestates_v,
        }
    }
    pub fn run(&mut self, rx: mpsc::Receiver<(usize, GameCommand)>) {
        let mut last_update = std::time::Instant::now();
        let turn_index = 0;
        let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
        println!("ccc");
        let mut remaining_cards = give_outstarting(&mut self.players, &cardmeta);
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
            if let Ok((player_id, game_command)) = rx.try_recv() {
                let mut need_update = false;
                let mut temp_board = BoardStruct::new(self.players.clone(),
                                                      self.gamestates.clone(),
                                                      remaining_cards.clone(),
                                                      wait_tx.clone());
                let mut type_is_reply = false;
                let &GameCommand { reply, killserver, .. } = &game_command;
                if let Some(_) = reply {
                    type_is_reply = true;
                }

                if let (&GameCommand { use_ink,
                                       use_remover,
                                       ref arranged,
                                       ref wild,
                                       submit_word,
                                       buyoffer,
                                       buylockup,
                                       .. },
                        Some(ref mut _p),
                        Some(ref con),
                        Some(ref mut _gamestate),
                        &None,
                        false) =
                    (&game_command,
                     self.players.get_mut(player_id),
                     self.connections.get(player_id),
                     self.gamestates.get_mut(player_id),
                     &wait_for_input[player_id],
                     type_is_reply) {

                    match _gamestate {
                        &mut &mut GameState::Spell => {
                            game_logic::spell::use_ink_or_remover::<T>(_p,
                                                                       con,
                                                                       use_ink,
                                                                       use_remover);
                            game_logic::spell::arrange(_p, arranged);
                        }
                        &mut &mut GameState::SubmitWord => {
                            //uses tempboard

                        }
                        &mut &mut GameState::Buy => {
                            if let Some(z) = buyoffer {
                                //z top of remaining deck
                                game_logic::purchase::buy_card_from(z,
                                                                    &mut remaining_cards,
                                                                    &cardmeta,
                                                                    _p,
                                                                    player_id,
                                                                    wait_tx.clone());
                            }
                            if let Some(z) = buylockup {
                                //z:index of player.lockup
                                game_logic::purchase::buy_card_from_lockup(z,
                                                                           &cardmeta,
                                                                           _p,
                                                                           player_id,
                                                                           wait_tx.clone());
                            }
                        }
                        _ => {}
                    }
                }
                //save temp_board.players to self.players
                for mut it in temp_board.players.iter_mut().zip(self.players.iter_mut()) {
                    let (ref mut _tb_p, ref mut _p) = it;
                    *_p = _tb_p;
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
            if let Ok(input_request) = wait_rx.recv() {
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
                            let ref con = it;
                            let k: Result<BoardCodec, String> =
                                Ok(BoardCodec { players: self.players.clone() });
                            let g = json!({
                                              "boardstate": k
                                          });
                            con.tx_send(OwnedMessage::Text(g.to_string()));
                        }
                    }
                }
                if wait_for_break {
                    break 'game;
                }
            }
            last_update = std::time::Instant::now();
        }
    }
}
pub fn give_outstarting(players: &mut Vec<Player>,
                        cardmeta: &[cards::ListCard<BoardStruct>; 180])
                        -> Vec<usize> {
    let mut owned_deck = vec![];
    for mut _p in players {
        game_logic::draw_card::player_starting(_p, cardmeta, &mut owned_deck);
    }
    owned_deck
}
