use std::sync::mpsc;
use codec_lib::codec::*;
use codec_lib::cards;
use codec_lib::cards::*;
use game_logic::board::BoardStruct;
use game_logic;
use std;

pub trait GameCon {
    fn tx_send(&self, ClientReceivedMsg, &mut Vec<ClientReceivedMsg>);
}
pub trait TheDraft {
    fn player_starting(&self, &mut Player, &[cards::ListCard<BoardStruct>; 180], &mut Vec<usize>);
    fn deck_starting(&self, &[cards::ListCard<BoardStruct>; 180], &Vec<usize>) -> Vec<usize>;
    fn ticks(&self) -> Option<u16>;
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
    pub fn run<D: TheDraft>(&mut self,
                            rx: mpsc::Receiver<(usize, GameCommand)>,
                            debug_struct: D,
                            log: &mut Vec<ClientReceivedMsg>) {
        let mut last_update = std::time::Instant::now();
        let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
        let mut turn_index = 0;
        let owned_deck = give_outstarting(&mut self.players, &cardmeta, &debug_struct);
        let mut remaining_cards = debug_struct.deck_starting(&cardmeta, &owned_deck);
        let mut wait_for_input: [WaitForInputType; 4] = [vec![], vec![], vec![], vec![]];
        let mut wait_for_break = false;
        let mut ticks: Option<u16> = debug_struct.ticks();
        let mut count_rec = 0;
        'game: loop {
            let sixteen_ms = std::time::Duration::new(1, 0);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);

            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            game_logic::draw_card::redraw_cards_to_hand_size(&mut self.players,
                                                             &mut self.gamestates,
                                                             &mut turn_index);
            game_logic::draw_card::uncover_cards::<T>(&mut self.players,
                                                      &mut self.gamestates,
                                                      &self.connections,
                                                      &cardmeta,
                                                      &remaining_cards,
                                                      &mut wait_for_input,
                                                      turn_index,
                                                      ticks,
                                                      log);

            game_logic::draw_card::update_gamestates(&mut self.gamestates,
                                                     &self.connections,
                                                     &self.players,
                                                     &remaining_cards,
                                                     turn_index,
                                                     ticks,
                                                     log);


