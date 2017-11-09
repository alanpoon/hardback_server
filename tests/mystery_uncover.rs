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
use hardback_server::testdraft::{ShortRec, TheMysteryUnCoverDraftStruct};

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
fn arrange_uncover_card() {
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
                           GameEngine::new(vec![p], connections)
                               .run(rx, TheMysteryUnCoverDraftStruct {}, &mut log);
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(42, Some("a".to_owned())),
                                (72, None),
                                (178, Some("a".to_owned())),
                                (82, Some("p".to_owned())),
                                (73, Some("t".to_owned()))]);
        /*
                        purchase         giveable        genre                 trash
              (42,"i",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (72,"d",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:uncover adjacent wild
            b.uncover_adjacent(p,c,w);
        })),None),
                (178,"t",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),

        }))),
        */
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 2 + assert 3
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 4 + assert 5 feedback
        let mut k3 = GameCommand::new();
        k3.reply = Some(0);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 6 Buy
        let mut k4 = GameCommand::new();
        k4.buy_offer = Some((true, 0));
        k4.killserver = Some(true);
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
    p.arranged = vec![(42, Some("a".to_owned())),
                      (72, None),
                      (178, Some("a".to_owned())),
                      (82, Some("p".to_owned())),
                      (73, Some("t".to_owned()))];
    p.hand = vec![42, 72, 178, 82, 73];
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
    p.skip_cards.push(72);
    p.arranged = vec![(42, None),
                      (72, None),
                      (178, None),
                      (82, Some("p".to_owned())),
                      (73, Some("t".to_owned()))];
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
               Some(ShortRec::Request((0,72,
                                       "There are a few wild cards adjacent to this card, can be opened".to_owned(),
                                       vec!["Continue".to_owned()],None))));
    //assert 4
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::ResolveAgain(Some(1), 72)],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.vp += 2;
    p.coin += 1;
    p.skip_cards.push(42);
    p.skip_cards.push(178);
    //assert 5
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Buy],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));

    //assert 6
    p.discard.push(26);
    p.coin -= 5;
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::DrawCard],
                                        offer_row: vec![23, 38, 80, 94, 98, 119, 1],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
}

#[test]
fn one_vp_per_wild_card() {
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
                           GameEngine::new(vec![p], connections)
                               .run(rx, TheMysteryUnCoverDraftStruct {}, &mut log);
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(42, Some("a".to_owned())),
                                (72, Some("d".to_owned())),
                                (178, Some("a".to_owned())),
                                (82, None), //one_vp_per_wild
                                (73, None)]);

        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 2 + assert 3
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 4 Say Continue when asked. + assert 5
        let mut k3 = GameCommand::new();
        k3.reply = Some(0);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 6  reply Continue to assert 5
        let mut k4 = GameCommand::new();
        k4.reply = Some(0);
        k4.killserver = Some(true);
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
    p.arranged = vec![(42, Some("a".to_owned())),
                      (72, Some("d".to_owned())),
                      (178, Some("a".to_owned())),
                      (82, None), //one_vp_per_wild
                      (73, None)]; //uncover
    p.hand = vec![42, 72, 178, 82, 73];
    p.draft = vec![141, 148, 7, 177, 70];
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.vp += 4;
    p.coin += 2;
    p.skip_cards.push(82);
    p.skip_cards.push(73);
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
                                       82,
                                       "You gain 3 vp from this card.".to_owned(),
                                       vec!["Continue".to_owned()],
                                       None))));
    p.vp += 3;
    //assert 4
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::WaitForReply],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //assert 5
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,
                                       73,
                                       "There are no adjacent wild cards that can be flipped over."
                                           .to_owned(),
                                       vec!["Continue".to_owned()],
                                       None))));
    //assert 6
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Buy],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));

}
