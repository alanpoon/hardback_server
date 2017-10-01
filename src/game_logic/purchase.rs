use std::sync::mpsc;
use server_lib::codec::*;
use server_lib::cards;
use server_lib::cards::{WaitForInputType, WaitForSingleInput};
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
                     wait_for_input: &mut [WaitForInputType; 4]) {
    println!("purchasing!");
    if let Some(_p) = _board.players.get_mut(player_id) {
        println!("player coin {}", _p.coin.clone());
        let res: Option<Result<WaitForSingleInput, String>> = match from.get(position_index) {

            Some(&_c) => {
                match cardmeta[_c].cost as f64 <= _p.coin as f64 + (_p.ink as f64 / 3.0).floor() {
                    true => {
                        match cardmeta[_c].cost <= _p.coin {
                            true => {
                                _p.coin -= cardmeta[_c].cost;
                                _p.discard.push(from.remove(position_index));
                                resolve_cards::resolve_purchase(_c, _p, &cardmeta);
                                None
                            }
                            false => {
                                let j = "You do not have enough coin to buy this card, you may trade in 3 ink for one coin to buy this".to_owned();
                                Some(Ok((_c,
                                         j,
                                         vec![(GameState::Buy,
                                               "Trade in 3 ink for one coin to buy this?"
                                                   .to_owned(),
                                               Box::new(move |ref mut p, ref mut rmcards| {
                                                            p.discard.push(rmcards.remove(_c));
                                                        })),
                                              (GameState::Buy,
                                               "No, I want to another card".to_owned(),
                                               Box::new(|ref mut p, ref mut rmcards| {})),
                                              (GameState::DrawCard,
                                               "No, I want to end my buy phase.".to_owned(),
                                               Box::new(|ref mut p, ref mut rmcards| {}))])))

                            }
                        }
                    }
                    false => {
                        println!("You can't afford t");
                        let j = "You can't afford to buy this card. Do you want to buy another card?"
                            .to_owned();
                        Some(Ok((_c,
                                 j,
                                 vec![(GameState::Buy,
                                       "Yes".to_owned(),
                                       Box::new(|ref mut p, _| {})),
                                      (GameState::DrawCard,
                                       "No, I want to end my buy phase".to_owned(),
                                       Box::new(|ref mut p, _| {}))])))
                    }
                }
            }
            None => Some(Err("Cannot find the card selected".to_owned())),
        };

        if let Some(Ok(a)) = res {
            println!("pushed...");
            //  wait_tx.send(Some(a)).unwrap();
            wait_for_input[player_id].push(Some(a));
            wait_for_input[player_id].push(None);
        }
    }

}
pub fn buy_card_from_lockup(position_index: usize,
                            cardmeta: &[cards::ListCard<BoardStruct>; 180],
                            _board: &mut BoardStruct,
                            player_id: usize,
                            wait_for_input: &mut [WaitForInputType; 4]) {
    if let Some(_p) = _board.players.get_mut(player_id) {
        let mut card_index = 2;
        if let Some(&_c) = _p.lockup.get(position_index) {
            card_index = _c;
        } else {
            println!("lockup does not have this card");
        }
        let res: Option<Result<WaitForSingleInput, String>> = match cardmeta[card_index].cost as f64 <=
              _p.coin as f64 +
              (_p.ink as f64 / 3.0).floor() {
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
                        Some(Ok((card_index,
                                 j,
                                 vec![(GameState::DrawCard,
                                       "Trade in 3 ink for one coin to buy this.".to_owned(),
                                       Box::new(move |ref mut p, _| {
                            let coin_left = p.coin;
                            let remainder = cost - coin_left;
                            p.coin = 0;
                            p.ink -= remainder * 3;
                            p.discard.push(card_index);
                            p.lockup.remove(position_index);

                        })),
                                      (GameState::Buy,
                                       "No, I want to buy another card.".to_owned(),
                                       Box::new(|ref mut p, _| {})),
                                      (GameState::DrawCard,
                                       "No, I want to end buy phase.".to_owned(),
                                       Box::new(|ref mut p, _| {}))])))

                    }
                }
            }
            false => {
                let j = "You can't afford to buy this card. Do you want to buy another card?"
                    .to_owned();
                Some(Ok((card_index,
                         j,
                         vec![(GameState::Buy, "Yes".to_owned(), Box::new(|ref mut p, _| {})),
                              (GameState::DrawCard,
                               "No, I want to end my buy phase".to_owned(),
                               Box::new(|ref mut p, _| {}))])))
            }

        };
        if let Some(Ok(a)) = res {
            //   wait_tx.send(Some(a)).unwrap();
            wait_for_input[player_id].push(Some(a));
        }
    }

}
pub fn lockup_a_card(position_index: usize,
                     _board: &mut BoardStruct,
                     player_id: usize,
                     remainingcards: &mut Vec<usize>,
                     wait_for_input: &mut [WaitForInputType; 4],
                     type_is_reply: &mut bool) {
    if let (Some(ref mut _p), Some(ref mut _w)) =
        (_board.players.get_mut(player_id), wait_for_input.get_mut(player_id)) {
        _p.lockup.push(remainingcards[position_index]);
        _w.push(None);
        remainingcards.remove(position_index);
        *type_is_reply = false;
    }
}
pub fn trash_another_card(position_index: usize,
                          _board: &mut BoardStruct,
                          player_id: usize,
                          wait_for_input: &mut [WaitForInputType; 4],
                          type_is_reply: &mut bool) {
    if let Some(ref mut _p) = _board.players.get_mut(player_id) {
        _p.hand.remove(position_index);
        _p.coin += 1;
        *type_is_reply = false;
    }
}
pub fn putback_discard(countdown: usize,
                       responsible: usize,
                       _board: &mut BoardStruct,
                       player_id: usize,
                       remainingcards: &mut Vec<usize>,
                       wait_for_input: &mut [WaitForInputType; 4],
                       type_is_reply: &mut bool) {

    match countdown {
        2 => {
            if let Some(ref mut _p) = _board.players.get_mut(player_id) {
                if let Some(_c) = remainingcards.clone().get(7) {
                    if !_p.discard.contains(_c) {
                        _p.discard.push(remainingcards.remove(8));
                    } else {
                        _p.discard.push(remainingcards.remove(7));
                    }
                }
            }
        }
        1 => {
            if let Some(ref mut _p) = _board.players.get_mut(player_id) {
                if let (Some(_c1), Some(_c2)) =
                    (remainingcards.clone().get(7), remainingcards.clone().get(8)) {
                    let mut veczz = vec![];
                    if !_p.discard.contains(_c1) {
                        veczz.push(_c1);
                    }
                    if !_p.discard.contains(_c2) {
                        veczz.push(_c2);
                    }
                    _p.discard.push(remainingcards.remove(7 + veczz.len()));
                }
            }
        }
        0 => {
            if let Some(ref mut _p) = _board.players.get_mut(player_id) {
                if let (Some(_c1), Some(_c2), Some(_c3)) =
                    (remainingcards.clone().get(7),
                     remainingcards.clone().get(8),
                     remainingcards.clone().get(9)) {
                    let mut veczz = vec![];
                    if !_p.discard.contains(_c1) {
                        veczz.push(_c1);
                    }
                    if !_p.discard.contains(_c2) {
                        veczz.push(_c2);
                    }
                    if !_p.discard.contains(_c3) {
                        veczz.push(_c3);
                    }
                    _p.discard.push(remainingcards.remove(7 + veczz.len()));
                }
            }
        }
        _ => {}
    }
    if countdown == 0 {
        wait_for_input[player_id].push(None);
    } else if countdown - 1 >= 0 {
        let _g: WaitForSingleInput =
            (responsible,
             "Do you want to put back the card or add to your own discard pile?".to_owned(),
             vec![(GameState::PutBackDiscard(countdown - 1, responsible),
                   "Put back the card".to_owned(),
                   Box::new(move |ref mut p, ref mut rmcards| {})),
                  (GameState::PutBackDiscard(countdown - 1, responsible),
                   "Add to own discard pile.".to_owned(),
                   Box::new(move |ref mut p, ref mut rmcards| {}))]);
        wait_for_input[player_id].push(Some(_g));
        wait_for_input[player_id].push(None);
    }


    *type_is_reply = false;

}
