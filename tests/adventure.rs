extern crate websocket;
extern crate futures;
extern crate tokio_core;
extern crate rust_wordnik;
extern crate rand;
#[macro_use]
extern crate serde_json;
pub extern crate hardback_server_lib;
pub extern crate hardback_server;
pub use hardback_server_lib as server_lib;

use hardback_server::game_logic::game_engine::*;
use server_lib::codec::*;
use server_lib::cards;
use server_lib::cards::*;
use hardback_server::game_logic::board::BoardStruct;
use hardback_server::game_logic;
use std::sync::mpsc;
use websocket::message::OwnedMessage;
use hardback_server::testdraft::TheAdventureDraftStruct;

#[derive(Clone)]
pub struct Connection {
    pub name: String,
    pub player_num: Option<usize>,
    pub sender: mpsc::Sender<OwnedMessage>,
}
impl GameCon for Connection {
    fn tx_send(&self, msg: OwnedMessage) {
        self.sender
            .clone()
            .send(msg)
            .unwrap();
    }
}
#[derive(Debug,PartialEq,Clone)]
enum ShortRec {
    board(BoardCodec),
    request((usize, String, Vec<String>)),
    turn_index(usize),
    None,
}
#[test]
fn arrange_adventure_card() {
    let (tx, rx) = mpsc::channel();
    let (con_tx, con_rx) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let connections = vec![Connection {
                               name: "DefaultPlayer".to_owned(),
                               player_num: Some(0),
                               sender: con_tx,
                           }];
    std::thread::spawn(|| {
                           GameEngine::new(vec![p], connections).run(rx,
                                                                     TheAdventureDraftStruct {});
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(7, Some("h".to_owned())),
                                (14, Some("o".to_owned())),
                                (20, Some("u".to_owned())),
                                (18, None),
                                (4, None)]);

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
        k5.buyoffer = Some((true, 0));
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
                    y = ShortRec::board(_boardstate);
                } else if let Some(Some(_request)) = request {
                    y = ShortRec::request(_request);
                } else if let Some(Some(_turn_index)) = turn_index {
                    y = ShortRec::turn_index(_turn_index);
                }
            }
        }
        y
    });
    let h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.arranged = vec![(7, Some("h".to_owned())),
                      (14, Some("o".to_owned())), //two_cent_per_adv
                      (20, Some("u".to_owned())),
                      (18, None),
                      (4, None)];
    p.hand = vec![7, 14, 20, 18, 4];
    p.draft = vec![141, 148, 7, 177, 70];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));

    //Test submit word, you can trash cards for benefit
    p.vp = 5;
    p.skip_cards = vec![18, 4];
    //assert 2
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));
    //assert 3
    assert_eq!(iter_o.next(),
               Some(ShortRec::request((18,
                                       "Do you want to trash this card for the benefit?"
                                           .to_owned(),
                                       vec!["Yes".to_owned(), "No".to_owned()]))));
    //assert 4
    p.vp += 2;
    p.hand = vec![7, 14, 20, 4];

    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::WaitForReply],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));
    //assert 5
    assert_eq!(iter_o.next(),
               Some(ShortRec::request((4,
                                       "Do you want to trash this card for the benefit?"
                                           .to_owned(),
                                       vec!["Yes".to_owned(), "No".to_owned()]))));

    //assert 6
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Buy],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));

    //assert 7
    assert_eq!(iter_o.next(),
               Some(ShortRec::request((26,"You can't afford to buy this card. Do you want to buy another card?"
                    .to_owned(),
                                       vec!["Yes".to_owned(),
                              "No, I want to end my buy phase".to_owned()]))));

    //assert 8
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::DrawCard],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));
    //assert 9
    assert_eq!(iter_o.next(), Some(ShortRec::turn_index(0)));
    p.discard.extend(p.hand.clone());
    p.arranged = vec![];
    p.hand = vec![70, 177, 7, 148, 141];
    p.draft = vec![];
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));
}
