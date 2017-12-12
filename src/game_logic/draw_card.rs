use codec_lib::codec::*;
use codec_lib::cards::{self, WaitForInputType};
use game_logic::board::BoardStruct;
use game_logic::game_engine::{continue_to_prob, continue_to_broadcast, GameCon};
use game_logic;
use std::collections::HashMap;
#[cfg(not(test))]
pub fn redraw_cards_to_hand_size(players: &mut Vec<Player>,
                                 unknown: &mut [Vec<usize>; 4],
                                 gamestates: &mut Vec<GameState>,
                                 turn_index: &mut usize) {
    use rand::Rng;
    use rand;
    let player_num = players.len();
    println!("gahh {:?}",gamestates.clone());
    for mut it in players.iter_mut().enumerate().zip(gamestates.iter_mut()) {
        let ((ref _index, ref mut _p), ref mut game_state) = it;
        //((x,y), z)
        match game_state {
            &mut &mut GameState::PreDrawCard => {
                _p.discard.extend(_p.hand.clone());
                _p.hand = vec![];
                for _ in 0usize..5usize {
                    if let Some(n) = unknown[_index.clone()].pop() {
                        _p.hand.push(n);
                    } else {
                        let mut rng = rand::thread_rng();
                        unknown[_index.clone()] = _p.discard.clone();
                        _p.discard = vec![];
                        rng.shuffle(&mut unknown[_index.clone()]);
                        if let Some(n) = unknown[_index.clone()].pop() {
                            _p.hand.push(n);
                        }
                    }
                }
                _p.skip_cards = vec![];
                _p.arranged = vec![];
                _p.draftlen = unknown[_index.clone()].len();
                if _p.draftlen == 0 {
                    let mut rng = rand::thread_rng();
                    unknown[_index.clone()] = _p.discard.clone();
                    rng.shuffle(&mut unknown[_index.clone()]);
                    _p.discard = vec![];
                    _p.draftlen = unknown[_index.clone()].len();
                }
                //unused coins will be converted into ink
                _p.ink += _p.coin;
                _p.coin = 0;
            }
            _ => {}
        }
    }
}
#[cfg(test)]
pub fn redraw_cards_to_hand_size(players: &mut Vec<Player>,
                                 unknown: &mut [Vec<usize>; 4],
                                 gamestates: &mut Vec<GameState>,
                                 turn_index: &mut usize) {

    let player_num = players.len();
    for mut it in players.iter_mut().enumerate().zip(gamestates.iter_mut()) {
        let ((ref _index, ref mut _p), ref mut game_state) = it;
        //((x,y), z)
        match game_state {
            &mut &mut GameState::PreDrawCard => {
                _p.discard.extend(_p.hand.clone());
                _p.hand = vec![];
                for _ in 0usize..5usize {
                    if let Some(n) = unknown[_index.clone()].pop() {
                        _p.hand.push(n);
                    } else {
                        unknown[_index.clone()] = _p.discard.clone();
                        _p.discard = vec![];
                        if let Some(n) = unknown[_index.clone()].pop() {
                            _p.hand.push(n);
                        }
                    }
                }

                _p.arranged = vec![];
                _p.skip_cards = vec![];
                _p.draftlen = unknown[_index.clone()].len();
                if _p.draftlen == 0 {
                    unknown[_index.clone()] = _p.discard.clone();
                    _p.discard = vec![];
                    _p.draftlen = unknown[_index.clone()].len();
                }
                if *turn_index < player_num - 1 {
                    *turn_index += 1;
                } else {
                    *turn_index = 0;
                }
                //unused coins will be converted into ink
                _p.ink += _p.coin;
                _p.coin = 0;
              //  **game_state=GameState::DrawCard;
                println!("change");
            }
            _ => {}
        }
    }
}
pub fn update_gamestates<T: GameCon>(gamestates: &mut Vec<GameState>,
                                     cons: &HashMap<usize, T>,
                                     players: &Vec<Player>,
                                     remaining_cards: &Vec<usize>,
                                     turn_index: &mut usize,
                                     ticks: Option<u16>,
                                     log: &mut Vec<ClientReceivedMsg>) {
    let mut needtempboardcast = false;
    let mut need_turn_index = false;
    let player_num = players.len();
    let still_processing = !gamestates.iter().filter(|x|x==&&GameState::PreDrawCard).collect::<Vec<&GameState>>().is_empty();
    if still_processing{
    if let (Some(ref _p),Some(ref mut _g)) = (players.get(turn_index.clone()),gamestates.get_mut(turn_index.clone())){
            if let GameState::PreDrawCard=**_g{
                needtempboardcast = true;
                need_turn_index = true;
                if let Some(_c) = cons.get(turn_index){
                          let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
                        h.set_hand(_p.hand.clone());
                        _c.tx_send(h, log);
                }
                if *turn_index < player_num - 1 {
                        *turn_index += 1;
                        while !cons.contains_key(turn_index){
                            if *turn_index<player_num-1{
                                *turn_index += 1;
                            }
                        }
                    } else {
                        *turn_index = 0;
                    }
                    **_g=GameState::DrawCard;
                
            }
        }
       for (_i,_g) in gamestates.iter_mut().enumerate(){
            if _i !=*turn_index{
                *_g = GameState::Spell;
            } else {
                *_g=GameState::TurnToSubmit;
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

        let game_state_c = gamestates.clone();
        println!("game_sssstate {:?}", game_state_c);
        match wait_vec[player_that_responsible].first() {
            Some(&Some(ref x)) => {
                println!("there is some {:?}", x.0.clone());
            }
            Some(&None) => {
                println!("there is really none");
            }
            _ => {
                println!("there is no first");
            }

        }
        if let (Some(ref mut _g), ref mut _w) =
            (gamestates.get_mut(player_that_responsible), &mut wait_vec[player_that_responsible]) {
            continue_to_broadcast::<T>(_w,
                                       &connections,
                                       &remaining_cards,
                                       players.clone(),
                                       game_state_c.clone(),
                                       turn_index,
                                       ticks,
                                       log);
            if let (_w, Some(_con)) = (_w, connections.get(&player_that_responsible)) {
                continue_to_prob::<T>(player_that_responsible, _w, _g, _con, ticks, log);
            }
        }
    }


}
