extern crate websocket;
extern crate futures;
extern crate tokio_core;
extern crate rust_wordnik;
extern crate rand;
extern crate serde_json;
pub extern crate hardback_codec;
pub extern crate hardback_server;
pub use hardback_codec as codec_lib;

use hardback_server::game_logic::game_engine::*;
use codec_lib::codec::*;
use codec_lib::cards;
//use codec_lib::cards::*;
use hardback_server::game_logic::board::BoardStruct;
//use hardback_server::game_logic;
use std::sync::mpsc;
use std::collections::HashMap;
use websocket::message::OwnedMessage;
use hardback_server::drafttest::{ShortRec, TheNormalDraftStruct, shortrec_process,redraw};
use rand::{thread_rng, Rng, SeedableRng, StdRng};

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
fn check_cardmeta() {
    let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
    for _k in cardmeta.iter().enumerate() {
        assert_eq!(_k.0, _k.1.id);
    }
}
#[test]
fn player_starting() {
    let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
    let mut remaining_deck = vec![];
    let mut _p = Player::new("defaultname".to_owned());
    let _normal_draft = TheNormalDraftStruct {};
    let mut unknown = vec![];
    _normal_draft.player_starting(&mut _p, &mut unknown, &cardmeta, &mut remaining_deck);
    assert_eq!(_p.draft.len(), 5);
    assert_eq!(_p.hand.len(), 5);
}

#[test]
fn normal() {
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
                                                                     TheNormalDraftStruct {},
                                                                     &mut log);
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(2, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(147, false, None, false),
                                (154, false, None, false),
                                (160, false, None, false),
                                (174, false, None, false),
                                (161, false, None, false)]);
        //  k.killserver = Some(true);
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 2
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 3
        let mut k3 = GameCommand::new();
        k3.buy_offer = Some((true, 0));
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k4 = GameCommand::new();
        k4.arranged = Some(vec![(177, false, None, false)]);
        tx.send((0, k4)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k5 = GameCommand::new();
        k5.submit_word = Some(true);
        tx.send((0, k5)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k6 = GameCommand::new();
        k6.buy_offer = Some((true, 0));
        tx.send((0, k6)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k7 = GameCommand::new();
        k7.arranged = Some(vec![(154, false, None, false)]);
        tx.send((0, k7)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k5 = GameCommand::new();
        k5.submit_word = Some(true);
        tx.send((0, k5)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k6 = GameCommand::new();
        k6.buy_offer = Some((false, 0));
        k6.killserver=Some(true);
        tx.send((0, k6)).unwrap();
        std::thread::sleep(three_seconds);
    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| shortrec_process(index, x, 1));

    let mut p = Player::new("DefaultPlayer".to_owned());
    p.hand = vec![147, 154, 160, 174, 161];
    let mut unknown: Vec<usize> = vec![141, 148, 7, 177, 70];
    //Test arranged
    p.arranged = vec![(147, false, None, false),
                      (154, false, None, false),
                      (160, false, None, false),
                      (174, false, None, false),
                      (161, false, None, false)];
    p.draft = vec![];
    //Test submit word
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![179, 178, 176, 175, 173, 172, 171],
                                        turn_index: 0,
                                        ticks: None,
                                    })));


    // test notification
    assert_eq!(iter_o.next(),
               Some(ShortRec::PushNotification("Player 1 has formed a word [house]".to_owned())));
    //Test buy card
    p.vp = 3;
    p.coin = 2;
    p.skip_cards = vec![147, 154, 160, 174, 161];
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Buy],
                                        offer_row: vec![179, 178, 176, 175, 173, 172, 171],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.discard = vec![179];
    p.discard.extend(vec![147, 154, 160, 174, 161]);
    redraw(&mut p,&mut unknown,&[1,2,3,4]);
    assert_eq!(iter_o.next(),
               Some(ShortRec::Hand(p.hand.clone())));
    assert_eq!(iter_o.next(), Some(ShortRec::TurnIndex(0)));
    //test give out
 
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![178, 176, 175, 173, 172, 171, 170],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //
    p.arranged = vec![(177, false, None, false)];
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![178, 176, 175, 173, 172, 171, 170],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    assert_eq!(iter_o.next(),
               Some(ShortRec::PushNotification("Player 1 has formed a word [t]".to_owned())));
    p.coin += 1;
    p.skip_cards.push(177);
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Buy],
                                        offer_row: vec![178, 176, 175, 173, 172, 171, 170],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.discard.push(178);
    p.discard.extend(p.hand.clone());
    redraw(&mut p,&mut unknown,&[1,2,3,4]); 
    assert_eq!(5,p.hand.clone().len());                           
    assert_eq!(iter_o.next(),
               Some(ShortRec::Hand(p.hand.clone())));
    assert_eq!(iter_o.next(), Some(ShortRec::TurnIndex(0)));
    assert_eq!(iter_o.next(),
    Some(ShortRec::Board(BoardCodec {
                            players: vec![p.clone()],
                            gamestates: vec![GameState::TurnToSubmit],
                            offer_row: vec![176, 175, 173, 172, 171, 170,169],
                            turn_index: 0,
                            ticks: None,
                        })));
    p.arranged = vec![(154, false, None, false)];
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![176, 175, 173, 172, 171, 170,169],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    assert_eq!(iter_o.next(),
               Some(ShortRec::PushNotification("Player 1 has formed a word [o]".to_owned())));
    p.vp+=1;
    p.skip_cards.push(154);
     assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Buy],
                                        offer_row: vec![176, 175, 173, 172, 171, 170,169],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.discard.extend(p.hand.clone());
    redraw(&mut p,&mut unknown,&[1,2,3,4]);
    assert_eq!(5,p.hand.clone().len());                           
    assert_eq!(iter_o.next(),
               Some(ShortRec::Hand(p.hand.clone())));
    assert_eq!(iter_o.next(), Some(ShortRec::TurnIndex(0)));
}
