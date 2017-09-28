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
use hardback_server::testdraft::TheMysteryUnCoverDraftStruct;

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
fn arrange_mystery2_card() {
    let (tx, rx) = mpsc::channel();
    let (con_tx, con_rx) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let connections = vec![Connection {
                               name: "DefaultPlayer".to_owned(),
                               player_num: Some(0),
                               sender: con_tx,
                           }];
    std::thread::spawn(|| {
                           GameEngine::new(vec![p], connections)
                               .run(rx, TheMysteryUnCoverDraftStruct {});
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(42, Some("a".to_owned())),
                                (72, None),
                                (178, Some("a".to_owned())),
                                (87, Some("p".to_owned())),
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
        k2.killserver = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 4 + assert 5 feedback
        let mut k3 = GameCommand::new();
        k3.reply = Some(0);
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
    p.coin = 10;
    p.arranged = vec![(42, Some("a".to_owned())),
                      (72, None),
                      (178, Some("a".to_owned())),
                      (87, Some("p".to_owned())),
                      (73, Some("t".to_owned()))];
    p.hand = vec![42, 72, 178, 87, 73];
    p.draft = vec![141, 148, 7, 177, 70];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));
    p.vp += 1;
    p.skip_cards.push(72);
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
               Some(ShortRec::request((72,
                                       "Do you want to uncover adjacent cards?".to_owned(),
                                       vec!["Yes".to_owned(), "No".to_owned()]))));
    //assert 4
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::UncoverAdjacent(Some(1), 72)],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));
    //assert 5
    p.arranged = vec![(42, None),
                      (72, None),
                      (178, None),
                      (87, Some("p".to_owned())),
                      (73, Some("t".to_owned()))];
    p.vp += 2;
    p.coin += 1;
    p.skip_cards.push(42);
    p.skip_cards.push(178);
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));
}
