use server_lib::codec::Player;
use server_lib::cards::*;
use server_lib::cards;
use game_logic::game_engine::GameState;
use std::sync::mpsc;

pub struct BoardStruct {
    pub players: Vec<Player>,
    pub gamestates: Vec<GameState>,
    pub tx: mpsc::Sender<Option<(usize, String, Vec<(String, Box<Fn(&mut Player)>)>)>>,
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
            if let Some(ref mut _g) = self.gamestates.get_mut(_j) {
                let j = format!("Player {} has played a card to force other players to lose a ink or ink remover.",
                                player_id);
                let _g: (usize, String, Vec<(String, Box<Fn(&mut Player)>)>) =
                    (player_id,
                     j,
                     vec![("lose a ink".to_owned(), Box::new(|ref mut p| { p.ink -= 1; })),
                          ("lose a ink remover".to_owned(),
                           Box::new(|ref mut p| { p.remover -= 1; }))]);
                self.tx
                    .clone()
                    .send(Some(_g))
                    .unwrap();
            }
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
               tx: mpsc::Sender<Option<(usize, String, Vec<(String, Box<Fn(&mut Player)>)>)>>)
               -> BoardStruct {
        BoardStruct {
            players: players,
            tx: tx,
            gamestates: gamestates,
        }
    }
}
pub fn get_valid_cards(_p: &mut Player) -> Vec<Option<usize>> {
    let mut valid_card = vec![];
    for it in _p.arranged.iter().zip(_p.wild.iter()) {
        let (&_a, _w) = it;
        if let &Some(_) = _w {
            valid_card.push(None);
        } else {
            valid_card.push(Some(_a));
        }
    }
    valid_card
}
