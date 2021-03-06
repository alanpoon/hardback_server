use std::sync::mpsc;
use codec_lib::codec::*;
use codec_lib::cards;
use codec_lib::cards::*;
use game_logic::board::BoardStruct;
use game_logic;
use std;
use std::collections::HashMap;

pub trait GameCon {
    fn tx_send(&self, ClientReceivedMsg, &mut Vec<ClientReceivedMsg>);
}
pub trait TheDraft {
    fn player_starting(&self,
                       &mut Player,
                       &mut Vec<usize>,
                       &[cards::ListCard<BoardStruct>; 180],
                       &mut Vec<usize>);
    fn deck_starting(&self, &[cards::ListCard<BoardStruct>; 180], &Vec<usize>) -> Vec<usize>;
    fn ticks(&self) -> Option<u16>;
    fn show_draft(&self) -> (bool, Option<&[usize]>); //show_draft,withrandseed
    fn push_notification(&self) -> bool;
}
pub struct GameEngine<T: GameCon> {
    players: Vec<Player>,
    connections: HashMap<usize, T>,
    gamestates: Vec<GameState>,
    unknown: [Vec<usize>; 4],
    init_hand: [Vec<usize>; 4],
}
impl<T> GameEngine<T>
    where T: GameCon
{
    pub fn new(players: Vec<Player>, connections: HashMap<usize, T>) -> Self {
        let mut gamestates_v = vec![];
        for _ in &players {
            gamestates_v.push(GameState::ShowDraft);
        }
        GameEngine {
            players: players,
            connections: connections,
            gamestates: gamestates_v,
            unknown: [vec![], vec![], vec![], vec![]],
            init_hand:[vec![],vec![],vec![],vec![]]
        }
    }
    pub fn run<D: TheDraft>(&mut self,
                            rx: mpsc::Receiver<(usize, GameCommand)>,
                            debug_struct: D,
                            log: &mut Vec<ClientReceivedMsg>) {

        let mut last_update = std::time::Instant::now();
        let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
        let mut turn_index = 0;
        let owned_deck = give_outstarting(&mut self.players,
                                          &mut self.unknown,
                                          &cardmeta,
                                          &debug_struct);
        let mut remaining_cards = debug_struct.deck_starting(&cardmeta, &owned_deck);
        let mut wait_for_input: [WaitForInputType; 4] = [vec![], vec![], vec![], vec![]];
        let mut wait_for_break = false;
        let ticks: Option<u16> = debug_struct.ticks();
        if let (true, _randseed) = debug_struct.show_draft() {
            game_logic::show_draft::give_player_index(&self.connections, log);
            game_logic::show_draft::broadcast(&mut self.gamestates,
                                              &self.connections,
                                              &self.players,
                                              &mut self.unknown,
                                              log);

        } else {
            for (index, game_state) in self.gamestates.iter_mut().enumerate() {
                if index == 0 {
                    *game_state = GameState::TurnToSubmit;
                } else {
                    *game_state = GameState::Spell;
                }

            }
        }
        let mut count_rec = 0;
        let mut player_60: Option<usize> = None;
        let mut end_game_notified = false;
        let mut turn_after_winner = 0;
        'game: loop {
            let sixteen_ms = std::time::Duration::new(1, 0);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);

            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            game_logic::draw_card::redraw_cards_to_hand_size(debug_struct.show_draft().1,
                                                             &mut self.players,
                                                             &mut self.unknown,
                                                             &mut self.init_hand,
                                                             &cardmeta,
                                                             &mut self.gamestates,
                                                             &mut turn_index);
            game_logic::end_game::first_to_60(&self.players,
                                              &self.connections,
                                              &mut player_60,
                                              &mut end_game_notified,
                                              log);

            game_logic::end_game::show_result(&mut self.players,
                                              &mut self.gamestates,
                                              &self.connections,
                                              &player_60,
                                              &mut turn_after_winner,
                                              turn_index,
                                              ticks,
                                              log);

            /*  game_logic::draw_card::uncover_cards::<T>(&mut self.players,
                                                      &mut self.gamestates,
                                                      &self.connections,
                                                      &cardmeta,
                                                      &remaining_cards,
                                                      &mut wait_for_input,
                                                      turn_index,
                                                      ticks,
                                                      log);
                                                      */
            game_logic::draw_card::update_gamestates(&mut self.gamestates,
                                                     &self.connections,
                                                     &self.players,
                                                     &remaining_cards,
                                                     &mut wait_for_input,
                                                     &mut turn_index,
                                                     ticks,
                                                     log);

            while let Ok((player_id, game_command)) = rx.try_recv() {
                count_rec = 0;
                println!("receive from client {}", count_rec);
                count_rec += 1;

                let mut temp_board = BoardStruct::new(self.players.clone(), &remaining_cards);
                let mut type_is_reply = false;
                let mut is_notification: Option<String> = None;
                let &GameCommand { reply, killserver, .. } = &game_command;
                if let Some(_) = reply {
                    type_is_reply = true;
                    let z = wait_for_input[player_id]
                        .iter()
                        .map(|x| if let &Some(ref _x) = x {
                                 Some((_x.1.clone(), _x.2.clone()))
                             } else {
                                 None
                             })
                        .collect::<Vec<Option<(GameState, String)>>>();
                    println!("zzz :{:?}", z);
                }

                if let (&GameCommand { ref go_to_shuffle,
                                       ref take_card_use_ink,
                                       ref use_ink,
                                       ref use_remover,
                                       ref arranged,
                                       ref personal,
                                       ref submit_word,
                                       ref lockup,
                                       ref buy_offer,
                                       ref buy_lockup,
                                       ref trash_other,
                                       ref putback_discard,
                                       ref exit_game,
                                       .. },
                        ref mut _board,
                        Some(ref con),
                        Some(ref mut _gamestate),
                        ref mut unknown,
                        ref mut wait_vec,
                        false) =
                    (&game_command,
                     &mut temp_board,
                     self.connections.get(&player_id),
                     self.gamestates.get_mut(player_id),
                     &mut self.unknown[player_id],
                     &mut wait_for_input,
                     type_is_reply.clone()) {
                    match _gamestate {
                        &mut &mut GameState::ShowDraft => {
                            game_logic::show_draft::go_to_shuffle::<T>(debug_struct.show_draft().1,
                                                                       _board,
                                                                       player_id,
                                                                       con,
                                                                       _gamestate,
                                                                       go_to_shuffle,
                                                                       unknown,
                                                                       ticks,
                                                                       log);
                        }
                        &mut &mut GameState::Spell => {
                            game_logic::spell::take_card_use_ink::<T>(_board,
                                                                      player_id,
                                                                      con,
                                                                      take_card_use_ink,
                                                                      unknown,
                                                                      wait_vec,
                                                                      log);
                            game_logic::spell::use_remover::<T>(_board,
                                                                player_id,
                                                                con,
                                                                use_remover.clone(),
                                                                wait_vec,
                                                                log);
                            game_logic::spell::arrange(_board, player_id, arranged, wait_vec);
                            game_logic::spell::personal(_board, player_id, personal, wait_vec);
                            if (arranged.is_some()) | (take_card_use_ink.is_some()) |
                               (use_remover.is_some()) |
                               (personal.is_some()) {
                                **_gamestate = GameState::PreSpell;
                            }
                        }
                        &mut &mut GameState::TurnToSubmit => {
                            println!("TurnToSubmit");
                            game_logic::spell::take_card_use_ink::<T>(_board,
                                                                      player_id,
                                                                      con,
                                                                      take_card_use_ink,
                                                                      unknown,
                                                                      wait_vec,
                                                                      log);

                            game_logic::spell::use_remover::<T>(_board,
                                                                player_id,
                                                                con,
                                                                use_remover.clone(),
                                                                wait_vec,
                                                                log);
                            game_logic::spell::arrange(_board, player_id, arranged, wait_vec);
                            game_logic::spell::personal(_board, player_id, personal, wait_vec);
                            if (arranged.is_some()) | (take_card_use_ink.is_some()) |
                               (use_remover.is_some()) |
                               (personal.is_some()) {
                                **_gamestate = GameState::PreTurnToSubmit;
                            }
                            if let Some((true, _kstring)) =
                                game_logic::spell::turn_to_submit(_board,
                                                                  player_id,
                                                                  &cardmeta,
                                                                  submit_word) {
                                game_logic::resolve_cards::resolve_cards(_board,
                                                                         player_id,
                                                                         &cardmeta,
                                                                         wait_vec);
                                if wait_vec[player_id].is_empty() {
                                    **_gamestate = GameState::PreBuy;
                                    if debug_struct.push_notification() {
                                        let mut _st = "Player ".to_owned();
                                        _st.push_str(&(player_id + 1).to_string());
                                        _st.push_str(" has formed a word [");
                                        _st.push_str(&_kstring);
                                        _st.push_str("]");
                                        is_notification = Some(_st);
                                    }
                                }
                            }
                        }

                        &mut &mut GameState::Buy => {
                            println!("Buy");
                            if let &Some((true, z)) = buy_offer {
                                //z top of remaining deck
                                **_gamestate = GameState::PreDrawCard;
                                game_logic::purchase::buy_card_from(z,
                                                                    &mut remaining_cards,
                                                                    &cardmeta,
                                                                    _board,
                                                                    player_id,
                                                                    wait_vec);

                            } else {
                                **_gamestate = GameState::PreDrawCard;
                            }

                        }
                        &mut &mut GameState::LockUp => {
                            if let &Some((true, z)) = lockup {
                                //z:index of player.lockup
                                println!("there is lockup");
                                game_logic::purchase::lockup_a_card(z,
                                                                    _board,
                                                                    player_id,
                                                                    &mut remaining_cards,
                                                                    wait_vec,
                                                                    &mut type_is_reply);
                                **_gamestate = GameState::PreBuy;
                            } else {
                                **_gamestate = GameState::PreBuy;
                            }
                        }
                        &mut &mut GameState::TrashOther(_) => {
                            if let &Some((true, z)) = trash_other {

                                game_logic::purchase::trash_another_card(z,
                                                                         _board,
                                                                         player_id,
                                                                         wait_vec,
                                                                         &mut type_is_reply);
                                **_gamestate = GameState::PreBuy;
                            }
                        }
                        &mut &mut GameState::PutBackDiscard(ind, responsible) => {
                            if let &Some(true) = putback_discard {
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


                if let (Some(&Some(ref __w)), Some(ref mut _g)) =
                    (wait_for_input[player_id.clone()].first(),
                     self.gamestates.get_mut(player_id.clone())) {
                    let &(_card_index, ref wait_state, ref _header, ref _option_vec) = __w;
                    **_g = wait_state.clone();
                }

                if let Some(ref _s) = is_notification {
                    println!("there is notification");
                    for (_, _con) in &self.connections {
                        let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
                        h.set_notification(_s.clone());
                        _con.tx_send(h, log);
                    }
                }
                let mut next_gamestate = None;
                if let (&GameCommand { reply, .. }, true) = (&game_command, type_is_reply) {
                    if let Some(_reply) = reply {

                        if let (Some(_p), Some(_gamestate), mut _wait_vec_vec) =
                            (self.players.get_mut(player_id),
                             self.gamestates.get_mut(player_id),
                             &mut wait_for_input[player_id]) {
                            if let Some(_wait_vec) = _wait_vec_vec.remove(0) {
                                if let Some(&(ref next_gstate, _, ref _closure)) =
                                    _wait_vec.3.get(_reply) {
                                    (*_closure)(_p,
                                                &mut remaining_cards,
                                                &mut self.unknown[player_id]);
                                    println!("inside closure, {:?} .{:?}",
                                             _wait_vec.0.clone(),
                                             _wait_vec.2.clone());
                                    next_gamestate = Some(next_gstate.clone());
                                }


                            }
                        }

                    }
                    if let (Some(_k), true) = (next_gamestate,
                                               wait_for_input[player_id].is_empty()) {
                        if let Some(_gamestate) = self.gamestates.get_mut(player_id) {
                            *_gamestate = _k;
                        }
                    }
                }

                if let Some(true) = killserver {
                    wait_for_break = true;
                }

            }
            count_rec += 1;
            if let Some(mut _tick) = ticks {
                _tick += 1;
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
                                     unknown: &mut [Vec<usize>; 4],
                                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                                     debug_struct: &D)
                                     -> Vec<usize> {
    let mut owned_deck = vec![];
    for (_index, mut _p) in players.iter_mut().enumerate() {
        debug_struct.player_starting(_p, &mut unknown[_index], cardmeta, &mut owned_deck);
    }
    owned_deck
}
pub fn continue_to_prob<T: GameCon>(player_num: usize,
                                    wait_for_input_p: &WaitForInputType,
                                    _g: &mut GameState,
                                    con: &T,
                                    ticks: Option<u16>,
                                    log: &mut Vec<ClientReceivedMsg>)
                                    -> bool {
    if let Some(&Some(ref __w)) = wait_for_input_p.first() {
        let mut temp_vec: Vec<String> = vec![];
        let &(card_index, ref wait_state, ref header, ref option_vec) = __w;
        *_g = wait_state.clone();
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
pub fn continue_to_broadcast<T: GameCon>(con_vec: &HashMap<usize, T>,
                                         remaining_cards: &Vec<usize>,
                                         players: Vec<Player>,
                                         gamestates: Vec<GameState>,
                                         turn_index: usize,
                                         ticks: Option<u16>,
                                         log: &mut Vec<ClientReceivedMsg>) {
    for it in con_vec.iter() {
        let offer_row = (0..7).zip(remaining_cards.iter()).map(|(e, c)| c.clone()).collect();
        let (_i, ref con) = it;

        //replace PreDrawCard with DrawCard
        let mut gc = gamestates.clone();
        for _h in gc.iter_mut() {
            if let &mut GameState::PreDrawCard = _h {
                *_h = GameState::DrawCard;
            }
        }
        let k: Result<BoardCodec, String> = Ok(BoardCodec {
                                                   players: players.clone(),
                                                   gamestates: gc,
                                                   offer_row: offer_row,
                                                   turn_index: turn_index,
                                                   ticks: ticks,
                                               });
        let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
        h.set_boardstate(k);
        con.tx_send(h, log);
        println!("plll {:?}", players.clone());
    }
}
