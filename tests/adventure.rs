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
        println!("msg:{:?}", msg.clone());
        self.sender
            .clone()
            .send(msg)
            .unwrap();
    }
}
#[derive(Debug,PartialEq,Clone)]
enum ShortRec {
    board(BoardCodec),
    request((String, Vec<String>)),
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

        //assert 4
        let mut k3 = GameCommand::new();
        k3.reply = Some(0);
        k3.killserver = Some(true);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);
        /*
        //assert 3 //say yes to discard card for benefit, send this to client
        let mut k3 = GameCommand::new();
        k3.buyoffer = Some(0);
         k3.killserver = Some(true);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);
        //assert 4: choose card to discard, 0:No,1:Yes,
        let mut k4 = GameCommand::new();
        k4.reply = Some(0);
        tx.send((0, k4)).unwrap();
        std::thread::sleep(three_seconds);
        */
    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| {
        let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, request, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate:{:?}", index);
                if let Some(Some(Ok(_boardstate))) = boardstate {
                    y = ShortRec::board(_boardstate);
                } else if let Some(Some(_request)) = request {
                    y = ShortRec::request(_request);
                }
            }
        }
        println!("what is y{:?}", y.clone());
        y
    });
    let h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.arranged = vec![(7, Some("h".to_owned())),
                      (14, Some("o".to_owned())),
                      (20, Some("u".to_owned())),
                      (18, None),
                      (4, None)];
    p.hand = vec![7, 14, 20, 18, 4];
    p.draft = vec![141, 148, 150, 177, 70];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                    })));

    //Test submit word, you can trash cards for benefit
    p.vp = 5;
    //assert 2
    assert_eq!(iter_o.next(),
               Some(ShortRec::board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                    })));
    //assert 3
    assert_eq!(iter_o.next(),
               Some(ShortRec::request(("Do you want to trash this card for the benefit?"
                                           .to_owned(),
                                       vec!["yes".to_owned(), "no".to_owned()]))));
    //assert 4
    assert_eq!(iter_o.next(),
               Some(ShortRec::request(("Do you want to trash this card for the benefit?"
                                           .to_owned(),
                                       vec!["yes".to_owned(), "no".to_owned()]))));

    /*
    p.arranged = vec![(7, Some("h".to_owned())),
                      (14, Some("o".to_owned())),
                      (20, Some("u".to_owned())),
                      (4, None)];
    p.hand = vec![7, 14, 20, 4];
    //assert 3 
    assert_eq!(iter_o.next(),
               Some(ShortRec::request(("Do yo want to trash this card for the benefit?".to_owned(),vec!["yes".to_owned(),"no".to_owned()]))));
                         */
    /*  p.discard = vec![26];
    //Test buy card
    assert_eq!(iter_o.next(),
               Some(Some(BoardCodec {
                             players: vec![p.clone()],
                             gamestates: vec![GameState::DrawCard],
                             offer_row: vec![ 23, 38, 80, 94, 98, 119,1],
                         })));
        */
}
