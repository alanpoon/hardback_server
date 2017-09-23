use server_lib::codec::{GameState, Player};
use server_lib::cards::*;
use server_lib::cards;
use std::sync::mpsc;

pub struct BoardStruct {
    pub players: Vec<Player>,
    pub offer_row: Vec<usize>,
    pub gamestates: Vec<GameState>,
    pub tx: mpsc::Sender<Option<(usize,
                                 String,
                                 Vec<(String, Box<Fn(&mut Player, &mut Vec<usize>)>)>)>>,
}
impl Board for BoardStruct {
    fn two_cent_per_adv(&mut self, player_id: usize, card_id: usize) {
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
    fn minus_other_ink(&mut self, player_id: usize, card_id: usize) {
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
        for _j in _o {
            let j = format!("Player {} has played a card to force other players to lose a ink or ink remover.",
                            player_id);
            let _g: (usize, String, Vec<(String, Box<Fn(&mut Player, &mut Vec<usize>)>)>) =
                (player_id,
                 j,
                 vec![("lose a ink".to_owned(),
                       Box::new(|ref mut p, ref mut rmcards| { p.ink -= 1; })),
                      ("lose a ink remover".to_owned(),
                       Box::new(|ref mut p, ref mut rmcards| { p.remover -= 1; }))]);
            self.tx
                .clone()
                .send(Some(_g))
                .unwrap();
        }
    }
    fn lockup_offer(&mut self, player_id: usize, card_id: usize) {}
    fn uncover_adjacent(&mut self, player_id: usize, card_id: usize) {}
    fn double_adjacent(&mut self, player_id: usize, card_id: usize) {}
    fn trash_other(&mut self, player_id: usize, card_id: usize) {}
    fn one_vp_per_wild(&mut self, player_id: usize, card_id: usize) {}
    fn keep_or_discard_three(&mut self, player_id: usize, card_id: usize) {}
}
impl BoardStruct {
    pub fn new(players: Vec<Player>,
               gamestates: Vec<GameState>,
               offer_row: Vec<usize>,
               tx: mpsc::Sender<Option<(usize,
                                        String,
                                        Vec<(String, Box<Fn(&mut Player, &mut Vec<usize>)>)>)>>)
               -> BoardStruct {
        BoardStruct {
            players: players,
            offer_row: offer_row,
            tx: tx,
            gamestates: gamestates,
        }
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