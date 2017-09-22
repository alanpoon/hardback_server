use server_lib::codec::*;
use server_lib::cards::{self, Board};
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use rand;
/* let mut temp_players = vec![];
            for mut it in _p.players.iter_mut().zip(_p.gamestates.iter_mut()) {
                let (ref mut _p, ref mut game_state) = it;
                //((x,y), z)
                match game_state {
                    &mut &mut GameState::DrawCard => {
                        println!("_p.hand.len(){}", _p.hand.len());
                        for _ in 0usize..(5 - _p.hand.len()) {
                            if let Some(n) = _p.draft.pop() {
                                _p.hand.push(n);
                            } else {
                                let mut rng = rand::thread_rng();
                                _p.draft = _p.discard.clone();
                                _p.discard = vec![];
                                rng.shuffle(&mut _p.draft);
                                if let Some(n) = _p.draft.pop() {
                                    _p.hand.push(n);
                                }
                            }
                        }
                        _p.arranged = vec![];
                        _p.wild = vec![];
                        _p.inked_cards = vec![];
                        game_state:
                    }
                    _ => {}
                }
                temp_players.push(_p.clone());
            }
*/
#[cfg(not(test))]
pub fn player_starting<T: Board>(_p: &mut Player,
                                 cardmeta: &[cards::ListCard<T>; 180],
                                 owned_deck: &mut Vec<usize>) {
    let mut collected_letter = vec![];
    let mut collected_id = vec![];
    let mut rand_id = vec![];
    let mut two_cards_id = vec![];
    let mut remaining_deck = vec![];
    for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in cardmeta.iter() {
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
        println!("c {}", c);
        if let Some(&idz) = rand_id.get(c) {
            two_cards_id.push(idz);
            rand_id.remove(c);
            owned_deck.push(idz);
        }
    }
    collected_id.extend(two_cards_id.clone());
    rng.shuffle(&mut collected_id);
    let vecdraft = collected_id.split_off(5);
    _p.hand = collected_id;
    _p.draft = vecdraft;
}
#[cfg(test)]
pub fn player_starting<T: Board>(_p: &mut Player,
                                 cardmeta: &[cards::ListCard<T>; 180],
                                 owned_deck: &mut Vec<usize>) {
    _p.hand = vec![145, 152, 158, 172, 159];
    _p.draft = vec![141, 148, 150, 177, 70];
    owned_deck.extend(_p.hand.clone());
    owned_deck.extend(_p.draft.clone());
}
