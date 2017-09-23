use std::sync::mpsc;
use server_lib::codec::*;
use server_lib::cards;
use websocket::message::OwnedMessage;
use game_logic::board::BoardStruct;
use game_logic::resolve_cards;
pub fn resolve_purchase_giveables(card_index: usize,
                                  cardmeta: &[cards::ListCard<BoardStruct>; 180],
                                  _p: &mut Player) {
    if let cards::GIVEABLE::VP(_vp) = cardmeta[card_index].purchase_giveables {
        _p.vp += _vp;
    }
}


pub fn buy_card_from(position_index: usize,
                     from: &mut Vec<usize>,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     _board: &mut BoardStruct,
                     player_id: usize,
                     wait_tx: mpsc::Sender<Option<(usize,
                                                   String,
                                                   Vec<(String,
                                                        Box<Fn(&mut Player,
                                                               &mut Vec<usize>)>)>)>>) {
    println!("purchasing!");
    if let (Some(_p), Some(_gamestate)) =
        (_board.players.get_mut(player_id), _board.gamestates.get_mut(player_id)) {
        println!("player coin {}", _p.coin.clone());
        let res: Option<Result<(usize,
                                String,
                                Vec<(String, Box<Fn(&mut Player, &mut Vec<usize>)>)>),
                               String>> = match from.get(position_index) {

            Some(&_c) => {
                println!("goo");
                match cardmeta[_c].cost as f64 <= _p.coin as f64 + (_p.ink as f64 / 3.0).floor() {
                    true => {
                        println!("ooo");
                        match cardmeta[_c].cost <= _p.coin {
                            true => {
                                _p.coin -= cardmeta[_c].cost;
                                _p.discard.push(from.remove(position_index));
                                resolve_cards::resolve_purchase(_c, _p, &cardmeta, _gamestate);
                                None
                            }
                            false => {
                                let j = "You do not have enough coin to buy this card, you may trade in 3 ink for one coin to buy this".to_owned();

                                Some(Ok((player_id,
                                         j,
                                         vec![("Trade in 3 ink for one coin to buy this?"
                                                   .to_owned(),
                                               Box::new(move |ref mut p, ref mut rmcards| {
                                                            p.discard.push(rmcards.remove(_c));
                                                        })),
                                              ("No".to_owned(),
                                               Box::new(|ref mut p, ref mut rmcards| {}))])))

                            }
                        }
                    }
                    false => Some(Err("Cannot afford the card".to_owned())),
                }
            }
            None => Some(Err("Cannot find the card selected".to_owned())),
        };

        if let Some(Ok(a)) = res {
            wait_tx.send(Some(a)).unwrap();
        }
    }

}
pub fn buy_card_from_lockup(position_index: usize,
                            cardmeta: &[cards::ListCard<BoardStruct>; 180],
                            _board: &mut BoardStruct,
                            player_id: usize,
                            wait_tx: mpsc::Sender<Option<(usize,
                                                          String,
                                                          Vec<(String,
                                                               Box<Fn(&mut Player,
                                                                      &mut Vec<usize>)>)>)>>) {
    if let Some(_p) = _board.players.get_mut(player_id) {
        let mut card_index = 2;
        if let Some(&_c) = _p.lockup.get(position_index) {
            card_index = _c;
        } else {
            println!("lockup does not have this card");
        }
        let res: Option<Result<(usize,
                                String,
                                Vec<(String, Box<Fn(&mut Player, &mut Vec<usize>)>)>),
                               String>> = match cardmeta[card_index].cost as f64 <=
              _p.coin as f64 + (_p.ink as f64 / 3.0).floor() {
            true => {
                match cardmeta[card_index].cost <= _p.coin {
                    true => {
                        _p.coin -= cardmeta[card_index].cost;
                        _p.discard.push(card_index);
                        _p.lockup.remove(position_index);
                        None
                    }
                    false => {
                        let j = "You do not have enough coin to buy this card, you may trade in 3 ink for one coin to buy this".to_owned();
                        let cost = cardmeta[card_index].cost.clone();
                        Some(Ok((player_id,
                                 j,
                                 vec![("Trade in 3 ink for one coin to buy this?".to_owned(),
                                       Box::new(move |ref mut p, _| {
                            let coin_left = p.coin;
                            let remainder = cost - coin_left;
                            p.coin = 0;
                            p.ink -= remainder * 3;
                            p.discard.push(card_index);
                            p.lockup.remove(position_index);

                        })),
                                      ("No".to_owned(), Box::new(|ref mut p, _| {}))])))

                    }
                }
            }
            false => Some(Err("Cannot afford the card".to_owned())),
        };
        if let Some(Ok(a)) = res {
            wait_tx.send(Some(a)).unwrap();
        }
    }

}