use codec_lib::codec::*;
use codec_lib::cards::{self, WaitForInputType};
use game_logic::board::BoardStruct;
use game_logic::game_engine::{continue_to_prob, continue_to_broadcast, GameCon};
use game_logic;
use std::collections::HashMap;

pub fn redraw_cards_to_hand_size(randseed: Option<&[usize]>,
                                 players: &mut Vec<Player>,
                                 unknown: &mut [Vec<usize>; 4],
                                 init_hand:&mut [Vec<usize>;4],
                                 cardmeta:&[cards::ListCard<BoardStruct>; 180],
                                 gamestates: &mut Vec<GameState>,
                                 turn_index: &mut usize) {
    use rand::{SeedableRng, StdRng, Rng, thread_rng};
    let player_num = players.len();
    for mut it in players.iter_mut().enumerate().zip(gamestates.iter_mut()) {
        let ((ref _index, ref mut _p), ref mut game_state) = it;
        //((x,y), z)
        match game_state {
            &mut &mut GameState::PreDrawCard => {
                //remove cards that are timeless from discard pile
                let timeless_arr = _p.discard.clone().iter().filter(|&&x|cardmeta[x].timeless).map(|x|x.clone()).collect::<Vec<usize>>();
                 println!("d_c {:?},timeless_arr:{:?}",_p.discard.clone(),timeless_arr.clone());
                _p.timeless_classic.extend(timeless_arr);
                _p.discard = _p.discard.clone().iter().filter(|&&x|!cardmeta[x].timeless).map(|x|x.clone()).collect::<Vec<usize>>();
                _p.discard.extend(init_hand[_index.clone()].clone());
                let mut new_hand =vec![];
                if let Some(_randseed) = randseed {
                    let mut rng: StdRng = SeedableRng::from_seed(_randseed);
                    for _ in 0usize..5usize {
                        if let Some(n) = unknown[_index.clone()].pop() {
                            new_hand.push(n);
                        } else {
                            unknown[_index.clone()] = _p.discard.clone();
                            _p.discard = vec![];
                            rng.shuffle(&mut unknown[_index.clone()]);
                            if let Some(n) = unknown[_index.clone()].pop() {
                                new_hand.push(n);
                            }
                        }
                    }
                } else {
                    let mut rng = thread_rng();
                    for _ in 0usize..5usize {
                        if let Some(n) = unknown[_index.clone()].pop() {
                            new_hand.push(n);
                        } else {
                            unknown[_index.clone()].extend(_p.discard.clone());
                            _p.discard = vec![];
                            rng.shuffle(&mut unknown[_index.clone()]);
                            if let Some(n) = unknown[_index.clone()].pop() {
                                new_hand.push(n);
                            }
                        }
                    }
                }
                if _p.hand.len() < 5 {
                    println!("draw error unknonw {:?}", unknown[_index.clone()].clone());
                }
                _p.hand=new_hand.clone();
                init_hand[_index.clone()]=new_hand;
                _p.skip_cards = vec![];
                _p.arranged = vec![];
                _p.draftlen = unknown[_index.clone()].len();
                //unused coins will be converted into ink
                _p.ink = _p.ink + (_p.coin as f32 / 3.0).floor() as usize;
                _p.coin = 0;
            }
            _ => {}
        }
    }
}

