use codec_lib::codec::{GameState, Player};
use codec_lib::cards::*;
use codec_lib::cards;
use game_logic::resolve_cards;

pub struct BoardStruct {
    pub players: Vec<Player>,
    pub offer_row: Vec<usize>,
}
impl BoardStruct {
    pub fn new(players: Vec<Player>, remaining_cards: &Vec<usize>) -> BoardStruct {
        BoardStruct {
            players: players,
            offer_row: (0..7).zip(remaining_cards.iter()).map(|(_, c)| c.clone()).collect(),
        }
    }
}
impl Board for BoardStruct {
    fn two_cent_per_adv(&mut self,
                        player_id: usize,
                        _card_id: usize,
                        _wait_for_input: &mut [WaitForInputType; 4]) {
        let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
        let mut adv_vec = vec![];
        if let Some(ref mut z) = self.players.get_mut(player_id) {
            let valid_cards = get_valid_cards(z);
            for it in valid_cards {
                if let Some(_c) = it {
                    match cardmeta[_c].genre {
                        Genre::ADVENTURE => {
                            adv_vec.push(_c);
                        }
                        _ => {}
                    }
                }
            }
            z.vp += adv_vec.len();
        }
    }
    fn minus_other_ink(&mut self,
                       player_id: usize,
                       card_id: usize,
                       wait_for_input: &mut [WaitForInputType; 4]) {
        let mut c = 0;
        let mut _o = vec![];
        for _p in self.players.iter() {
            if c != player_id {
                if (_p.ink > 0) | (_p.remover > 0) {
                    _o.push(c);
                }
            }
            c += 1;
        }
        wait_for_input.iter_mut().map(move| x|{
             let j = format!("Player {} has played a card to force other players to lose a ink or ink remover.",
                            player_id);
            let _g: WaitForSingleInput=
                (card_id,
                GameState::PreWaitForReply,
                 j,
                 vec![(GameState::Spell,"lose a ink".to_owned(),
                       Box::new(|ref mut p, ref mut _rmcards,ref mut _unknown| { p.ink -= 1; })),
                      (GameState::Spell,"lose a ink remover".to_owned(),
                       Box::new(|ref mut p, _,_| { p.remover -= 1; }))]);
            x.push(Some(_g));
        }).collect::<Vec<()>>();

    }
    fn lockup_offer(&mut self,
                    player_id: usize,
                    card_id: usize,
                    wait_for_input: &mut [WaitForInputType; 4]) {
        if let Some(_w) = wait_for_input.get_mut(player_id) {

            let _g: WaitForSingleInput =
                (card_id,
                 GameState::PreWaitForReply,
                 "Do you want to lock up any offer row card?".to_owned(),
                 vec![(GameState::LockUp,
                       "Yes".to_owned(),
                       Box::new(|ref mut _p, ref mut _rmcards, _| {})),
                      (GameState::PreBuy, "No".to_owned(), Box::new(|_, _, _| {}))]);
            _w.push(Some(_g));
        }

    }

