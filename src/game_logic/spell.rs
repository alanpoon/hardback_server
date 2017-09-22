use server_lib::codec::*;
use server_lib::cards;
use server_lib::cards::Board;
use websocket::message::OwnedMessage;
use game_logic::game_engine::GameCon;
use game_logic::wordapi;
pub fn use_ink_or_remover<T: GameCon>(_p: &mut Player,
                                      con: &T,
                                      use_ink: Option<usize>,
                                      use_remover: Option<usize>) {
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
pub fn arrange(_p: &mut Player, arranged: &Option<Vec<(usize, Option<String>)>>) {
    if let &Some(ref z) = arranged {
        _p.arranged = z.clone();
    }

}

pub fn turn_to_submit<T: Board>(_p: &mut Player,
                                cardmeta: &[cards::ListCard<T>; 180],
                                submit_word: Option<bool>) {
    println!("turn to submit{:?}", _p.arranged.clone());
    if let Some(true) = submit_word {
        let letter_iter = _p.arranged.iter().map(|&(x, ref some_wild)| if let &Some(ref _wild) =
            some_wild {
                                                     _wild.to_owned()
                                                 } else {
                                                     cardmeta[x].letter.to_owned()
                                                 });
        let k = letter_iter.collect::<String>();
        println!("k {:?}", k);
        if wordapi::there_such_word(&k) {
            println!("there is such word");
        } else {
            println!("there is no such word");
        }
    }
}
