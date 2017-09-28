use server_lib::codec::{GameState, Player};
use server_lib::cards::*;
use server_lib::cards;
use game_logic::resolve_cards;

pub struct BoardStruct {
    pub players: Vec<Player>,
    pub offer_row: Vec<usize>,
}
impl BoardStruct {
    pub fn new(players: Vec<Player>, remaining_cards: &Vec<usize>) -> BoardStruct {
        BoardStruct {
            players: players,
            offer_row: (0..7).zip(remaining_cards.iter()).map(|(e, c)| c.clone()).collect(),
        }
    }
}
impl Board for BoardStruct {
    fn two_cent_per_adv(&mut self,
                        player_id: usize,
                        card_id: usize,
                        wait_for_input: &mut [WaitForInputType; 4]) {
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
                 j,
                 vec![(GameState::Spell,"lose a ink".to_owned(),
                       Box::new(|ref mut p, ref mut rmcards| { p.ink -= 1; })),
                      (GameState::Spell,"lose a ink remover".to_owned(),
                       Box::new(|ref mut p, ref mut rmcards| { p.remover -= 1; }))]);
            x.push(Some(_g));
            x.push(None);
        });

    }
    fn lockup_offer(&mut self,
                    player_id: usize,
                    card_id: usize,
                    wait_for_input: &mut [WaitForInputType; 4]) {
        if let Some(_w) = wait_for_input.get_mut(player_id) {

            let _g: WaitForSingleInput = (card_id,
                                          "Do you want to lock up any offer row card?".to_owned(),
                                          vec![(GameState::LockUp,
                                                "Yes".to_owned(),
                                                Box::new(|ref mut p, ref mut rmcards| {})),
                                               (GameState::Buy,
                                                "No".to_owned(),
                                                Box::new(|ref mut p, ref mut rmcards| {}))]);
            _w.push(Some(_g));
            _w.push(None);
        }

    }

    fn uncover_adjacent(&mut self,
                        player_id: usize,
                        card_id: usize,
                        wait_for_input: &mut [WaitForInputType; 4]) {
        let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
        if let (Some(_p), Some(_w)) =
            (self.players.get(player_id), wait_for_input.get_mut(player_id)) {
            let index = _p.arranged.iter().position(|x| x.0 == card_id);
            let _g: WaitForSingleInput = (card_id,
                                          "Do you want to uncover adjacent cards?".to_owned(),
                                          vec![(GameState::UncoverAdjacent(index, card_id),
                                                "Yes".to_owned(),
                                                Box::new(move |ref mut p, ref mut rmcards| {})),
                                               (GameState::Buy,
                                                "No".to_owned(),
                                                Box::new(|ref mut p, ref mut rmcards| {}))]);
            _w.push(Some(_g));
            _w.push(None);
        }
    }
    fn double_adjacent(&mut self,
                       player_id: usize,
                       card_id: usize,
                       wait_for_input: &mut [WaitForInputType; 4]) {
    }
    fn trash_other(&mut self,
                   player_id: usize,
                   card_id: usize,
                   wait_for_input: &mut [WaitForInputType; 4]) {
    }
    fn one_vp_per_wild(&mut self,
                       player_id: usize,
                       card_id: usize,
                       wait_for_input: &mut [WaitForInputType; 4]) {
    }
    fn keep_or_discard_three(&mut self,
                             player_id: usize,
                             card_id: usize,
                             wait_for_input: &mut [WaitForInputType; 4]) {
    }
}

pub fn get_valid_cards(_p: &mut Player) -> Vec<Option<usize>> {
    let mut valid_card = vec![];
    for it in _p.arranged.iter() {
        let &(_a, ref _w) = it;
        if let &Some(_) = _w {
            valid_card.push(None);
        } else {
            valid_card.push(Some(_a));
        }
    }
    valid_card
}
