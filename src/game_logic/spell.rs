use codec_lib::codec::*;
use codec_lib::cards;
use codec_lib::cards::{Board, WaitForInputType};
use game_logic::game_engine::GameCon;
use game_logic::wordapi;
use game_logic::board::BoardStruct;

pub fn use_ink_or_remover<T: GameCon>(_board: &mut BoardStruct,
                                      player_id: usize,
                                      con: &T,
                                      use_ink: Option<usize>,
                                      use_remover: Option<usize>,
                                      log: &mut Vec<ClientReceivedMsg>) {
    if let Some(ref _p) = _board.players.get_mut(player_id) {
        if let Some(z) = use_ink {
            _p.inked_cards.push(z);
        } else if let Some(z) = use_remover {
            if _p.inked_cards.contains(&z) {
                _p.hand.push(_p.inked_cards.remove(z));
            } else {
                let k: Result<BoardCodec, String> = Err("cannot remove a card that is not inked"
                                                            .to_owned());

                let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
                h.set_boardstate(k);
                con.tx_send(h, log);

            }
        }
    }

}

pub fn take_card_use_ink(_board:&mut BoardStruct,player_id:usize,_take_card_use_ink:Option<bool>,
               wait_for_input: &mut [WaitForInputType; 4]){
    if let (Some(ref mut _p),Some(true)) = (_board.players.get_mut(player_id),_take_card_use_ink){
        let p_c = _p.clone();
        if (p_c.ink>0) & (!p_c.draft.is_empty()){
            _p.arranged.push((p_c.draft.remove(0),true,None));
            _p.draft = p_c.draft;
            wait_for_input[player_id].push(None);
        } else{
            let j = Some((player_id,"GameCommand error:take_card_use_ink".to_owned(),
            vec![(GameState::Spell,"Ok".to_owned(),Box::new(|_,_|{}))]));
            wait_for_input[player_id].push(j);
        }
    }
}
pub fn arrange(_board: &mut BoardStruct,
               player_id: usize,
               arranged: &Option<Vec<(usize,bool, Option<String>)>>,
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
                                submit_word: Option<bool>)
                                -> Option<bool> {
    if let (Some(_p), Some(true)) = (_board.players.get_mut(player_id), submit_word) {

        let letter_iter = _p.arranged.iter().map(|&(x,_, ref some_wild)| if let &Some(ref _wild) =
            some_wild {
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
