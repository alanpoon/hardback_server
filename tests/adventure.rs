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
use hardback_server::drafttest::{ShortRec, TheAdventureDraftStruct};

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
fn adventure() {
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
                                                                     TheAdventureDraftStruct {},
                                                                     &mut log);
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(7, false, Some("h".to_owned()), false),
                                (14, false, Some("o".to_owned()), false),
                                (20, false, Some("u".to_owned()), false),
                                (18, false, None, false),
                                (4, false, None, false)]);

        //:s=>GIVEABLE::VP(1) purchase ,GIVEABLE::VP(2) giveable ,GIVEABLE::VP(1) genre,GIVEABLE::VP(2) thrash
        //:e=>GIVEABLE::NONE purchase ,GIVEABLE::VP(1) giveable ,GIVEABLE::VP(1) genre,GIVEABLE::COIN(2) thrash
        //  k.killserver = Some(true);
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 2 + assert 3
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);

        //assert 4 + assert 5
        let mut k3 = GameCommand::new();
        k3.reply = Some(0);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);

        //assert 6
        let mut k4 = GameCommand::new(); //say no to trash card
        k4.reply = Some(1);
        tx.send((0, k4)).unwrap();
        std::thread::sleep(three_seconds);

        //assert 7
        let mut k5 = GameCommand::new(); //buy first card, not enough coin
        k5.buy_offer = Some((true, 0));
        k5.killserver = Some(true);
        tx.send((0, k5)).unwrap();
        std::thread::sleep(three_seconds);

        //assert 8 + assert 9
        let mut k6 = GameCommand::new(); //go to drawCard
        k6.reply = Some(1);
        tx.send((0, k6)).unwrap();
        std::thread::sleep(three_seconds);

        //assert 8 + assert 9
        let mut k6 = GameCommand::new(); //go to drawCard
        k6.reply = Some(1);
        tx.send((0, k6)).unwrap();
        std::thread::sleep(three_seconds);
    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| {
        let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, request, turn_index, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate:{:?}", index);
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
    p.arranged = vec![(7, false, Some("h".to_owned()), false),
                      (14, false, Some("o".to_owned()), false), //two_cent_per_adv
                      (20, false, Some("u".to_owned()), false),
                      (18, false, None, false),
                      (4, false, None, false)];
    p.hand = vec![7, 14, 20, 18, 4];
    p.draft = vec![]; //141, 148, 7, 177, 70
    //assert 1

    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));

    //Test submit word, you can trash cards for benefit
    p.vp = 5;
    p.skip_cards = vec![18, 4];
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
                                       18,
                                       "Do you want to trash this card for the benefit?"
                                           .to_owned(),
                                       vec!["Yes".to_owned(), "No".to_owned()],
                                       None))));
    //assert 4
    p.vp += 2;
    p.hand = vec![7, 14, 20, 4];

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
                                       4,
                                       "Do you want to trash this card for the benefit?"
                                           .to_owned(),
                                       vec!["Yes".to_owned(), "No".to_owned()],
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

    //assert 7
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,26,"You can't afford to buy this card. Do you want to buy another card?"
                    .to_owned(),
                                       vec!["Yes".to_owned(),
                              "No, I want to end my buy phase".to_owned()],None))));

    //assert 8
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::DrawCard],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //assert 9
    assert_eq!(iter_o.next(), Some(ShortRec::TurnIndex(0)));
    p.discard = vec![];
    p.arranged = vec![];
    p.hand = vec![70, 177, 7, 148, 141];
    p.draft = vec![];
    p.draftlen -= 1;
    p.skip_cards = vec![];
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
}
