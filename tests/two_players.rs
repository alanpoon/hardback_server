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
use hardback_server::drafttest::TheTwoPlayerDraftStruct;
use std::collections::HashMap;
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
fn two_players() {
    let (_tx, rx) = mpsc::channel();
    let (con_tx1, con_rx1) = mpsc::channel();
    let (con_tx2, con_rx2) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let p2 = Player::new("Player 2".to_owned());
    let connections: HashMap<usize, Connection> = [(0,
                                                    Connection {
                                                        name: "DefaultPlayer".to_owned(),
                                                        player_num: Some(0),
                                                        sender: con_tx1,
                                                    }),
                                                   (1,
                                                    Connection {
                                                        name: "Player 2".to_owned(),
                                                        player_num: Some(1),
                                                        sender: con_tx2,
                                                    })]
            .iter()
            .cloned()
            .collect();
    std::thread::spawn(|| {
                           let mut log: Vec<ClientReceivedMsg> = vec![];
                           GameEngine::new(vec![p, p2], connections).run(rx,
                                                                         TheTwoPlayerDraftStruct {},
                                                                         &mut log);
                       });


    let mut iter_o = con_rx1.iter().enumerate().map(|(index, x)| {
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

    assert_eq!(iter_o.next(), Some(ShortRec::PlayerIndex(0)));
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.arranged = vec![];
    p.hand = vec![105, 135, 108, 110, 111];
    p.draft = vec![141, 148, 7, 177, 70];
    let mut p2 = Player::new("Player 2".to_owned());
    p2.hand = vec![90, 49, 2, 75, 77];
    p2.draft = vec![84, 130, 12, 34, 91];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::ShowDraft,
                                                         GameState::ShowDraft],
                                        offer_row: vec![],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    let mut iter_o2 = con_rx2.iter().enumerate().map(|(index, x)| {
        let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, request, turn_index, player_index, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate2:{:?}", index);
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
    assert_eq!(iter_o2.next(), Some(ShortRec::PlayerIndex(1)));

    assert_eq!(iter_o2.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::ShowDraft,
                                                         GameState::ShowDraft],
                                        offer_row: vec![],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
}
