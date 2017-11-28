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
use hardback_server::drafttest::{ShortRec, TheOverlayDraftStruct};

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
fn overlay() {
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
                                                                     TheOverlayDraftStruct {},
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

        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 2
        let mut k2 = GameCommand::new();
        k2.take_card_use_ink = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 3 confirm
        let mut k3 = GameCommand::new();
        k3.reply = Some(0);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 2
        let mut k2b = GameCommand::new();
        k2b.take_card_use_ink = Some(true);
        tx.send((0, k2b)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 3 confirm
        let mut k3b = GameCommand::new();
        k3b.reply = Some(0);
        tx.send((0, k3b)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 4
        let mut k4 = GameCommand::new();
        k4.use_remover = Some(vec![141, 148]);
        tx.send((0, k4)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 5 confirm
        let mut k5 = GameCommand::new();
        k5.reply = Some(0);
        tx.send((0, k5)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 5 confirm
        let mut k5b = GameCommand::new();
        k5b.reply = Some(0);
        tx.send((0, k5b)).unwrap();
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
    p.coin = 10;
    p.remover = 2;
    p.hand = vec![105, 135, 108, 110, 111];
    p.draft = vec![141, 148, 7, 177, 70];
    p.ink = 3;
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //Test use_ink
    //assert 2
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,141,
                                       "You need to use this card to form the word. You may not convert this card to wild. If you can't use this card, you may use ink remover to convert this to a wild card."
                                           .to_owned(),
                                       vec!["Continue".to_owned()],None))));
    p.arranged.push((p.draft.remove(0), true, None, false));
    p.ink -= 1;
    //assert 3
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //Test use_ink
    //assert 2b
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,148,
                                       "You need to use this card to form the word. You may not convert this card to wild. If you can't use this card, you may use ink remover to convert this to a wild card."
                                           .to_owned(),
                                       vec!["Continue".to_owned()],None))));
    p.arranged.push((p.draft.remove(0), true, None, false));
    p.ink -= 1;
    //assert 3b
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //Test use_remover
    //assert 4
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,141,
                                       "You may convert this inked card back to a normal card using a remover token. You may add it back to your hand or use it to form word or a wild card."
                                           .to_owned(),
                                       vec!["Continue".to_owned()],None))));
    p.remover -= 1;
    for &mut (ref _ci, ref mut _ink, _, _) in p.arranged.iter_mut() {
        if *_ci == 141 {
            *_ink = false;
        }
    }
    assert_eq!(iter_o.next(),
              Some(ShortRec::Request((0,148,
                                       "You may convert this inked card back to a normal card using a remover token. You may add it back to your hand or use it to form word or a wild card."
                                           .to_owned(),
                                       vec!["Continue".to_owned()],None))));
    //assert 5
    p.remover -= 1;
    for &mut (ref _ci, ref mut _ink, _, _) in p.arranged.iter_mut() {
        if *_ci == 148 {
            *_ink = false;
        }
    }
    //assert 6
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
}
