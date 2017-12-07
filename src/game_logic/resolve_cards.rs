use codec_lib::codec::*;
use codec_lib::cards;
use codec_lib::cards::{GIVEABLE, WaitForInputType};
use game_logic::board::BoardStruct;
#[derive(Clone)]
pub enum CardType {
    Inked,
    Normal(usize),
    OthersTimeless(usize),
}
pub fn resolve_cards(mut _board: &mut BoardStruct,
                     player_id: usize,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     wait_for_input: &mut [WaitForInputType; 4]) {
    
    let mut valid_card = vec![];
    let mut own_timeless_class_card = vec![];
    let mut skip_cards = vec![];
    if let Some(ref _p) = _board.players.get(player_id) {
        valid_card = _p.arranged
            .iter()
            .map(|ref x| if x.3 {
                     if _p.timeless_classic.contains(&x.0) {
                         //remove from timeless_class
                         own_timeless_class_card.push(x.0);
                         CardType::Normal(x.0)
                     } else {
                         CardType::OthersTimeless(x.0)
                     }
                 } else {
                     if let None = x.2 {
                         CardType::Normal(x.0)
                     } else {
                         CardType::Inked
                     }
                 })
            .collect::<Vec<CardType>>();
        skip_cards = _p.skip_cards.clone();
    }
    if let Some(ref mut _p) = _board.players.get_mut(player_id) {
        for _time in own_timeless_class_card {
            if let Some(_card_position) = _p.timeless_classic.iter().position(|&x| x == _time) {
                _p.timeless_classic.remove(_card_position);
            }
        }
    }
    println!("wad are the cards skipped{:?}", skip_cards.clone());

    let mut adv_vec = vec![];
    let mut hor_vec = vec![];
    let mut mys_vec = vec![];
    let mut rom_vec = vec![];
    for t in &valid_card {
        match t {
            &CardType::Normal(_c) => {
                track_genre(_c.clone(),
                            &cardmeta,
                            &mut adv_vec,
                            &mut hor_vec,
                            &mut mys_vec,
                            &mut rom_vec);
                //skip_cards delay resolve, check if card is
                if let None = skip_cards.iter().position(|&x| x == _c) {
                    resolve_giveable(_c.clone(),
                                     &cardmeta,
                                     player_id,
                                     &mut _board,
                                     wait_for_input);

                }
            }
            &CardType::Inked => {}
            &CardType::OthersTimeless(_c) => {
                for ref mut _p in _board.players.iter_mut() {
                    if let Some(_card_position) =
                        _p.timeless_classic.iter().position(|&x| x == _c) {
                        _p.timeless_classic.remove(_card_position);
                    }
                }
            }
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
                           valid_card.clone());
    for t in &valid_card {
        if let &CardType::Normal(_c) = t {
            if let None = skip_cards.iter().position(|&x| x == _c) {
                skip_cards.push(_c);
            }
        }
    }
    if let Some(ref mut _p) = _board.players.get_mut(player_id) {
        _p.skip_cards = skip_cards;
    }
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
                        board: &mut BoardStruct,
                        wait_for_input: &mut [WaitForInputType; 4]) {
    if let Some(ref mut z) = board.players.get_mut(player_id) {
        giveable_match(z,
                       player_id,
                       &cardmeta[card_index].giveables,
                       card_index,
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
                              board: &mut BoardStruct,
                              wait_for_input: &mut [WaitForInputType; 4],
                              cardmeta: &[cards::ListCard<BoardStruct>; 180],
                              genre_vec: Vec<&Vec<usize>>) {
    let mut init_skip_cards = vec![];
    if let Some(ref mut z) = board.players.get_mut(player_id) {
        init_skip_cards = z.skip_cards.clone();
        for _o in genre_vec.clone() {
            if _o.len() >= 2 {
                for &_c in _o {
                    if let None = z.skip_cards.iter().position(|&x| x == _c) {
                        giveable_match(z,
                                       player_id,
                                       &cardmeta[_c].genre_giveables,
                                       _c,
                                       wait_for_input);
                        println!("genre card_index{}, player.vp:{}, player.coin:{}",
                                 _c,
                                 z.vp.clone(),
                                 z.coin.clone());

                    }
                }
            }

        }

    }

    for _o in genre_vec {
        if _o.len() >= 2 {
            for &_c in _o {
                if let None = init_skip_cards.iter().position(|&x| x == _c) {
                    if let Some(ref _closure) = cardmeta[_c].genrefn {
                        (*_closure)(board, player_id, _c, wait_for_input);
                    }
                }
            }

        }

    }
}
pub fn resolve_trash_giveable(player_id: usize,
                              board: &mut BoardStruct,
                              wait_for_input: &mut [WaitForInputType; 4],
                              cardmeta: &[cards::ListCard<BoardStruct>; 180],
                              valid_card: Vec<CardType>) {
    if let (Some(ref mut z), ref mut _wait_vec) =
        (board.players.get_mut(player_id), &mut wait_for_input[player_id]) {
        for _oc in valid_card {
            if let CardType::Normal(_c) = _oc {
                let y = z.skip_cards.iter().position(|&x| x == _c);
                println!("yyyy{:?}", y);
                if let None = z.skip_cards.iter().position(|&x| x == _c) {
                    let header = "Do you want to trash this card for the benefit?".to_owned();
                    let vec_option: Option<Vec<(GameState,
                                                String,
                                                Box<Fn(&mut Player,
                                                       &mut Vec<usize>,
                                                       &mut Vec<usize>)>)>> =
                        match cardmeta[_c.clone()].trash {
                            GIVEABLE::VP(_x) => {
                                Some(vec![(GameState::Buy,
                                           "Yes".to_owned(),
                                           Box::new(move |ref mut _p, _, _| {
                                    _p.vp += _x;
                                    let index = _p.hand
                                        .iter()
                                        .position(|&x| x == _c)
                                        .unwrap();
                                    _p.hand.remove(index);
                                })),
                                          (GameState::Buy,
                                           "No".to_owned(),
                                           Box::new(|ref mut _p, _, _| {}))])
                            }
                            GIVEABLE::COIN(_x) => {
                                Some(vec![(GameState::Buy,
                                           "Yes".to_owned(),
                                           Box::new(move |ref mut _p, _, _| {
                                    _p.coin += _x;
                                    let index = _p.hand
                                        .iter()
                                        .position(|&x| x == _c)
                                        .unwrap();
                                    _p.hand.remove(index);
                                })),
                                          (GameState::Buy,
                                           "No".to_owned(),
                                           Box::new(|ref mut _p, _, _| {}))])
                            }
                            _ => None,
                        };
                    if let Some(_opts) = vec_option {
                        _wait_vec.push(Some((_c, header, _opts)));
                        _wait_vec.push(None);
                    }

                }
            }
        }
    }
}
pub fn giveable_match(z: &mut Player,
                      player_id: usize,
                      giveables: &cards::GIVEABLE,
                      card_index: usize,
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
            wait_for_input[player_id].push(Some((card_index,
                                                 choose_bet,
                                                 vec![(GameState::DrawCard,
                                                       "Ink".to_owned(),
                                                       Box::new(|ref mut _p, _, _| {
                                                                    _p.ink += 1;
                                                                })),
                                                      (GameState::DrawCard,
                                                       "Ink Remover".to_owned(),
                                                       Box::new(|ref mut _p, _, _| {
                                                                    _p.remover += 1;
                                                                }))])));
            wait_for_input[player_id].push(None);
        }
        &cards::GIVEABLE::VPINK(_x) => {
            z.vp += _x;
            wait_for_input[player_id].push(Some((card_index,
                                                 choose_bet,
                                                 vec![(GameState::DrawCard,
                                                       "Ink".to_owned(),
                                                       Box::new(|ref mut _p, _, _| {
                                                                    _p.ink += 1;
                                                                })),
                                                      (GameState::DrawCard,
                                                       "Ink Remover".to_owned(),
                                                       Box::new(|ref mut _p, _, _| {
                                                                    _p.remover += 1;
                                                                }))])));
            wait_for_input[player_id].push(None);
        }
        &cards::GIVEABLE::NONE => {}
        &cards::GIVEABLE::INK => {
            wait_for_input[player_id].push(Some((card_index,
                                                 choose_bet,
                                                 vec![(GameState::DrawCard,
                                                       "Ink".to_owned(),
                                                       Box::new(|ref mut _p, _, _| {
                                                                    _p.ink += 1;
                                                                })),
                                                      (GameState::DrawCard,
                                                       "Ink Remover".to_owned(),
                                                       Box::new(|ref mut _p, _, _| {
                                                                    _p.remover += 1;
                                                                }))])));
            wait_for_input[player_id].push(None);
        }
        &cards::GIVEABLE::VPORCOIN(_x) => {
            let j1 = format!("{} vps", _x);
            let j2 = format!("{} coins", _x);
            let header = "You have the options to choose between vps and coins. Which one do you want?"
                .to_owned();
            wait_for_input[player_id].push(Some((card_index,
                                                 choose_bet,
                                                 vec![(GameState::Buy,
                                                       j1,
                                                       Box::new(move |ref mut _p, _, _| {
                                                                    _p.vp += _x;
                                                                })),
                                                      (GameState::Buy,
                                                       j2,
                                                       Box::new(move |ref mut _p, _, _| {
                                                                    _p.coin += _x;
                                                                }))])));
            wait_for_input[player_id].push(None);
        }
        &cards::GIVEABLE::VPORCOININK(_x) => {
            let j1 = format!("{} VP and 1 ink", _x);
            let j2 = format!("{} Coin and 1 ink", _x);
            let j3 = format!("{} VP and 1 ink remover", _x);
            let j4 = format!("{} Coin and 1 ink remover", _x);
            wait_for_input[player_id].push(Some((card_index,
                                                 choose_bet,
                                                 vec![(GameState::Buy,
                                                       j1,
                                                       Box::new(move |ref mut _p, _, _| {
                                                                    _p.vp += _x;
                                                                    _p.ink += 1;
                                                                })),
                                                      (GameState::Buy,
                                                       j2,
                                                       Box::new(move |ref mut _p, _, _| {
                                                                    _p.coin += _x;
                                                                    _p.ink += 1;
                                                                })),
                                                      (GameState::Buy,
                                                       j3,
                                                       Box::new(move |ref mut _p, _, _| {
                                                                    _p.vp += _x;
                                                                    _p.remover += 1;
                                                                })),
                                                      (GameState::Buy,
                                                       j4,
                                                       Box::new(move |ref mut _p, _, _| {
                                                                    _p.coin += _x;
                                                                    _p.remover += 1;
                                                                }))])));
            wait_for_input[player_id].push(None);
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
