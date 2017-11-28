use game_logic;
use codec_lib::cards;
use codec_lib::codec::*;
use game_logic::game_engine::GameCon;
use game_logic::board::BoardStruct;
use websocket::message::OwnedMessage;
use std::sync::mpsc;
pub struct TheNormalDraftStruct {}
impl game_logic::game_engine::TheDraft for TheNormalDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown:&mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.hand = vec![147, 154, 160, 174, 161];
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        let mut remaining_deck = vec![];
        for &cards::ListCard { id, .. } in _cardmeta.iter().rev() {
            if !owned_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
    fn ticks(&self) -> Option<u16> {
        None
    }
    fn show_draft(&self) -> (bool, bool) {
        (false, false)
    }
}
pub struct TheNotifyDraftStruct {}
impl game_logic::game_engine::TheDraft for TheNotifyDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.draft = vec![141, 148, 7, 177, 70, 7, 14, 20, 18, 4];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, id, .. } in _cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
    fn ticks(&self) -> Option<u16> {
        None
    }
    fn show_draft(&self) -> (bool, bool) {
        (true, true)
    }
}
pub struct TheAdventureDraftStruct {}
impl game_logic::game_engine::TheDraft for TheAdventureDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.hand = vec![7, 14, 20, 18, 4];
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { id, .. } in _cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
    fn ticks(&self) -> Option<u16> {
        None
    }
    fn show_draft(&self) -> (bool, bool) {
        (false, false)
    }
}
pub struct TheHorrorDraftStruct {}
impl game_logic::game_engine::TheDraft for TheHorrorDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.hand = vec![41, 48, 54, 52, 38];
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in
            _cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
    fn ticks(&self) -> Option<u16> {
        None
    }
    fn show_draft(&self) -> (bool, bool) {
        (false, false)
    }
}

pub struct TheMysteryDraftStruct {}
impl game_logic::game_engine::TheDraft for TheMysteryDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.hand = vec![76, 83, 89, 87, 73];
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in
            _cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
    fn ticks(&self) -> Option<u16> {
        None
    }
    fn show_draft(&self) -> (bool, bool) {
        (false, false)
    }
}
pub struct TheMysteryUnCoverDraftStruct {}
impl game_logic::game_engine::TheDraft for TheMysteryUnCoverDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.hand = vec![42, 72, 178, 82, 73];
        _p.draft = vec![141, 148, 7, 177, 70]; //82 is one vp per wild
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { id, .. } in _cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
    fn ticks(&self) -> Option<u16> {
        None
    }
    fn show_draft(&self) -> (bool, bool) {
        (false, false)
    }
}
pub struct TheRomanceDraftStruct {}
impl game_logic::game_engine::TheDraft for TheRomanceDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.hand = vec![105, 135, 108, 110, 111]; //105 is doubleadjacent,110 is trash other card,111 is keep_or_discard_three
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in
            _cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
    fn ticks(&self) -> Option<u16> {
        None
    }
    fn show_draft(&self) -> (bool, bool) {
        (false, false)
    }
}
pub struct TheOverlayDraftStruct {}
impl game_logic::game_engine::TheDraft for TheOverlayDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.ink = 3;
        _p.remover = 2;
        _p.hand = vec![105, 135, 108, 110, 111]; //105 is doubleadjacent,110 is trash other card,111 is keep_or_discard_three
        _p.draft = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        //start 4coin,4ink
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in
            _cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
    fn ticks(&self) -> Option<u16> {
        None
    }
    fn show_draft(&self) -> (bool, bool) {
        (false, false)
    }
}
pub struct TheTwoPlayerDraftStruct {}
impl game_logic::game_engine::TheDraft for TheTwoPlayerDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        let player_index = (owned_deck.len() as f64 / 10.0).floor() as usize;
        if player_index == 0 {
            _p.hand = vec![105, 135, 108, 110, 111]; //105 is doubleadjacent,110 is trash other card,111 is keep_or_discard_three
            _p.draft = vec![141, 148, 7, 177, 70];
        } else {
            _p.hand = vec![90, 49, 2, 75, 77]; //v,p,c,g,i
            _p.draft = vec![84, 130, 12, 34, 91]; //p,e,m,y,w
        }
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_p.draft.clone());
    }
    fn deck_starting(&self,
                     _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     owned_deck: &Vec<usize>)
                     -> Vec<usize> {
        let mut remaining_deck = vec![26, 23, 38, 80, 94, 98, 119, 1]; //a:26 use ink,x:23 can afford,d:38 cannot afford,l:80,94,98,119
        let mut owned_reserved_deck = owned_deck.clone();
        owned_reserved_deck.extend(remaining_deck.clone());
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in
            _cardmeta.iter().rev() {
            if !owned_reserved_deck.contains(&id) {
                remaining_deck.push(id);
            }
        }
        remaining_deck
    }
    fn ticks(&self) -> Option<u16> {
        None
    }
    fn show_draft(&self) -> (bool, bool) {
        (false, false)
    }
}
#[derive(Clone)]
pub struct Connection {
    pub name: String,
    pub player_num: Option<usize>,
    pub sender: mpsc::Sender<OwnedMessage>,
}

impl GameCon for Connection {
    fn tx_send(&self, msg: ClientReceivedMsg, log: &mut Vec<ClientReceivedMsg>) {
        let ClientReceivedMsg { boardstate, request, .. } = msg.clone();
        if let Some(Some(_)) = boardstate.clone() {
            if let Some(0) = self.player_num {
                log.push(msg.clone());
            }
        } else if let Some(Some(_)) = request.clone() {
            log.push(msg.clone());
        }

        self.sender
            .clone()
            .send(OwnedMessage::Text(ClientReceivedMsg::serialize_send(msg).unwrap()))
            .unwrap();
    }
}
#[derive(Debug,PartialEq,Clone)]
pub enum ShortRec {
    Board(BoardCodec),
    Request((usize, usize, String, Vec<String>, Option<u16>)), //player_index,card_index,Vec of option,
    TurnIndex(usize),
    PlayerIndex(usize),
    None,
}
