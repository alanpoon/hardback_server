use std::sync::mpsc;
use server_lib::codec::*;
use server_lib::cards;
use server_lib::cards::*;
use websocket::message::OwnedMessage;
use game_logic::board::BoardStruct;
use game_logic;
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
        let owned_deck = give_outstarting(&mut self.players, &cardmeta, &debug_struct);
        let mut remaining_cards = debug_struct.deck_starting(&cardmeta, &owned_deck);
        let mut wait_for_input: [WaitForInputType; 4] = [vec![], vec![], vec![], vec![]];
        let mut wait_for_break = false;
        let mut need_update = false;
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

                let mut offer_row =
                    (0..7).zip(remaining_cards.iter()).map(|(_, c)| c.clone()).collect();
                let mut temp_board = BoardStruct::new(self.players.clone(), offer_row);
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
                        ref mut wait_vec,
                        false) =
                    (&game_command,
                     &mut temp_board,
                     self.connections.get(player_id),
                     self.gamestates.get_mut(player_id),
                     &mut wait_for_input,
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
                            **_gamestate = GameState::Buy;
                            if let Some(true) = game_logic::spell::turn_to_submit(_board,
                                                                                  player_id,
                                                                                  &cardmeta,
                                                                                  submit_word) {
                                game_logic::resolve_cards::resolve_cards(_board,
                                                                         player_id,
                                                                         &cardmeta,
                                                                         wait_vec);
                            }
                        }

                        &mut &mut GameState::Buy => {
                            if let Some(z) = buyoffer {
                                //z top of remaining deck
                                **_gamestate = GameState::DrawCard;
                                game_logic::purchase::buy_card_from(z,
                                                                    &mut remaining_cards,
                                                                    &cardmeta,
                                                                    _board,
                                                                    player_id,
                                                                    wait_vec);

                            }
                            if let Some(z) = buylockup {
                                //z:index of player.lockup
                                **_gamestate = GameState::DrawCard;
                                game_logic::purchase::buy_card_from_lockup(z,
                                                                           &cardmeta,
                                                                           _board,
                                                                           player_id,
                                                                           wait_vec);
                            }
                        }
                        _ => {}
                    }
                }
                //save temp_board.players to self.players
                for it in temp_board.players.iter().zip(self.players.iter_mut()) {
                    let (_tb_p, mut _p) = it;
                    *_p = _tb_p.clone();
                }

                for _missingcard in 0..(7 - temp_board.offer_row.len()) {
                    if let Some(x) = remaining_cards.pop() {
                        temp_board.offer_row.push(x);
                    }
                }
                for mut it in wait_for_input.iter()
                        .zip(self.gamestates.iter_mut())
                        .zip(self.connections.iter()) {
                    let ((ref _w, ref mut _g), ref con) = it;
                    if !_w.is_empty() {
                        **_g = GameState::WaitForReply;
                        let mut temp_vec: Vec<String> = vec![];
                        if let &Some(&(_, ref header, ref option_vec)) = &_w.first() {
                            for &(ref sz, _) in option_vec {
                                temp_vec.push(sz.clone());
                            }
                            let g = json!({
                                              "request": (header.clone(), temp_vec)
                                          });
                            con.tx_send(OwnedMessage::Text(g.to_string()));
                        }
                    }
                }
                if let (&GameCommand { reply, .. }, Some(_p), Some(_gamestate), _wait, true) =
                    (&game_command,
                     self.players.get_mut(player_id),
                     self.gamestates.get_mut(player_id),
                     &mut wait_for_input[player_id],
                     type_is_reply) {
                    if let (Some(_reply), ref mut _wait_vec_vec) = (reply, _wait) {
                        let mut next_gamestate = GameState::DrawCard;
                        let len = _wait_vec_vec.len();
                        if let Some(_wait_vec) = _wait_vec_vec.pop() {
                            next_gamestate = _wait_vec.0;
                            if let Some(&(_, ref _closure)) = _wait_vec.2.get(_reply) {
                                (*_closure)(_p, &mut remaining_cards);
                            }
                            if len == 1 {
                                *_gamestate = next_gamestate;
                            } else {
                                *_gamestate = GameState::WaitForReply;
                            }
                        }
                    }

                }
                println!("killserver P{:?}", killserver.clone());
                if let Some(true) = killserver {
                    wait_for_break = true;
                }
            }
            if need_update {
                for it in self.connections.iter() {
                    let offer_row =
                        (0..7).zip(remaining_cards.iter()).map(|(e, c)| c.clone()).collect();
                    let ref con = it;
                    let k: Result<BoardCodec, String> = Ok(BoardCodec {
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
             if wait_for_break {
                    println!("closing server");
                    break 'game;
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
