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
use hardback_server::drafttest::TheNotifyDraftStruct;
use rand::{Rng, SeedableRng, StdRng};
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
enum ShortRec {
    Board(BoardCodec),
    Request((usize, usize, String, Vec<String>, Option<u16>)), //player_id,card_id,
    TurnIndex(usize),
    PlayerIndex(usize),
    None,
}
#[test]
fn notifydraft() {
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
                                                                     TheNotifyDraftStruct {},
                                                                     &mut log);
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 3
        let mut k1 = GameCommand::new();
        k1.reply = Some(0);
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 4
        let mut k2 = GameCommand::new();
        k2.reply = Some(0);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| {
        let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, request, turn_index, player_index, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate:{:?}", index);
                if let Some(Some(Ok(_boardstate))) = boardstate {
                    y = ShortRec::Board(_boardstate);
                } else if let Some(Some(_request)) = request {
                    y = ShortRec::Request(_request);
                } else if let Some(Some(_turn_index)) = turn_index {
                    y = ShortRec::TurnIndex(_turn_index);
                } else if let Some(Some(_player_index)) = player_index {
                    y = ShortRec::PlayerIndex(_player_index);
                }

            }
        }
        y
    });
    //assert 1
    assert_eq!(iter_o.next(), Some(ShortRec::PlayerIndex(0)));
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.arranged = vec![];
    p.draft = vec![141, 148, 7, 177, 70, 90, 14, 20, 18, 4];
    //assert 2
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::ShowDraft],
                                        offer_row: vec![],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    rng.shuffle(&mut p.draft);
    let vecdraft = p.draft.split_off(5);
    p.hand = vecdraft;
    p.draft = vec![];
    //assert 3
    assert_eq!(iter_o.next(),
        Some(ShortRec::Request((0,0,
                                "Let's Start! You shuffle all 10 cards and draw 5 cards into your hand. It is your turn to submit word.".to_owned(),
                                vec!["Continue".to_owned()],None))));

    //assert 4
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Shuffle],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
                                       /*
    //assert 4
     assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,0,
                                       "It is your turn to submit word.".to_owned(),
                                       vec!["Continue".to_owned()],None))));
    assert_eq!(iter_o.next(),
            Some(ShortRec::Board(BoardCodec {
                                    players: vec![p.clone()],
                                    gamestates: vec![GameState::TurnToSubmit],
                                    offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                    turn_index: 0,
                                    ticks: None,
                                })));
                                */
}
