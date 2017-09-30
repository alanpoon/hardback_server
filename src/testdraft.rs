use game_logic;
use server_lib::cards;
use server_lib::codec::*;
use game_logic::board::BoardStruct;
pub struct TheNormalDraftStruct {}
impl game_logic::game_engine::TheDraft for TheNormalDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.hand = vec![147, 154, 160, 174, 161];
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        let mut remaining_deck = vec![];
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in cardmeta.iter().rev() {
            if !owned_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
}
pub struct TheAdventureDraftStruct {}
impl game_logic::game_engine::TheDraft for TheAdventureDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.hand = vec![7, 14, 20, 18, 4];
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
}
pub struct TheHorrorDraftStruct {}
impl game_logic::game_engine::TheDraft for TheHorrorDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.hand = vec![41, 48, 54, 52, 38];
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
}

pub struct TheMysteryDraftStruct {}
impl game_logic::game_engine::TheDraft for TheMysteryDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.hand = vec![76, 83, 89, 87, 73];
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
}
pub struct TheMysteryUnCoverDraftStruct {}
impl game_logic::game_engine::TheDraft for TheMysteryUnCoverDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.hand = vec![42, 72, 178, 82, 73];
        _p.draft = vec![141, 148, 7, 177, 70]; //82 is one vp per wild
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
}
pub struct TheRomanceDraftStruct {}
impl game_logic::game_engine::TheDraft for TheRomanceDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.hand = vec![105, 135, 108, 110, 111]; //105 is doubleadjacent,110 is trash other card,111 is keep_or_discard_three
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
}