pub fn update_gamestates<T: GameCon>(gamestates: &mut Vec<GameState>,
                                     cons: &HashMap<usize, T>,
                                     players: &Vec<Player>,
                                     remaining_cards: &Vec<usize>,
                                     wait_vec: &mut [WaitForInputType; 4],
                                     turn_index: &mut usize,
                                     ticks: Option<u16>,
                                     log: &mut Vec<ClientReceivedMsg>) {
    let mut needtempboardcast = false;
    let mut need_turn_index = false;
    let player_num = players.len();
    let still_processing = !gamestates.iter()
                                .filter(|x| x == &&GameState::PreWaitForReply)
                                .collect::<Vec<&GameState>>()
                                .is_empty();
    if still_processing {
        if let Some(ref mut _g) = gamestates.get_mut(turn_index.clone()) {
            **_g = GameState::WaitForReply;
        }
        continue_to_broadcast::<T>(&cons,
                                   &remaining_cards,
                                   players.clone(),
                                   gamestates.clone(),
                                   turn_index.clone(),
                                   ticks,
                                   log);
        if let (Some(&Some(ref __w)), Some(con)) =
            (wait_vec[turn_index.clone()].first(), cons.get(&turn_index.clone())) {
            let &(_card_index, ref wait_state, ref _header, ref _option_vec) = __w;
            let mut temp_vec: Vec<String> = vec![];
            for &(_, ref sz, _) in _option_vec {
                temp_vec.push(sz.clone());
            }
            let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
            h.set_request((turn_index.clone(), _card_index, _header.clone(), temp_vec, ticks));
            con.tx_send(h, log);
        }

    }
    let still_processing = !gamestates.iter()
                                .filter(|x| x == &&GameState::PreBuy)
                                .collect::<Vec<&GameState>>()
                                .is_empty();
    if still_processing {
        if let Some(ref mut _g) = gamestates.get_mut(turn_index.clone()) {
            needtempboardcast = true;
            **_g = GameState::Buy;

        }
    }
    let still_processing = !gamestates.iter()
                                .filter(|x| x == &&GameState::PreTurnToSubmit)
                                .collect::<Vec<&GameState>>()
                                .is_empty();
    if still_processing {
        if let Some(ref mut _g) = gamestates.get_mut(turn_index.clone()) {
            needtempboardcast = true;
            **_g = GameState::TurnToSubmit;
        }
    }
    let still_processing = !gamestates.iter()
                                .filter(|x| x == &&GameState::PreSpell)
                                .collect::<Vec<&GameState>>()
                                .is_empty();
    if still_processing {
        if let Some(ref mut _g) = gamestates.get_mut(turn_index.clone()) {
            needtempboardcast = true;
            **_g = GameState::Spell;
        }
    }
    let still_processing = !gamestates.iter()
                                .filter(|x| if let &&GameState::PreTrashOther(_) = x {
                                            true
                                        } else {
                                            false
                                        })
                                .collect::<Vec<&GameState>>()
                                .is_empty();
    if still_processing {
        if let Some(ref mut _g) = gamestates.get_mut(turn_index.clone()) {
            if let GameState::PreTrashOther(z) = _g.clone().clone() {
                needtempboardcast = true;
                **_g = GameState::TrashOther(z);
            }
        }
    }
    let still_processing = !gamestates.iter()
                                .filter(|x| if let &&GameState::PrePutBackDiscard(_, _) = x {
                                            true
                                        } else {
                                            false
                                        })
                                .collect::<Vec<&GameState>>()
                                .is_empty();
    if still_processing {
        if let Some(ref mut _g) = gamestates.get_mut(turn_index.clone()) {
            if let GameState::PrePutBackDiscard(z, j) = _g.clone().clone() {
                needtempboardcast = true;
                **_g = GameState::PutBackDiscard(z, j);
            }
        }
    }
    let still_processing = !gamestates.iter()
                                .filter(|x| x == &&GameState::PreDrawCard)
                                .collect::<Vec<&GameState>>()
                                .is_empty();
    if still_processing {
        if let (Some(ref _p), Some(ref mut _g)) =
            (players.get(turn_index.clone()), gamestates.get_mut(turn_index.clone())) {
            if let GameState::PreDrawCard = **_g {
                needtempboardcast = true;
                need_turn_index = true;
                if let Some(_c) = cons.get(turn_index) {
                    let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
                    h.set_hand(_p.hand.clone());
                    _c.tx_send(h, log);
                }
                println!("turn index has changed");
                if *turn_index < player_num - 1 {
                    *turn_index += 1;
                    while !cons.contains_key(turn_index) {
                        if *turn_index < player_num - 1 {
                            *turn_index += 1;
                        }
                    }
                } else {
                    *turn_index = 0;
                }
                **_g = GameState::DrawCard;

            }
        }

        for (_i, _g) in gamestates.iter_mut().enumerate() {
            if _i != *turn_index {
                *_g = GameState::Spell;
            } else {
                *_g = GameState::TurnToSubmit;
            }
        }
    }
    if needtempboardcast {
        for (_, _con) in cons.iter() {
            let offer_row = (0..7).zip(remaining_cards.iter()).map(|(_, c)| c.clone()).collect();
            if need_turn_index {
                let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
                h.set_turn_index(turn_index.clone());
                _con.tx_send(h, log);
            }
            let k: Result<BoardCodec, String> = Ok(BoardCodec {
                                                       players: players.clone(),
                                                       gamestates: gamestates.clone(),
                                                       offer_row: offer_row,
                                                       turn_index: turn_index.clone(),
                                                       ticks: ticks,
                                                   });

            let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
            h.set_boardstate(k);
            _con.tx_send(h, log);
        }
    }

}
pub fn uncover_cards<T: GameCon>(players: &mut Vec<Player>,
                                 gamestates: &mut Vec<GameState>,
                                 connections: &HashMap<usize, T>,
                                 cardmeta: &[cards::ListCard<BoardStruct>; 180],
                                 remaining_cards: &Vec<usize>,
                                 wait_vec: &mut [WaitForInputType; 4],
                                 turn_index: usize,
                                 ticks: Option<u16>,
                                 log: &mut Vec<ClientReceivedMsg>) {

    let mut tempboard = BoardStruct::new(players.clone(), &remaining_cards);
    let mut player_that_responsible = None;
    for (player_id, ref mut _gamestates) in
        (0..tempboard.players.len()).zip(gamestates.iter_mut()) {
        match _gamestates {
            &mut &mut GameState::ResolveAgain(_, _) => {
                player_that_responsible = Some(player_id);
                **_gamestates = GameState::Buy;
                println!("resolveagain {:?}", players.clone().get(player_id));
                game_logic::resolve_cards::resolve_cards(&mut tempboard,
                                                         player_id,
                                                         &cardmeta,
                                                         wait_vec);
            }
            _ => {}
        }
    }
    if let Some(player_that_responsible) = player_that_responsible {
        for it in tempboard.players.iter().zip(players.iter_mut()) {
            let (_tb_p, mut _p) = it;
            *_p = _tb_p.clone();
        }

        if let (Some(_con), Some(_g)) =
            (connections.get(&player_that_responsible), gamestates.get_mut(player_that_responsible)) {
            continue_to_prob::<T>(player_that_responsible,
                                  &mut wait_vec[player_that_responsible],
                                  _g,
                                  _con,
                                  ticks,
                                  log);
        }
    }


}
