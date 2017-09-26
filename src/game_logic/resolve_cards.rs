use server_lib::codec::*;
use server_lib::cards;
use server_lib::cards::{GIVEABLE, WaitForInputType};
use game_logic::board::BoardStruct;

pub fn resolve_cards(mut _board: &mut BoardStruct,
                     player_id: usize,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     wait_for_input: &mut [WaitForInputType; 4]) {
    //broadcast those benefits that don't need to wait for user reply
    if let Some(ref mut it) = wait_for_input.get_mut(player_id) {
        it.push(None);
    }
    let mut valid_card = vec![];
    if let Some(_p) = _board.players.get(player_id) {
        valid_card = _p.arranged
            .iter()
            .map(|x| if let None = x.1 { Some(x.0) } else { None })
            .collect::<Vec<Option<usize>>>();
    }

    let mut adv_vec = vec![];
    let mut hor_vec = vec![];
    let mut mys_vec = vec![];
    let mut rom_vec = vec![];
    for t in &valid_card {
        if let &Some(_c) = t {
            track_genre(_c.clone(),
                        &cardmeta,
                        &mut adv_vec,
                        &mut hor_vec,
                        &mut mys_vec,
                        &mut rom_vec);
            resolve_giveable(_c.clone(),
                             &cardmeta,
                             player_id,
                             &mut _board,
                             wait_for_input);
        }
    }
    resolve_genre_giveable(player_id,
                           &mut _board,
                           wait_for_input,
                           &cardmeta,
                           vec![&adv_vec, &hor_vec, &mys_vec, &rom_vec]);
    resolve_trash_giveable(player_id,
                           &mut _board,
                           wait_for_input,
                           &cardmeta,
                           &valid_card);
}
pub fn track_genre(card_index: usize,
                   cardmeta: &[cards::ListCard<BoardStruct>; 180],
                   adv: &mut Vec<usize>,
                   hor: &mut Vec<usize>,
                   mys: &mut Vec<usize>,
                   rom: &mut Vec<usize>) {
    match cardmeta[card_index].genre {
        cards::Genre::ADVENTURE => {
            adv.push(card_index);
        }
        cards::Genre::HORROR => {
            hor.push(card_index);
        }
        cards::Genre::MYSTERY => {
            mys.push(card_index);
        }
        cards::Genre::ROMANCE => {
            rom.push(card_index);
        }
        _ => {}
    }
}
pub fn resolve_giveable(card_index: usize,
                        cardmeta: &[cards::ListCard<BoardStruct>; 180],
                        player_id: usize,
                        mut board: &mut BoardStruct,
                        wait_for_input: &mut [WaitForInputType; 4]) {
    if let Some(ref mut z) = board.players.get_mut(player_id) {
        giveable_match(z,
                       player_id,
                       &cardmeta[card_index].giveables,
                       wait_for_input);
        println!("card_index:{:?}, player.vp:{}, player.coin:{}",
                 card_index,
                 z.vp.clone(),
                 z.coin.clone());
    }
    //resolve closure
    if let Some(ref _closure) = cardmeta[card_index].giveablefn {
        (*_closure)(board, player_id, card_index, wait_for_input);
    }
}

