use server_lib::codec::*;
use server_lib::cards;
use server_lib::cards::{Board, WaitForInputType};
use websocket::message::OwnedMessage;
use game_logic::game_engine::GameCon;
use game_logic::wordapi;
use game_logic::board::BoardStruct;
pub fn use_ink_or_remover<T: GameCon>(_board: &mut BoardStruct,
                                      player_id: usize,
                                      con: &T,
                                      use_ink: Option<usize>,
                                      use_remover: Option<usize>) {
    if let Some(_p) = _board.players.get_mut(player_id) {
        if let Some(z) = use_ink {
            _p.inked_cards.push(z);
        } else if let Some(z) = use_remover {
            if _p.inked_cards.contains(&z) {
                _p.hand.push(_p.inked_cards.remove(z));
            } else {
                let k: Result<BoardCodec, String> = Err("cannot remove a card that is not inked"
                                                            .to_owned());
                let g = json!({
                                  "boardstate": k
                              });
                con.tx_send(OwnedMessage::Text(g.to_string()));

            }
        }
    }

}
pub fn arrange(_board: &mut BoardStruct,
               player_id: usize,
               arranged: &Option<Vec<(usize, Option<String>)>>,
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

        let letter_iter = _p.arranged.iter().map(|&(x, ref some_wild)| if let &Some(ref _wild) =
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
