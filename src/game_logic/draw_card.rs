use server_lib::codec::*;
use server_lib::cards::{self, Board, WaitForInputType, WaitForSingleInput};
use game_logic::board::BoardStruct;
use game_logic::game_engine::{continue_to_prob, continue_to_broadcast, GameCon};
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
                _p.skip_cards = vec![];
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
    let mut needtempboardcast = false;
    let mut need_turn_index = false;
    println!("gamestate in update {:?}", gamestates.clone());
    if let Some(ref mut _g) = gamestates.get_mut(turn_index) {
        if let GameState::DrawCard = **_g {
            **_g = GameState::TurnToSubmit;
            needtempboardcast = true;
            need_turn_index = true;
        }
    }
    if needtempboardcast {
        println!("needtempboardcast true");
        for _con in cons.iter() {
            let offer_row = (0..7).zip(remaining_cards.iter()).map(|(e, c)| c.clone()).collect();
            if need_turn_index {
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
pub fn uncover_cards<T: GameCon>(players: &mut Vec<Player>,
                                 gamestates: &mut Vec<GameState>,
                                 connections: &Vec<T>,
                                 cardmeta: &[cards::ListCard<BoardStruct>; 180],
                                 remaining_cards: &Vec<usize>,
                                 wait_vec: &mut [WaitForInputType; 4],
                                 turn_index: usize) {

    let mut tempboard = BoardStruct::new(players.clone(), &remaining_cards);
    let mut player_that_responsible = None;
    for (player_id, ref mut _gamestates) in
        (0..tempboard.players.len()).zip(gamestates.iter_mut()) {
        match _gamestates {
            &mut &mut GameState::ResolveAgain(_, _) => {

                player_that_responsible = Some(player_id);
                **_gamestates = GameState::Buy;
                println!("resolveagain {:?}",players.clone().get(player_id));

                game_logic::resolve_cards::resolve_cards(&mut tempboard,
                                                         player_id,
                                                         &cardmeta,
                                                         wait_vec);
            }
            _ => {}
        }
    }
    if let Some(player_that_responsible) = player_that_responsible {
        for it in tempboard.players.iter().zip(players.iter_mut()) {
            let (_tb_p, mut _p) = it;
            *_p = _tb_p.clone();
        }

        let game_state_c = gamestates.clone();
        println!("game_sssstate {:?}", game_state_c);
        match wait_vec[player_that_responsible].first() {
            Some(&Some(ref x)) => {
                println!("there is some {:?}", x.0.clone());
            }
            Some(&None) => {
                println!("there is really none");
            }
            _ => {
                println!("there is no first");
            }

        }
        if let (Some(ref mut _g), ref mut _w) =
            (gamestates.get_mut(player_that_responsible), &mut wait_vec[player_that_responsible]) {
            continue_to_broadcast::<T>(_w,
                                       &connections,
                                       &remaining_cards,
                                       players.clone(),
                                       game_state_c.clone(),
                                       turn_index);
            if let (_w, Some(_con)) = (_w, connections.get(player_that_responsible)) {
                continue_to_prob::<T>(_w, _g, _con);
            }
        }
    }


}