pub fn resolve_genre_giveable(player_id: usize,
                              mut board: &mut BoardStruct,
                              wait_for_input: &mut [WaitForInputType; 4],
                              cardmeta: &[cards::ListCard<BoardStruct>; 180],
                              genre_vec: Vec<&Vec<usize>>) {
    if let Some(ref mut z) = board.players.get_mut(player_id) {
        for _o in genre_vec.clone() {
            if _o.len() >= 2 {
                for &_c in _o {
                    giveable_match(z, player_id, &cardmeta[_c].genre_giveables, wait_for_input);
                    println!("genre card_index{}, player.vp:{}, player.coin:{}",
                             _c,
                             z.vp.clone(),
                             z.coin.clone());
                }
            }

        }

    }
    for _o in genre_vec {
        if _o.len() >= 2 {
            for &_c in _o {
                if let Some(ref _closure) = cardmeta[_c].giveablefn {
                    (*_closure)(board, player_id, _c, wait_for_input);
                }
            }

        }
    }
}
pub fn resolve_trash_giveable(player_id: usize,
                              mut board: &mut BoardStruct,
                              wait_for_input: &mut [WaitForInputType; 4],
                              cardmeta: &[cards::ListCard<BoardStruct>; 180],
                              valid_card: &Vec<Option<usize>>) {
    if let (Some(ref mut z), ref mut _wait_vec) =
        (board.players.get_mut(player_id), &mut wait_for_input[player_id]) {
        for &_oc in valid_card {
            if let Some(_c) = _oc {
                let header = "Do you want to trash this card for the benefit?".to_owned();
                let vec_option: Option<Vec<(String, Box<Fn(&mut Player, &mut Vec<usize>)>)>> =
                    match cardmeta[_c].trash {
                        GIVEABLE::VP(_x) => {
                            Some(vec![("yes".to_owned(),
                                       Box::new(move |ref mut p, _| {
                                p.vp += _x;
                                let index = p.hand
                                    .iter()
                                    .position(|x| *x == _c)
                                    .unwrap();
                                p.hand.remove(index);
                            })),
                                      ("no".to_owned(), Box::new(|ref mut p, _| {}))])
                        }
                        GIVEABLE::COIN(_x) => {
                            Some(vec![("yes".to_owned(),
                                       Box::new(move |ref mut p, _| {
                                p.coin += _x;
                                let index = p.hand
                                    .iter()
                                    .position(|x| *x == _c)
                                    .unwrap();
                                p.hand.remove(index);
                            })),
                                      ("no".to_owned(), Box::new(|ref mut p, _| {}))])
                        }
                        _ => None,
                    };
                if let Some(_opts) = vec_option {
                    _wait_vec.push(Some((GameState::Buy, header, _opts)));
                    _wait_vec.push(None);
                }
            }
        }
    }
}
pub fn giveable_match(z: &mut Player,
                      player_id: usize,
                      giveables: &cards::GIVEABLE,
                      wait_for_input: &mut [WaitForInputType; 4]) {
    let choose_bet = "Choose between".to_owned();
    match giveables {
        &cards::GIVEABLE::VP(_x) => {
            z.vp += _x;
        }
        &cards::GIVEABLE::COIN(_x) => {
            z.coin += _x;
        }
        &cards::GIVEABLE::VPCOIN(_x1, _x2) => {
            z.vp += _x1;
            z.coin += _x2;
        }
        &cards::GIVEABLE::COININK(_x) => {
            z.coin += _x;
            wait_for_input[player_id].push(Some((GameState::DrawCard,
                                                 choose_bet,
                                                 vec![("Ink".to_owned(),
                                                       Box::new(|ref mut p, _| {
                                                                    p.ink += 1;
                                                                })),
                                                      ("Ink Remover".to_owned(),
                                                       Box::new(|ref mut p, _| {
                                                                    p.remover += 1;
                                                                }))])));
            /*     wait_tx.send(Some((player_id,
                               choose_bet,
                               vec![("Ink".to_owned(),
                                     Box::new(|ref mut p, _| { p.ink += 1; })),
                                    ("Ink Remover".to_owned(),
                                     Box::new(|ref mut p, _| { p.remover += 1; }))])))
                .unwrap();
                */
        }
        &cards::GIVEABLE::VPINK(_x) => {
            z.vp += _x;
            /*  wait_tx.send(Some((player_id,
                               choose_bet,
                               vec![("1 Ink".to_owned(),
                                     Box::new(|ref mut p, _| { p.ink += 1; })),
                                    ("1 Ink Remover".to_owned(),
                                     Box::new(|ref mut p, _| { p.remover += 1; }))])))
                .unwrap();
                */
        }
        &cards::GIVEABLE::NONE => {}
        &cards::GIVEABLE::INK => {
            /*     wait_tx.send(Some((player_id,
                               choose_bet,
                               vec![("1 Ink".to_owned(),
                                     Box::new(|ref mut p, _| { p.ink += 1; })),
                                    ("1 Ink Remover".to_owned(),
                                     Box::new(|ref mut p, _| { p.remover += 1; }))])))
                .unwrap();
                */
        }
        &cards::GIVEABLE::VPORCOIN(_x) => {
            let j1 = format!("{} VP", _x);
            let j2 = format!("{} Coin", _x);
            let _xc = _x.clone();
            let _xcc = _xc.clone();
            /*  wait_tx.send(Some((player_id,
                               choose_bet,
                               vec![(j1, Box::new(move |ref mut p, _| { p.vp += _x; })),
                                    (j2, Box::new(move |ref mut p, _| { p.coin += _x; }))])))
                .unwrap();
            */
        }
        &cards::GIVEABLE::VPORCOININK(_x) => {
            let j1 = format!("{} VP and 1 ink", _x);
            let j2 = format!("{} Coin and 1 ink", _x);
            let j3 = format!("{} VP and 1 ink remover", _x);
            let j4 = format!("{} Coin and 1 ink remover", _x);
            /*     wait_tx.send(Some((player_id,
                               choose_bet,
                               vec![(j1,
                                     Box::new(move |ref mut p, _| {
                                                  p.vp += _x;
                                                  p.ink += 1;
                                              })),
                                    (j2,
                                     Box::new(move |ref mut p, _| {
                                                  p.coin += _x;
                                                  p.ink += 1;
                                              })),
                                    (j3,
                                     Box::new(move |ref mut p, _| {
                                                  p.vp += _x;
                                                  p.remover += 1;
                                              })),
                                    (j4,
                                     Box::new(move |ref mut p, _| {
                                                  p.coin += _x;
                                                  p.remover += 1;
                                              }))])))
                .unwrap();
                */
        }
    }
}
pub fn resolve_purchase(card_index: usize,
                        _p: &mut Player,
                        cardmeta: &[cards::ListCard<BoardStruct>; 180]) {
    match cardmeta[card_index].purchase_giveables {
        cards::GIVEABLE::COIN(_x) => {
            _p.coin += _x;
        }
        _ => {}
    }

}
