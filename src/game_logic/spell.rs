use server_lib::codec::*;
use server_lib::cards;
use websocket::message::OwnedMessage;
use game_logic::game_engine::GameCon;
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
pub fn arrange(_p: &mut Player, arranged: &Option<Vec<usize>>) {
    if let &Some(ref z) = arranged {
        _p.arranged = z.clone();
    }

}
pub fn wild(_p: &mut Player, wild: Option<(usize, String)>) {
    if let Some((card_index, ref replacement)) = wild {
        let mut wild_vec = vec![];
        for _c in &_p.arranged {
            if *_c == card_index {
                wild_vec.push(Some(replacement.clone()));
            } else {
                wild_vec.push(None);
            }
        }
        _p.wild = wild_vec;
    }
}
