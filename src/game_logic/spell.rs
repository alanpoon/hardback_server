use codec_lib::codec::*;
use codec_lib::cards;
use codec_lib::cards::{Board, WaitForInputType,WaitForSingleInput};
use game_logic::game_engine::GameCon;
use game_logic::wordapi;
use game_logic::board::BoardStruct;

pub fn use_remover<T: GameCon>(_board: &mut BoardStruct,
                               player_id: usize,
                               con: &T,
                               use_remover: &Option<Vec<usize>>,
                               log: &mut Vec<ClientReceivedMsg>) {
    if let (Some(ref mut _p), &Some(ref _r)) = (_board.players.get_mut(player_id), use_remover) {
        let mut arrange_c = _p.arranged.clone();
        let arrange_filter = _p.arranged.clone();
        let filter = arrange_filter.iter()
            .filter(|&&(card_i, _, _,_timeless)| _r.contains(&card_i))
            .collect::<Vec<&(usize, bool, Option<String>,bool)>>();
        if filter.is_empty() {
            let k: Result<BoardCodec, String> = Err("cannot remove a card that is not inked"
                                                        .to_owned());
            let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
            h.set_boardstate(k);
            con.tx_send(h, log);
        } else {
            for &mut (ref card_i, ref mut ink_bool, ref mut op_st,ref _timeless) in arrange_c.iter_mut() {
                if _r.contains(&card_i) {
                    *ink_bool = false;
                    *op_st = None;
                }
            }
            _p.arranged = arrange_c;
        }

    }
}
pub fn take_card_use_ink<T: GameCon>(_board: &mut BoardStruct,
                                     player_id: usize,
                                     con: &T,
                                     _take_card_use_ink: &Option<bool>,
                                     wait_for_input:&mut [WaitForInputType; 4],
                                     log: &mut Vec<ClientReceivedMsg>) {
    if let (Some(ref mut _p), &Some(true)) =
        (_board.players.get_mut(player_id), _take_card_use_ink) {
        let p_c = _p.clone();
        if (p_c.ink > 0) & (!p_c.draft.is_empty()) {
            let _g: WaitForSingleInput =
            (p_c.draft.get(0).unwrap().clone(),
             "You need to use this card to form the word. You may not convert this card to wild. If you can't use this card, you may use ink remover to convert this to a wild card.".to_owned(),
             vec![(GameState::TurnToSubmit,
                   "Continue".to_owned(),
                   Box::new(move |ref mut _p, ref mut _rmcards| {
                    let r = _p.draft.remove(0).clone();
                    _p.ink-=1;
                    _p.arranged.push((r,true,None,false));
                   }))]);
        wait_for_input[player_id].push(Some(_g));
        wait_for_input[player_id].push(None);
        } else {
            let k: Result<BoardCodec, String> = Err("Gamecommand Error:take_card_use_ink"
                                                        .to_owned());
            let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
            h.set_boardstate(k);
            con.tx_send(h, log);
        }
    }
}
pub fn arrange(_board: &mut BoardStruct,
               player_id: usize,
               arranged: &Option<Vec<(usize, bool, Option<String>,bool)>>,
               wait_for_input: &mut [WaitForInputType; 4]) {
    if let (Some(_p), mut _w) =
        (_board.players.get_mut(player_id), &mut wait_for_input[player_id]) {
        if let &Some(ref z) = arranged {
            _p.arranged = z.clone();
            _w.push(None);
        }
    }
}

pub fn turn_to_submit<T: Board>(_board: &mut BoardStruct,
                                player_id: usize,
                                cardmeta: &[cards::ListCard<T>; 180],
                                submit_word: &Option<bool>)
                                -> Option<bool> {
    if let (Some(_p), &Some(true)) = (_board.players.get_mut(player_id), submit_word) {

        let letter_iter =
            _p.arranged.iter().map(|&(x, _, ref some_wild,ref _timeless)| if let &Some(ref _wild) = some_wild {
                                       _wild.to_owned()
                                   } else {
                                       cardmeta[x].letter.to_owned()
                                   });
        let k = letter_iter.collect::<String>();
        println!("k {:?}", k);
        Some(wordapi::there_such_word(&k))
    } else {
        None
    }


}