            while let Ok((player_id, game_command)) = rx.try_recv() {
                count_rec = 0;
                println!("receive from client {}", count_rec);
                count_rec += 1;

                let mut temp_board = BoardStruct::new(self.players.clone(), &remaining_cards);
                let mut type_is_reply = false;
                let &GameCommand { reply, killserver, .. } = &game_command;
                if let Some(_) = reply {
                    type_is_reply = true;
                }

                if let (&GameCommand { use_ink,
                                       use_remover,
                                       ref arranged,
                                       submit_word,
                                       lockup,
                                       buy_offer,
                                       buy_lockup,
                                       trash_other,
                                       putback_discard,
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
                     type_is_reply.clone()) {
                    match _gamestate {
                        &mut &mut GameState::Spell => {
                            println!("spell");
                            game_logic::spell::use_ink_or_remover::<T>(_board,
                                                                       player_id,
                                                                       con,
                                                                       use_ink,
                                                                       use_remover,
                                                                       log);
                            game_logic::spell::arrange(_board, player_id, arranged, wait_vec);
                        }
                        &mut &mut GameState::TurnToSubmit => {
                            println!("TurnToSubmit");
                            game_logic::spell::use_ink_or_remover::<T>(_board,
                                                                       player_id,
                                                                       con,
                                                                       use_ink,
                                                                       use_remover,
                                                                       log);
                            game_logic::spell::arrange(_board, player_id, arranged, wait_vec);
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
                            println!("Buy");
                            if let Some((true, z)) = buy_offer {
                                //z top of remaining deck
                                **_gamestate = GameState::DrawCard;
                                game_logic::purchase::buy_card_from(z,
                                                                    &mut remaining_cards,
                                                                    &cardmeta,
                                                                    _board,
                                                                    player_id,
                                                                    wait_vec);
                                //broadcast for every purchase
                                if let Some(ref mut _w) = wait_vec.get_mut(player_id) {
                                    if _w.len() == 0 {
                                        _w.push(None);
                                    }
                                }

                            } else {
                                **_gamestate = GameState::DrawCard;
                            }

                        }
                        &mut &mut GameState::LockUp => {
                            if let Some((true, z)) = lockup {
                                //z:index of player.lockup
                                println!("there is lockup");
                                game_logic::purchase::lockup_a_card(z,
                                                                    _board,
                                                                    player_id,
                                                                    &mut remaining_cards,
                                                                    wait_vec,
                                                                    &mut type_is_reply);
                                **_gamestate = GameState::Buy;
                            } else {
                                **_gamestate = GameState::Buy;
                            }
                        }
                        &mut &mut GameState::TrashOther => {
                            if let Some((true, z)) = trash_other {
                                game_logic::purchase::trash_another_card(z,
                                                                         _board,
                                                                         player_id,
                                                                         wait_vec,
                                                                         &mut type_is_reply);
                                **_gamestate = GameState::Buy;
                                //broadcast for every TrashOther
                                if let Some(ref mut _w) = wait_vec.get_mut(player_id) {
                                    if _w.len() == 0 {
                                        _w.push(None);
                                    }
                                }
                            }
                        }
                        &mut &mut GameState::PutBackDiscard(ind, responsible) => {
                            if let Some(true) = putback_discard {
                                game_logic::purchase::putback_discard(ind,
                                                                      responsible,
                                                                      _board,
                                                                      player_id,
                                                                      &mut remaining_cards,
                                                                      wait_vec,
                                                                      &mut type_is_reply);
                            }
                        }
                        _ => {
                            println!("stateless, {:?}", _gamestate.clone());
                        }
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
                if !type_is_reply {
                    println!("before broadcast len {:?}", wait_for_input[player_id].len());
                    continue_to_broadcast::<T>(&mut wait_for_input[player_id],
                                               &self.connections,
                                               &remaining_cards,
                                               self.players.clone(),
                                               self.gamestates.clone(),
                                               turn_index,
                                               ticks,
                                               log);
                    println!("after broadcast len {:?}", wait_for_input[player_id].len());

                    if let (_w, Some(_g), Some(_con)) =
                        (&mut wait_for_input[player_id],
                         self.gamestates.get_mut(player_id),
                         self.connections.get(player_id)) {
                        continue_to_prob::<T>(player_id, _w, _g, _con, ticks, log);
                        println!("after prob len {:?}", _w.len());

                    }

                }

                let mut next_gamestate = GameState::DrawCard;
                if let (&GameCommand { reply, .. }, true) = (&game_command, type_is_reply) {
                    if let Some(_reply) = reply {
                        if let (Some(_p), Some(_gamestate), mut _wait_vec_vec) =
                            (self.players.get_mut(player_id),
                             self.gamestates.get_mut(player_id),
                             &mut wait_for_input[player_id]) {
                            if let Some(_wait_vec) = _wait_vec_vec.remove(0) {
                                if let Some(&(ref next_gstate, _, ref _closure)) =
                                    _wait_vec.2.get(_reply) {
                                    (*_closure)(_p, &mut remaining_cards);
                                    next_gamestate = next_gstate.clone();
                                }
                            }
                        }
                        let len = wait_for_input[player_id].len();
                        println!("reply's wait vec len:{}", len);
                        if wait_for_input[player_id].len() == 1 {
                            if let Some(_gamestate) = self.gamestates.get_mut(player_id) {
                                *_gamestate = next_gamestate;
                            }
                        }
                        continue_to_broadcast::<T>(&mut wait_for_input[player_id],
                                                   &self.connections,
                                                   &remaining_cards,
                                                   self.players.clone(),
                                                   self.gamestates.clone(),
                                                   turn_index,
                                                   ticks,
                                                   log);

                        if let (Some(_con), Some(_gamestate)) =
                            (self.connections.get(player_id), self.gamestates.get_mut(player_id)) {
                            continue_to_prob::<T>(player_id,
                                                  &wait_for_input[player_id],
                                                  _gamestate,
                                                  &_con,
                                                  ticks,
                                                  log);
                        }
                        println!("End of reply's wait vec len:{}",
                                 wait_for_input[player_id].len());
                    }
                }

                if let Some(true) = killserver {
                    wait_for_break = true;
                }

            }
            count_rec += 1;
            if let Some(mut tick) = ticks {
                tick += 1;
            }

            if (wait_for_break) & (count_rec >= 15) {
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
pub fn continue_to_prob<T: GameCon>(player_num: usize,
                                    wait_for_input_p: &WaitForInputType,
                                    _g: &mut GameState,
                                    con: &T,
                                    ticks: Option<u16>,
                                    mut log: &mut Vec<ClientReceivedMsg>)
                                    -> bool {
    if let Some(&Some(ref __w)) = wait_for_input_p.first() {
        println!("solo");
        *_g = GameState::WaitForReply;
        let mut temp_vec: Vec<String> = vec![];
        let &(card_index, ref header, ref option_vec) = __w;
        for &(_, ref sz, _) in option_vec {
            temp_vec.push(sz.clone());
        }

        let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
        h.set_request((player_num, card_index, header.clone(), temp_vec, ticks));
        con.tx_send(h, log);
        true
    } else {
        false
    }
}
pub fn continue_to_broadcast<T: GameCon>(wait_for_input_p: &mut WaitForInputType,
                                         con_vec: &Vec<T>,
                                         remaining_cards: &Vec<usize>,
                                         players: Vec<Player>,
                                         gamestates: Vec<GameState>,
                                         turn_index: usize,
                                         ticks: Option<u16>,
                                         log: &mut Vec<ClientReceivedMsg>) {
    let mut remove_first = false;
    match wait_for_input_p.first() {
        Some(&Some(ref x)) => {
            println!("there is some {:?}", x.0.clone());
        }
        Some(&None) => {
            remove_first = true;
            for it in con_vec.iter() {
                let offer_row =
                    (0..7).zip(remaining_cards.iter()).map(|(e, c)| c.clone()).collect();
                let ref con = it;
                let k: Result<BoardCodec, String> = Ok(BoardCodec {
                                                           players: players.clone(),
                                                           gamestates: gamestates.clone(),
                                                           offer_row: offer_row,
                                                           turn_index: turn_index,
                                                           ticks: ticks,
                                                       });
                let g = json!({
                                  "boardstate": k
                              });
                let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
                h.set_boardstate(k);
                con.tx_send(h, log);
            }

            println!("there is really none");
        }
        None => {
            println!("there is none");
        }
    }
    if let Some(&None) = wait_for_input_p.first() {}
    if remove_first {
        wait_for_input_p.remove(0);
        println!("remove_first");
    }
}