    fn uncover_adjacent(&mut self,
                        player_id: usize,
                        card_id: usize,
                        wait_for_input: &mut [WaitForInputType; 4]) {
        if let (Some(ref mut _p), Some(ref mut _w)) =
            (self.players.get_mut(player_id), wait_for_input.get_mut(player_id)) {
            let index = _p.arranged.iter().position(|x| x.0 == card_id);
            if !there_is_wild_beside(_p, index) {
                let _g: WaitForSingleInput =
                    (card_id,
                     GameState::PreWaitForReply,
                     "There are no adjacent wild cards that can be flipped over.".to_owned(),
                     vec![(GameState::PreBuy, "Continue".to_owned(), Box::new(move |_, _, _| {}))]);
                _w.push(Some(_g));
            } else {
                let _g: WaitForSingleInput =
                    (card_id,
                     GameState::PreWaitForReply,
                     "There are a few wild cards adjacent to this card, can be opened".to_owned(),
                     vec![(GameState::ResolveAgain(index, card_id),
                           "Continue".to_owned(),
                           Box::new(move |_, _, _| {}))]);
                _w.push(Some(_g));

            }

        }
    }
    fn double_adjacent(&mut self,
                       player_id: usize,
                       card_id: usize,
                       wait_for_input: &mut [WaitForInputType; 4]) {

        if let (Some(ref mut _p), Some(ref mut _w)) =
            (self.players.get_mut(player_id), wait_for_input.get_mut(player_id)) {
            let index = _p.arranged.iter().position(|x| x.0 == card_id);
            let (there_is, cards_to_double) = there_is_valid_beside(_p, index);
            if !there_is {
                let _g: WaitForSingleInput =
                    (card_id,
                     GameState::PreWaitForReply,
                     "There are no adjacent valid cards that you double their benefits."
                         .to_owned(),
                     vec![(GameState::PreBuy, "Continue".to_owned(), Box::new(move |_, _, _| {}))]);
                _w.push(Some(_g));
            } else {
                let _g: WaitForSingleInput =
                    (card_id,
                     GameState::PreWaitForReply,
                     "There are a few valid cards adjacent to this card, can be doubled."
                         .to_owned(),
                     vec![(GameState::PreBuy,
                           "Continue".to_owned(),
                           Box::new(move |ref mut p, ref mut rmcards, _| {
                        let cardmeta: [cards::ListCard<BoardStruct>; 180] =
                            cards::populate::<BoardStruct>();
                        for card in cards_to_double.clone() {
                            if let Some(pos) = p.skip_cards.iter().position(|x| *x == card) {
                                p.skip_cards.remove(pos);
                            }
                        }

                        let mut tempboard = BoardStruct::new(vec![p.clone()], &rmcards);
                        let mut wait_for_input: [WaitForInputType; 4] = [vec![], vec![], vec![],
                                                                         vec![]];
                        resolve_cards::resolve_cards(&mut tempboard,
                                                     player_id,
                                                     // 0,
                                                     &cardmeta,
                                                     &mut wait_for_input);
                        if let Some(_p) = tempboard.players.get(player_id) {
                            **p = _p.clone();
                        }
                    }))]);
                _w.push(Some(_g));

            }

        }
    }
    fn trash_other(&mut self,
                   player_id: usize,
                   card_id: usize,
                   wait_for_input: &mut [WaitForInputType; 4]) {
        if let (Some(ref mut _p), Some(ref mut _w)) =
            (self.players.get_mut(player_id), wait_for_input.get_mut(player_id)) {
            //println!("trash by {:?}", card_id.clone());
            let _g: WaitForSingleInput =
                (card_id.clone(),
                 GameState::PreWaitForReply,
                 "Do you want to trash another card for one cent?".to_owned(),
                 vec![(GameState::PreTrashOther(card_id),
                       "Yes".to_owned(),
                       Box::new(move |_, _, _| {})),
                      (GameState::PreBuy, "No".to_owned(), Box::new(move |_, _, _| {}))]);
            _w.push(Some(_g));
        }
    }
    fn one_vp_per_wild(&mut self,
                       player_id: usize,
                       card_id: usize,
                       wait_for_input: &mut [WaitForInputType; 4]) {
        if let (Some(ref mut _p), Some(ref mut _w)) =
            (self.players.get_mut(player_id), wait_for_input.get_mut(player_id)) {
            let num_wild = _p.arranged
                .iter()
                .filter(|&&(ref _cx, _, ref _w, ref _timeless)| if let &Some(_) = _w {
                            true
                        } else {
                            false
                        })
                .collect::<Vec<&(usize, bool, Option<String>, bool)>>()
                .len();
            let _num_wild = num_wild.clone();
            let j = format!("You gain {} vp from this card.", num_wild);
            let _g: WaitForSingleInput =
                (card_id,
                 GameState::PreWaitForReply,
                 j,
                 vec![(GameState::PreBuy,
                       "Continue".to_owned(),
                       Box::new(move |ref mut p, ref mut rmcards, _| { p.vp += _num_wild; }))]);
            _w.push(Some(_g));
        }
    }
    fn putback_or_discard_three(&mut self,
                                player_id: usize,
                                card_id: usize,
                                wait_for_input: &mut [WaitForInputType; 4]) {
        if let (Some(ref mut _p), Some(ref mut _w)) =
            (self.players.get_mut(player_id), wait_for_input.get_mut(player_id)) {
            let _g: WaitForSingleInput =
                (card_id,
                GameState::PrePutBackDiscard(2,card_id),
                 "You may draw three cards from the top of deck and choose to keep or discard each of them.".to_owned(),
                 vec![(GameState::PrePutBackDiscard(2,card_id),
                       "Continue".to_owned(),
                       Box::new(move |_,_,_| {
                        }))]);
            println!("did push putback_or_discard_three");
            _w.push(Some(_g));

        }
    }
}

