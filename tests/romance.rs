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
use websocket::message::OwnedMessage;
use hardback_server::testdraft::{ShortRec, TheRomanceDraftStruct};

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
    let connections = vec![Connection {
                               name: "DefaultPlayer".to_owned(),
                               player_num: Some(0),
                               sender: con_tx,
                           }];
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
        k1.arranged = Some(vec![(105, None),
                                (135, None),
                                (108, Some("a".to_owned())),
                                (110, Some("p".to_owned())),
                                (111, Some("t".to_owned()))]);
        /*
                        purchase         giveable        genre                 trash
        (105,"z",5,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:double adjacent
            b.double_adjacent(p,c,w);
        })),None), 
        (135,"o",8,GIVEABLE::NONE,GIVEABLE::VPCOIN(1,2),GIVEABLE::VPCOIN(1,1),GIVEABLE::NONE,Genre::ROMANCE,true,None,None),
        }))),
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
    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| {
        let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, request, turn_index, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate:{:?}", index + 1);
                if let Some(Some(Ok(_boardstate))) = boardstate {
                    y = ShortRec::Board(_boardstate);
                } else if let Some(Some(_request)) = request {
                    y = ShortRec::Request(_request);
                } else if let Some(Some(_turn_index)) = turn_index {
                    y = ShortRec::TurnIndex(_turn_index);
                }
            }
        }
        y
    });
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.coin = 10;
    p.arranged = vec![(105, None),
                      (135, None),
                      (108, Some("a".to_owned())),
                      (110, Some("p".to_owned())),
                      (111, Some("t".to_owned()))];
    p.hand = vec![105, 135, 108, 110, 111];
    p.draft = vec![141, 148, 7, 177, 70];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.coin += 7;
    p.vp += 2;
    p.skip_cards.push(105);
    p.skip_cards.push(135);
    //assert 2
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
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
    let connections = vec![Connection {
                               name: "DefaultPlayer".to_owned(),
                               player_num: Some(0),
                               sender: con_tx,
                           }];
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
        k1.arranged = Some(vec![(105, Some("a".to_owned())),
                                (135, Some("d".to_owned())),
                                (108, Some("a".to_owned())),
                                (110, None),
                                (111, Some("t".to_owned()))]);
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
        let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, request, turn_index, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate:{:?}", index + 1);
                if let Some(Some(Ok(_boardstate))) = boardstate {
                    y = ShortRec::Board(_boardstate);
                } else if let Some(Some(_request)) = request {
                    y = ShortRec::Request(_request);
                } else if let Some(Some(_turn_index)) = turn_index {
                    y = ShortRec::TurnIndex(_turn_index);
                }
            }
        }
        y
    });
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.coin = 10;
    p.arranged = vec![(105, Some("a".to_owned())),
                      (135, Some("d".to_owned())),
                      (108, Some("a".to_owned())),
                      (110, None),
                      (111, Some("t".to_owned()))];
    p.hand = vec![105, 135, 108, 110, 111];
    p.draft = vec![141, 148, 7, 177, 70];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.vp += 1;
    p.skip_cards.push(110);
    //assert 2
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
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
                                        gamestates: vec![GameState::TrashOther],
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
    let connections = vec![Connection {
                               name: "DefaultPlayer".to_owned(),
                               player_num: Some(0),
                               sender: con_tx,
                           }];
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
        k1.arranged = Some(vec![(105, Some("a".to_owned())),
                                (135, Some("d".to_owned())),
                                (108, Some("a".to_owned())),
                                (110, Some("p".to_owned())),
                                (111, None)]);
        /*
                        purchase         giveable        genre                 trash
        (111,"r",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, gen:keep or discard top3
            b.keep_or_discard_three(p,c,w);
        }))),
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

        let mut k4 = GameCommand::new();
        k4.putback_discard = Some(true);
        tx.send((0, k4)).unwrap();
        std::thread::sleep(three_seconds);
    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| {
        let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, request, turn_index, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate:{:?}", index + 1);
                if let Some(Some(Ok(_boardstate))) = boardstate {
                    y = ShortRec::Board(_boardstate);
                } else if let Some(Some(_request)) = request {
                    y = ShortRec::Request(_request);
                } else if let Some(Some(_turn_index)) = turn_index {
                    y = ShortRec::TurnIndex(_turn_index);
                }
            }
        }
        y
    });
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.coin = 10;
    p.arranged = vec![(105, Some("a".to_owned())),
                      (135, Some("d".to_owned())),
                      (108, Some("a".to_owned())),
                      (110, Some("p".to_owned())),
                      (111, None)];
    p.hand = vec![105, 135, 108, 110, 111];
    p.draft = vec![141, 148, 7, 177, 70];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.vp += 1;
    p.skip_cards.push(110);
    //assert 2
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //assert 3
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,110,
                                       "You may draw three cards from the top of deck and choose to keep or discard each of them."
                                           .to_owned(),
                                       vec!["Continue".to_owned()],None))));
    //assert 4
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::PutBackDiscard(2, 110)],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));

}
