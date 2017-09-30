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
use hardback_server::testdraft::TheHorrorDraftStruct;

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
fn horror() {
    let (tx, rx) = mpsc::channel();
    let (con_tx, con_rx) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let connections = vec![Connection {
                               name: "DefaultPlayer".to_owned(),
                               player_num: Some(0),
                               sender: con_tx,
                           }];
    std::thread::spawn(|| {
                           GameEngine::new(vec![p], connections).run(rx, TheHorrorDraftStruct {});
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(41, Some("h".to_owned())),
                                (48, Some("o".to_owned())),
                                (54, Some("u".to_owned())),
                                (52, None),
                                (38, None)]);
        /*
                purchase             giveable                genre giveable      trash
        (52,"s",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (38,"e",8,GIVEABLE::NONE,GIVEABLE::COININK(2),GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        */
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 3
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 4 +assert 5
        let mut k3 = GameCommand::new(); //you choose ink over ink remover
        k3.reply = Some(0);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 6
        let mut k4 = GameCommand::new(); //you choose vp over coin for genre benefit
        k4.reply = Some(0);
        k4.killserver = Some(true);
        tx.send((0, k4)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 5
     /*  let mut k3 = GameCommand::new();
       k3.buyoffer = Some((true,0));
       k3.killserver = Some(true);
       tx.send((0,k3)).unwrap();
       std::thread::sleep(three_seconds);
       */
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
    p.coin = 10;
    p.arranged = vec![(41, Some("h".to_owned())), //minus_other_ink
                      (48, Some("o".to_owned())),
                      (54, Some("u".to_owned())),
                      (52, None),
                      (38, None)];
    p.hand = vec![41, 48, 54, 52, 38];
    p.draft = vec![141, 148, 7, 177, 70];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));

    p.vp += 2;
    p.coin += 2;
    p.skip_cards = vec![52, 38];
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
               Some(ShortRec::request((38,
                                       "Choose between".to_owned(),
                                       vec!["Ink".to_owned(), "Ink Remover".to_owned()]))));
    //assert 4
    p.ink += 1;
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::WaitForReply],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));
    //assert 5
    assert_eq!(iter_o.next(),
               Some(ShortRec::request((38,
                                       "Choose between".to_owned(),
                                       vec!["2 vps".to_owned(), "2 coins".to_owned()]))));

    //assert 6
    p.vp += 2;
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Buy],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                    })));

}
