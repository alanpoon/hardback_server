use codec_lib::codec::*;
use codec_lib::cards;
use codec_lib::cards::{Board, WaitForInputType, WaitForSingleInput};
use game_logic::game_engine::GameCon;
use game_logic::wordapi;
use game_logic::board::BoardStruct;
use std::cmp;

pub fn use_remover<T: GameCon>(_board: &mut BoardStruct,
                               player_id: usize,
                               con: &T,
                               use_remover: Option<Vec<usize>>,
                               wait_for_input: &mut [WaitForInputType; 4],
                               log: &mut Vec<ClientReceivedMsg>) {
    if let Some(vec_r) = use_remover {
        if let Some(ref mut _p) = _board.players.get_mut(player_id) {
            let p_c = _p.clone();
            let arrange_filter = _p.arranged.clone();
            let filter = arrange_filter.iter()
                .filter(|&&(card_i, _, _, _timeless)| vec_r.contains(&card_i))
                .collect::<Vec<&(usize, bool, Option<String>, bool)>>();
            if filter.is_empty() {
                let k: Result<BoardCodec, String> = Err("cannot remove a card that is not inked"
                                                            .to_owned());
                let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
                h.set_boardstate(k);
                con.tx_send(h, log);
            } else if p_c.remover == 0 {
                let k: Result<BoardCodec, String> = Err("Not enough remover".to_owned());
                let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
                h.set_boardstate(k);
                con.tx_send(h, log);
            } else {
                for _r in vec_r {
                    let _g: WaitForSingleInput =
                    WaitForSingleInput(_r.clone(),
            GameState::PreWaitForReply,
             "You may convert this inked card back to a normal card using a remover token. You may add it back to your hand or use it to form word or a wild card.".to_owned(),
             vec![(GameState::TurnToSubmit,
                   "Continue".to_owned(),
                   Box::new(move |ref mut _p, ref mut _rmcards,ref mut _unknown| {
                    _p.remover-=1;
                    for &mut (ref _card,ref mut _ink_bool,_,_) in _p.arranged.iter_mut(){
                        if *_card==_r.clone(){
                            *_ink_bool = false;
                        }
                    }
                    
                   }))]);
                    wait_for_input[player_id].push(Some(_g));
                }
            }


        }
    }

}
pub fn take_card_use_ink<T: GameCon>(_board: &mut BoardStruct,
                                     player_id: usize,
                                     con: &T,
                                     _take_card_use_ink: &Option<bool>,
                                     unknown: &mut Vec<usize>,
                                     wait_for_input: &mut [WaitForInputType; 4],
                                     log: &mut Vec<ClientReceivedMsg>) {
    if let (Some(ref mut _p), &Some(true)) =
        (_board.players.get_mut(player_id), _take_card_use_ink) {
        if (_p.ink > 0) & (!unknown.is_empty()) {
            let _g: WaitForSingleInput =
            WaitForSingleInput(unknown.get(0).unwrap().clone(),
            GameState::PreWaitForReply,
             "You need to use this card to form the word. You may not convert this card to wild. If you can't use this card, you may use ink remover to convert this to a wild card.".to_owned(),
             vec![(GameState::TurnToSubmit,
                   "Continue".to_owned(),
                   Box::new(move |ref mut _p, ref mut _rmcards,mut _unknown| {
                    let r = _unknown.remove(0).clone();
                    _p.ink-=1;
                    _p.draftlen=_unknown.len();
                    _p.arranged.push((r,true,None,false));
                   }))]);
            wait_for_input[player_id].push(Some(_g));
        } else {
            let k: Result<BoardCodec, String> = Err("Gamecommand Error:take_card_use_ink"
                                                        .to_owned());
            let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
            h.set_boardstate(k);
            con.tx_send(h, log);
        }
    }
}
// may be used interchangably with personal
pub fn arrange(_board: &mut BoardStruct,
               player_id: usize,
               arranged: &Option<Vec<(usize, bool, Option<String>, bool)>>,
               wait_for_input: &mut [WaitForInputType; 4]) {
    if let (Some(_p), mut _w) =
        (_board.players.get_mut(player_id), &mut wait_for_input[player_id]) {
        if let &Some(ref z) = arranged {
            _p.arranged = z.clone();
        }
    }
}
pub fn personal(_board: &mut BoardStruct,
                player_id: usize,
                personal: &Option<Personal>,
                wait_for_input: &mut [WaitForInputType; 4]) {
    if let (Some(_p), mut _w) =
        (_board.players.get_mut(player_id), &mut wait_for_input[player_id]) {
        if let &Some(ref z) = personal {
            _p.arranged = z.arranged.clone();
            _p.hand = z.hand.clone();
        }
    }
}
pub fn turn_to_submit<T: Board>(_board: &mut BoardStruct,
                                player_id: usize,
                                cardmeta: &[cards::ListCard<T>; 180],
                                submit_word: &Option<bool>)
                                -> Option<(bool, String)> {
    let mut max_literacy = 0;
    let mut need_update = false;
    for _p in _board.players.iter() {
        if _p.literacy_award > max_literacy {
            max_literacy = _p.literacy_award;
        }
    }
    let result = if let (Some(_p), &Some(true)) =
        (_board.players.get_mut(player_id), submit_word) {
        let letter_iter = _p.arranged.iter().map(|&(x, _, ref some_wild, ref _timeless)| {
            if let &Some(ref _wild) = some_wild {
                _wild.to_owned()
            } else {
                cardmeta[x].letter.to_owned()
            }
        });
        let k = letter_iter.collect::<String>();
        let word_len = k.chars().count();
        if (word_len >= 7) & (max_literacy < word_len) {
            _p.literacy_award = cmp::min(word_len, 12);
            need_update = true;
        }
        println!("k {:?}", k);
        Some((wordapi::there_such_word(&k), k))
    } else {
        None
    };
    if need_update {
        for (_i, _p) in _board.players.iter_mut().enumerate() {
            if _i != player_id {
                _p.literacy_award = 0;
            }
        }
    }
    result
}
