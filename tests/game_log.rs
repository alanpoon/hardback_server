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
use std::io::prelude::*;
use std::fs::File;
use hardback_server::game_logic::board::BoardStruct;
use hardback_server::game_logic;
use std::sync::mpsc;
use websocket::message::OwnedMessage;
use hardback_server::testdraft::TheRomanceDraftStruct;

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
    board(BoardCodec),
    request((usize, usize, String, Vec<String>, Option<u16>)),
    turn_index(usize),
    None,
}
#[test]
fn game_log() {
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
        GameEngine::new(vec![p], connections).run(rx, TheRomanceDraftStruct {}, &mut log);
        let s = json!({
                          "log": log
                      });
        let mut buffer = File::create("foo.txt").unwrap();

        buffer.write(s.to_string().as_bytes()).unwrap();
        println!("finish");
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
        (105,"z",5,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p,c,w| {
            //rommanc, Non-gen:double adjacent
            b.double_adjacent(p,c,w);
        })),None), 
        (135,"o",8,GIVEABLE::NONE,GIVEABLE::VPCOIN(1,2),GIVEABLE::VPCOIN(1,1),GIVEABLE::NONE,Genre::ROMANCE,true,None,None),
        }))),
        */
        k1.killserver = Some(true);
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);

    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| {
        let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, request, turn_index, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate:{:?}", index + 1);
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
    let mut p = Player::new("DefaultPlayer".to_owned());
    p.coin = 10;
    p.arranged = vec![(105, Some("a".to_owned())),
                      (135, Some("d".to_owned())),
                      (108, Some("a".to_owned())),
                      (110, None),
                      (111, Some("t".to_owned()))];
    p.hand = vec![105, 135, 108, 110, 111];
    p.draft = vec![141, 148, 7, 177, 70];
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: Some(0),
                                    })));
    loop {}
}
