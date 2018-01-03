use game_logic;
use codec_lib::cards;
use codec_lib::codec::*;
use game_logic::board::BoardStruct;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use rand;
pub struct TheStartingDraftStruct {}
impl game_logic::game_engine::TheDraft for TheStartingDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        let mut collected_letter = vec![];
        let mut collected_id = vec![];
        let mut rand_id = vec![];
        let mut two_cards_id = vec![];
        let mut remaining_deck = vec![];
        for &cards::ListCard { letter, ref genre, id, .. } in cardmeta.iter() {
            if !owned_deck.contains(&id) {
                //if it is not owned
                remaining_deck.push(id);
            }
        }
        for r_id in remaining_deck {
            match (&cardmeta[r_id].genre, &cardmeta[r_id].giveables) {
                (&cards::Genre::NONE, &cards::GIVEABLE::COIN(_)) => {
                    let letc = cardmeta[r_id].letter.to_owned();
                    if !collected_letter.contains(&letc) {
                        //has not collected letter
                        collected_letter.push(cardmeta[r_id].letter.to_owned());
                        collected_id.push(r_id);
                        owned_deck.push(r_id);
                    }
                }
                (&cards::Genre::NONE, &cards::GIVEABLE::VP(_)) => {
                    rand_id.push(r_id);
                }
                _ => {}
            }
        }
        let mut rng = rand::thread_rng();
        for _ in 0..2 {
            let between = Range::new(0, rand_id.len() - 1);
            let c = between.ind_sample(&mut rng) as usize;
            if let Some(&idz) = rand_id.get(c) {
                two_cards_id.push(idz);
                rand_id.remove(c);
                owned_deck.push(idz);
            }
        }
        collected_id.extend(two_cards_id.clone());
        rng.shuffle(&mut collected_id);
        _p.draft = collected_id;
    }
    fn deck_starting(&self,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        let mut remaining_deck = vec![];
        for &cards::ListCard { ref genre, id, .. } in cardmeta.iter() {
            if !owned_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut remaining_deck);
        remaining_deck
    }
    fn ticks(&self) -> Option<u16> {
        Some(0)
    }
    fn show_draft(&self) -> (bool,  Option<&[usize]>) {
        (true, None)
    }
    fn push_notification(&self) -> bool {
        true
    }
}
