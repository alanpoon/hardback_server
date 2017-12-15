extern crate websocket;
extern crate futures;
extern crate tokio_core;
extern crate rust_wordnik;
extern crate rand;
#[macro_use]
extern crate serde_json;
pub extern crate hardback_codec;
pub extern crate hardback_server;
pub use hardback_codec as codec_lib;

use hardback_server::game_logic::game_engine::*;
use codec_lib::codec::*;
use std::sync::mpsc;
use std::collections::HashMap;
use websocket::message::OwnedMessage;
use hardback_server::drafttest::{ShortRec, TheRomanceDraftStruct, shortrec_process};

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

#[test]
fn doubleadjacent() {
    let (tx, rx) = mpsc::channel();
    let (con_tx, con_rx) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let connections: HashMap<usize, Connection> = [(0,
                                                    Connection {
                                                        name: "DefaultPlayer".to_owned(),
                                                        player_num: Some(0),
                                                        sender: con_tx,
                                                    })]
            .iter()
            .cloned()
            .collect();
    std::thread::spawn(|| {
                           let mut log: Vec<ClientReceivedMsg> = vec![];
                           GameEngine::new(vec![p], connections).run(rx,
                                                                     TheRomanceDraftStruct {},
                                                                     &mut log);
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(105, false, None, false),
                                (135, false, None, false),
                                (108, false, Some("a".to_owned()), false),
                                (110, false, Some("p".to_owned()), false),
                                (111, false, Some("t".to_owned()), false)]);
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 2 + assert 3
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 4
        let mut k3 = GameCommand::new();
        k3.reply = Some(0);
        k3.killserver = Some(true);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);
    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| shortrec_process(index, x, 0));
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.coin = 10;
    p.arranged = vec![(105, false, None, false),
                      (135, false, None, false),
                      (108, false, Some("a".to_owned()), false),
                      (110, false, Some("p".to_owned()), false),
                      (111, false, Some("t".to_owned()), false)];
    p.hand = vec![105, 135, 108, 110, 111];
    //doubleadjacent
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //assert 1
    p.coin += 7;
    p.vp += 2;
    p.skip_cards.push(105);
    p.skip_cards.push(135);
    //assert 2
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::WaitForReply],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //assert 3
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,105,
                                       "There are a few valid cards adjacent to this card, can be doubled."
                                           .to_owned(),
                                       vec!["Continue".to_owned()],None))));

    //assert 5
    p.vp += 2;
    p.coin += 3;
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Buy],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
}
#[test]
fn trash_other() {
    let (tx, rx) = mpsc::channel();
    let (con_tx, con_rx) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let connections: HashMap<usize, Connection> = [(0,
                                                    Connection {
                                                        name: "DefaultPlayer".to_owned(),
                                                        player_num: Some(0),
                                                        sender: con_tx,
                                                    })]
            .iter()
            .cloned()
            .collect();
    std::thread::spawn(|| {
                           let mut log: Vec<ClientReceivedMsg> = vec![];
                           GameEngine::new(vec![p], connections).run(rx,
                                                                     TheRomanceDraftStruct {},
                                                                     &mut log);
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(105, false, Some("a".to_owned()), false),
                                (135, false, Some("d".to_owned()), false),
                                (108, false, Some("a".to_owned()), false),
                                (110, false, None, false),
                                (111, false, Some("t".to_owned()), false)]);
        /*
                        purchase         giveable        genre                 trash
        (110,"s",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:thrash other
            b.trash_other(p,c,w);
        })),None),
        */
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 2 + assert 3
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 4
        let mut k3 = GameCommand::new();
        k3.reply = Some(0);
        k3.killserver = Some(true);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 5
        let mut k4 = GameCommand::new();
        k4.trash_other = Some((true, 0));
        tx.send((0, k4)).unwrap();
        std::thread::sleep(three_seconds);

    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| {
        shortrec_process(index,x,0)
    });
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.coin = 10;
    p.arranged = vec![(105, false, Some("a".to_owned()), false),
                      (135, false, Some("d".to_owned()), false),
                      (108, false, Some("a".to_owned()), false),
                      (110, false, None, false),
                      (111, false, Some("t".to_owned()), false)];
    p.hand = vec![105, 135, 108, 110, 111];
    p.draft = vec![];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.skip_cards.push(110);
    p.vp+=1;                                 
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::WaitForReply],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //assert 3
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,
                                       110,
                                       "Do you want to trash another card for one cent?"
                                           .to_owned(),
                                       vec!["Yes".to_owned(), "No".to_owned()],
                                       None))));
    //assert 4
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TrashOther(110)],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //assert 5
    p.hand = vec![135, 108, 110, 111];
    p.coin += 1;
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Buy],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
}
#[test]
fn keep_or_discard() {
    let (tx, rx) = mpsc::channel();
    let (con_tx, con_rx) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let connections: HashMap<usize, Connection> = [(0,
                                                    Connection {
                                                        name: "DefaultPlayer".to_owned(),
                                                        player_num: Some(0),
                                                        sender: con_tx,
                                                    })]
            .iter()
            .cloned()
            .collect();
    std::thread::spawn(|| {
                           let mut log: Vec<ClientReceivedMsg> = vec![];
                           GameEngine::new(vec![p], connections).run(rx,
                                                                     TheRomanceDraftStruct {},
                                                                     &mut log);
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(105, false, Some("a".to_owned()), false),
                                (135, false, Some("d".to_owned()), false),
                                (108, false, Some("a".to_owned()), false),
                                (111, false, None, false),
                                (110, false, None, false)]);
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 2 + assert 3
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 4
        let mut k3 = GameCommand::new();
        k3.reply = Some(1);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);

        let mut k4 = GameCommand::new();
        k4.reply = Some(0);
        tx.send((0, k4)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k5 = GameCommand::new();
        k5.putback_discard = Some(true);
        k5.killserver = Some(true);
        tx.send((0, k5)).unwrap();
        std::thread::sleep(three_seconds);
    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| {
       shortrec_process(index,x,0)
    });
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.coin = 10;
    p.arranged = vec![(105, false, Some("a".to_owned()), false),
                                (135, false, Some("d".to_owned()), false),
                                (108, false, Some("a".to_owned()), false),
                                (111, false, None, false),
                                (110, false, None, false)];
    p.hand = vec![105, 135, 108, 110, 111];
    p.draft = vec![];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.vp += 5;
    p.skip_cards.push(111);
    p.skip_cards.push(110);
    //assert 2
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::WaitForReply],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //assert 3
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,
                                       110,
                                       "Do you want to trash another card for one cent?"
                                           .to_owned(),
                                       vec!["Yes".to_owned(), "No".to_owned()],
                                       None))));
    assert_eq!(iter_o.next(),
            Some(ShortRec::Board(BoardCodec {
                                    players: vec![p.clone()],
                                    gamestates: vec![GameState::WaitForReply],
                                    offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                    turn_index: 0,
                                    ticks: None,
                                })));
    //assert 3
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,111,
                                       "You may draw three cards from the top of deck and choose to keep or discard each of them."
                                           .to_owned(),
                                       vec!["Continue".to_owned()],None))));
    //assert 4
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::PutBackDiscard(2, 111)],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));

}
