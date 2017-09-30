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
use hardback_server::testdraft::TheNormalDraftStruct;

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
    _normal_draft.player_starting(&mut _p, &cardmeta, &mut remaining_deck);
    assert_eq!(_p.draft.len(), 5);
    assert_eq!(_p.hand.len(), 5);
}

#[test]
fn arrange_normal_card() {
    let (tx, rx) = mpsc::channel();
    let (con_tx, con_rx) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let connections = vec![Connection {
                               name: "DefaultPlayer".to_owned(),
                               player_num: Some(0),
                               sender: con_tx,
                           }];
    std::thread::spawn(|| {
                           GameEngine::new(vec![p], connections).run(rx, TheNormalDraftStruct {});
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(147, None), (154, None), (160, None), (174, None), (161, None)]);
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
        //assert 4
        let mut k4 = GameCommand::new();
        k4.buy_offer = Some((true, 0));
        tx.send((0, k4)).unwrap();
        std::thread::sleep(three_seconds);
    });

    let mut iter_o = con_rx.iter().map(|x| {
        let mut y = None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                y = Some(boardstate.unwrap().unwrap().unwrap());
            }
        }
        y
    });
    let h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.arranged = vec![(147, None), (154, None), (160, None), (174, None), (161, None)];
    p.hand = vec![147, 154, 160, 174, 161];
    p.draft = vec![141, 148, 150, 177, 70];

    assert_eq!(iter_o.next(),
               Some(Some(BoardCodec {
                             players: vec![p.clone()],
                             gamestates: vec![GameState::TurnToSubmit],
                             offer_row: vec![179, 178, 176, 175, 173, 172, 171],
                             turn_index: 0,
                         })));
    //Test submit word
    p.vp = 3;
    p.coin = 2;
    assert_eq!(iter_o.next(),
               Some(Some(BoardCodec {
                             players: vec![p.clone()],
                             gamestates: vec![GameState::Buy],
                             offer_row: vec![179, 178, 176, 175, 173, 172, 171],
                             turn_index: 0,
                         })));
    p.discard = vec![179];
    //Test buy card
    assert_eq!(iter_o.next(),
               Some(Some(BoardCodec {
                             players: vec![p.clone()],
                             gamestates: vec![GameState::DrawCard],
                             offer_row: vec![178, 176, 175, 173, 172, 171, 170],
                             turn_index: 0,
                         })));

}
