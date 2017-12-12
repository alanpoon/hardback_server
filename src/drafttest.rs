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
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.hand = vec![147, 154, 160, 174, 161];
        _p.draft = vec![];
        *_unknown = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_unknown.clone());
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
    fn push_notification(&self) -> bool {
        true
    }
}
pub struct TheNotifyDraftStruct {}
impl game_logic::game_engine::TheDraft for TheNotifyDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.draft = vec![141, 148, 7, 177, 70, 90, 14, 20, 18, 4];
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
        //(notify player_turn,random shuffle or not)
        (true, true)
    }
    fn push_notification(&self) -> bool {
        false
    }
}
pub struct TheAdventureDraftStruct {}
impl game_logic::game_engine::TheDraft for TheAdventureDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.hand = vec![7, 14, 20, 18, 4];
        _p.draft = vec![];
        *_unknown = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_unknown.clone());
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
    fn push_notification(&self) -> bool {
        false
    }
}
pub struct TheHorrorDraftStruct {}
impl game_logic::game_engine::TheDraft for TheHorrorDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.draft = vec![];
        _p.hand = vec![41, 48, 54, 52, 38];
        *_unknown = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_unknown.clone());
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
    fn push_notification(&self) -> bool {
        false
    }
}

pub struct TheMysteryDraftStruct {}
impl game_logic::game_engine::TheDraft for TheMysteryDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.draft = vec![];
        _p.hand = vec![76, 83, 89, 87, 73];
        *_unknown = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_unknown.clone());
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
    fn push_notification(&self) -> bool {
        false
    }
}
pub struct TheMysteryUnCoverDraftStruct {}
impl game_logic::game_engine::TheDraft for TheMysteryUnCoverDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.draft = vec![];
        _p.hand = vec![42, 72, 178, 82, 73];
        *_unknown = vec![141, 148, 7, 177, 70]; //82 is one vp per wild
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_unknown.clone());
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
    fn push_notification(&self) -> bool {
        false
    }
}
pub struct TheRomanceDraftStruct {}
impl game_logic::game_engine::TheDraft for TheRomanceDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.draft = vec![];
        _p.hand = vec![105, 135, 108, 110, 111]; //105 is doubleadjacent,110 is trash other card,111 is keep_or_discard_three
        *_unknown = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_unknown.clone());
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
    fn push_notification(&self) -> bool {
        false
    }
}
pub struct TheTimelessDraftStruct {}
impl game_logic::game_engine::TheDraft for TheTimelessDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        let player_index = (owned_deck.len() as f64 / 10.0).floor() as usize;
        if player_index == 0 {
            _p.timeless_classic = vec![136, 96, 135]; //:r ,gain 2vp, ++trash another card, :a, gain 2coin,++gain 1 coin
            _p.hand = vec![105, 99, 108, 110, 124]; //105 is doubleadjacent,110 is trash other card,111 is keep_or_discard_three
            //z,o,u,s,e
            *_unknown = vec![141, 148, 7, 177, 70];
            //forms 136,135,110,124->rose
        } else {
            _p.timeless_classic = vec![101]; //:t ,gain 2vp,++gain 2vp, lockup card
            _p.hand = vec![90, 49, 2, 75, 77]; //v,p,c,g,i
            *_unknown = vec![84, 130, 12, 34, 91]; //p,e,m,y,w
            //forms 2,96,49->cap
        }

        _p.coin = 10;
        _p.draft = vec![];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_unknown.clone());
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
    fn push_notification(&self) -> bool {
        false
    }
}
pub struct TheOverlayDraftStruct {}
impl game_logic::game_engine::TheDraft for TheOverlayDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        _p.coin = 10;
        _p.draft = vec![];
        _p.ink = 3;
        _p.remover = 2;
        _p.hand = vec![105, 135, 108, 110, 111]; //105 is doubleadjacent,110 is trash other card,111 is keep_or_discard_three
        *_unknown = vec![141, 148, 7, 177, 70];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_unknown.clone());
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
    fn push_notification(&self) -> bool {
        false
    }
}
pub struct TheTwoPlayerDraftStruct {}
impl game_logic::game_engine::TheDraft for TheTwoPlayerDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        let player_index = (owned_deck.len() as f64 / 10.0).floor() as usize;
        if player_index == 0 {
            _p.hand = vec![105, 135, 108, 110, 111]; //105 is doubleadjacent,110 is trash other card,111 is keep_or_discard_three
            *_unknown = vec![141, 148, 7, 177, 70];
        } else {
            _p.hand = vec![90, 49, 2, 75, 77]; //v,p,c,g,i
            *_unknown = vec![84, 130, 12, 34, 91]; //p,e,m,y,w
        }
        _p.draft = vec![];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_unknown.clone());
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
    fn push_notification(&self) -> bool {
        false
    }
}

pub struct TheEndGameDraftStruct {}
impl game_logic::game_engine::TheDraft for TheEndGameDraftStruct {
    fn player_starting(&self,
                       _p: &mut Player,
                       _unknown: &mut Vec<usize>,
                       _cardmeta: &[cards::ListCard<BoardStruct>; 180],
                       owned_deck: &mut Vec<usize>) {
        let player_index = (owned_deck.len() as f64 / 10.0).floor() as usize;
        if player_index == 0 {
            _p.hand = vec![143, 135, 108, 110, 111]; //105 is doubleadjacent,110 is trash other card,111 is keep_or_discard_three
            *_unknown = vec![141, 148, 7, 177, 70];
            _p.vp=59;
        } else {
            _p.hand = vec![90, 49, 2, 75, 159]; //v,p,c,g,i
            *_unknown = vec![84, 130, 12, 34, 91]; //p,e,m,y,w
        }
        
        _p.draft = vec![];
        owned_deck.extend(_p.hand.clone());
        owned_deck.extend(_unknown.clone());
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
    fn push_notification(&self) -> bool {
        false
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
    PushNotification(String),
    Hand(Vec<usize>),
    None,
}
pub fn shortrec_process(index:usize,ownm:OwnedMessage,j:usize)->ShortRec{
let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = ownm {
            if let Ok(ClientReceivedMsg { boardstate, request, turn_index, player_index,notification,hand, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate{:?}:{:?}",j, index);
                if let Some(Some(Ok(_boardstate))) = boardstate {
                    y = ShortRec::Board(_boardstate);
                } else if let Some(Some(_request)) = request {
                    y = ShortRec::Request(_request);
                } else if let Some(Some(_turn_index)) = turn_index {
                    y = ShortRec::TurnIndex(_turn_index);
                } else if let Some(Some(_player_index)) = player_index {
                    y = ShortRec::PlayerIndex(_player_index);
                } else if let Some(Some(_notif)) = notification{
                    y = ShortRec::PushNotification(_notif);
                }else if let Some(Some(_hand)) = hand{
                    y = ShortRec::Hand(_hand);
                }

            }
        }
        y
}