use server_lib::codec::*;
use server_lib::cards::{self, Board, WaitForInputType};
use game_logic::board::BoardStruct;
use game_logic::game_engine::GameCon;
use websocket::message::OwnedMessage;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use rand;
use game_logic;
pub struct TheDraftStruct {}

impl game_logic::game_engine::TheDraft for TheDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       cardmeta: &[cards::ListCard<BoardStruct>; 180],
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
    fn deck_starting(&self,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        let mut remaining_deck = vec![];
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in cardmeta.iter() {
            if !owned_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut remaining_deck);
        remaining_deck
    }
}
#[cfg(not(test))]
pub fn redraw_cards_to_hand_size(players: &mut Vec<Player>,
                                 gamestates: &mut Vec<GameState>,
                                 turn_index: &mut usize) {
    let player_num = players.len();
    for mut it in players.iter_mut().zip(gamestates.iter_mut()) {
        let (ref mut _p, ref mut game_state) = it;
        //((x,y), z)
        match game_state {
            &mut &mut GameState::DrawCard => {
                _p.discard = _p.hand.clone();
                _p.hand = vec![];
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
                _p.inked_cards = vec![];
                if *turn_index < player_num - 1 {
                    *turn_index += 1;
                } else {
                    *turn_index = 0;
                }
            }
            _ => {}
        }
    }
}
#[cfg(test)]
pub fn redraw_cards_to_hand_size(players: &mut Vec<Player>,
                                 gamestates: &mut Vec<GameState>,
                                 turn_index: &mut usize) {
    let player_num = players.len();
    for mut it in players.iter_mut().zip(gamestates.iter_mut()) {
        let (ref mut _p, ref mut game_state) = it;
        //((x,y), z)
        match game_state {
            &mut &mut GameState::DrawCard => {
                _p.discard = _p.hand.clone();
                _p.hand = vec![];
                for _ in 0usize..(5 - _p.hand.len()) {
                    if let Some(n) = _p.draft.pop() {
                        _p.hand.push(n);
                    } else {
                        _p.draft = _p.discard.clone();
                        _p.discard = vec![];
                        if let Some(n) = _p.draft.pop() {
                            _p.hand.push(n);
                        }
                    }
                }
                _p.arranged = vec![];
                _p.inked_cards = vec![];
                _p.skip_cards =vec![];
                if *turn_index < player_num - 1 {
                    *turn_index += 1;
                } else {
                    *turn_index = 0;
                }
            }
            _ => {}
        }
    }
}
pub fn update_gamestates<T: GameCon>(gamestates: &mut Vec<GameState>,
                                     cons: &Vec<T>,
                                     players: &Vec<Player>,
                                     remaining_cards: &Vec<usize>,
                                     turn_index: usize) {
    let mut need_boardcast = false;
    let mut need_turn_index = false;
    if let Some(ref mut _g) = gamestates.get_mut(turn_index) {
        if let GameState::DrawCard = **_g {
            **_g = GameState::TurnToSubmit;
            need_boardcast = true;
            need_turn_index = true;
        }
        else if let GameState::UncoverAdjacent(_,_)=**_g{
            **_g = GameState::TurnToSubmit;
            need_boardcast = true;
        }
    }
    if need_boardcast {
        for _con in cons.iter() {
            let offer_row = (0..7).zip(remaining_cards.iter()).map(|(e, c)| c.clone()).collect();
            if need_turn_index{
            let g = json!({
                              "turn_index": turn_index
                          });
            _con.tx_send(OwnedMessage::Text(g.to_string()));
            }
            let k: Result<BoardCodec, String> = Ok(BoardCodec {
                                                       players: players.clone(),
                                                       gamestates: gamestates.clone(),
                                                       offer_row: offer_row,
                                                       turn_index: turn_index,
                                                   });
            let h = json!({
                              "boardstate": k
                          });
            _con.tx_send(OwnedMessage::Text(h.to_string()));
        }
    }

}
pub fn uncover_cards(players: &mut Vec<Player>,
                     gamestates: &mut Vec<GameState>,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     remaining_cards: &Vec<usize>,
                     turn_index: &mut usize,
                     wait_vec) {
        let mut _board = BroadStruct::new(players.clone(),&remaining_cards);
    for mut it in _board.players.iter_mut().zip(gamestates.iter_mut()) {
        let (ref mut _p, ref mut game_state) = it;
        //((x,y), z)
        match game_state {
            &mut &mut GameState::UncoverAdjacent(position_card, uncoverer_card_index) => {
                println!("uncoverinngg.g.");
                if let Some(_position_card) = position_card {
                    if _position_card == 0 {
                        if let Some(&mut (covered_card, ref mut _wild)) = _p.arranged.get_mut(1) {
                            //wild
                            if let &mut Some(_) = _wild {
                                *_wild = None;
                            }
                        }
                    } else if _position_card == _p.arranged.len() - 1 {
                        if let Some(&mut (covered_card, ref mut _wild)) =
                            _p.arranged.get_mut(_position_card - 1) {
                            //wild
                            if let &mut Some(_) = _wild {
                                *_wild = None;
                            }
                        }
                    } else {
                        if let Some(&mut (covered_card, ref mut _wild)) =
                            _p.arranged.get_mut(_position_card - 1) {
                            //wild
                            if let &mut Some(_) = _wild {
                                *_wild = None;
                            }
                        }
                        if let Some(&mut (covered_card, ref mut _wild)) =
                            _p.arranged.get_mut(_position_card + 1) {
                            //wild
                            if let &mut Some(_) = _wild {
                                *_wild = None;
                            }
                        }
                    }
                    //resolve cards again
                   
                    game_logic::resolve_cards::resolve_cards(_board.clone(),
                                                                         player_id,
                                                                         &cardmeta,
                                                                         wait_vec);
                }
            }
            _ => {}
        }
    }
}