pub fn get_valid_cards(_p: &mut Player) -> Vec<Option<usize>> {
    let mut valid_card = vec![];
    for it in _p.arranged.iter() {
        let &(_a, _, ref _w, ref _timeless) = it;
        if let &Some(_) = _w {
            valid_card.push(None);
        } else {
            valid_card.push(Some(_a));
        }
    }
    valid_card
}
fn there_is_wild_beside(_p: &mut Player, position_card: Option<usize>) -> bool {
    let mut there_is_wild_beside = false;
    if let Some(_position_card) = position_card {
        if _position_card == 0 {
            if let Some(&mut (_covered_card, _, ref mut _wild, ref _timeless)) =
                _p.arranged.get_mut(1) {
                //remove wild
                if let &mut Some(_) = _wild {
                    *_wild = None;
                    there_is_wild_beside = true;
                }
            }
        } else if _position_card == _p.arranged.len() - 1 {
            if let Some(&mut (_covered_card, _, ref mut _wild, ref _timeless)) =
                _p.arranged.get_mut(_position_card - 1) {
                //remove wild
                if let &mut Some(_) = _wild {
                    *_wild = None;
                    there_is_wild_beside = true;
                }
            }
        } else {
            if let Some(&mut (_covered_card, _, ref mut _wild, ref _timeless)) =
                _p.arranged.get_mut(_position_card - 1) {
                //remove wild
                if let &mut Some(_) = _wild {
                    *_wild = None;
                    there_is_wild_beside = true;
                }
            }
            if let Some(&mut (_covered_card, _, ref mut _wild, ref _timeless)) =
                _p.arranged.get_mut(_position_card + 1) {
                //remove wild
                if let &mut Some(_) = _wild {
                    *_wild = None;
                    there_is_wild_beside = true;
                }
            }
        }
    }
    there_is_wild_beside
}
fn there_is_valid_beside(_p: &mut Player, position_card: Option<usize>) -> (bool, Vec<usize>) {
    let mut there_is_valid_beside = false;
    let mut cards_to_double = vec![];
    if let Some(_position_card) = position_card {
        if _position_card == 0 {
            if let Some(&mut (card_index, _, ref mut _wild, ref _timeless)) =
                _p.arranged.get_mut(1) {
                if let &mut None = _wild {
                    cards_to_double.push(card_index);
                    there_is_valid_beside = true;
                }
            }
        } else if _position_card == _p.arranged.len() - 1 {
            if let Some(&mut (card_index, _, ref mut _wild, ref _timeless)) =
                _p.arranged.get_mut(_position_card - 1) {
                //remove wild
                if let &mut None = _wild {
                    cards_to_double.push(card_index);
                    there_is_valid_beside = true;
                }
            }
        } else {
            if let Some(&mut (card_index, _, ref mut _wild, ref _timeless)) =
                _p.arranged.get_mut(_position_card - 1) {
                //remove wild
                if let &mut None = _wild {
                    cards_to_double.push(card_index);
                    there_is_valid_beside = true;
                }
            }
            if let Some(&mut (card_index, _, ref mut _wild, ref _timeless)) =
                _p.arranged.get_mut(_position_card + 1) {
                //remove wild
                if let &mut None = _wild {
                    cards_to_double.push(card_index);
                    there_is_valid_beside = true;
                }
            }
        }

    }
    (there_is_valid_beside, cards_to_double)
}
fn remove_skip_card(_p: &mut Player, card_index: usize) {
    println!("tttremove_caaaaaaaaaaaaaaa{:?}", _p.skip_cards.clone());
    if let Some(_index) = _p.skip_cards.iter().position(|x| *x == card_index) {
        println!("remove_caaaaaaaaaaaaaaa{}", _index);
        _p.skip_cards.remove(_index);
    }
}
