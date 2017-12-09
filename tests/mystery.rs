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
use hardback_server::drafttest::{ShortRec, TheMysteryDraftStruct};

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
fn arrange_mystery_card() {
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
                                                                     TheMysteryDraftStruct {},
                                                                     &mut log);
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(76, false, Some("h".to_owned()), false),
                                (83, false, Some("o".to_owned()), false),
                                (89, false, Some("u".to_owned()), false),
                                (87, false, None, false),
                                (73, false, Some("e".to_owned()), false)]);
        /*
                        purchase             giveable                genre giveable      trash
                (89,"u",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery,  gen: uncover adjacent
            b.uncover_adjacent(p,c,w);
            
        }))),
                (87,"s",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, Non-gen:Lockup offer rowcard
            b.lockup_offer(p,c,w);
        })),None),
                (73,"e",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen:uncover adjacent wild
            b.uncover_adjacent(p,c,w);
        }))),
        */
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);

        //assert 2(board update) + assert 3, receive the request whether to lockupcar
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);

        //assert 4, tell the server that you want to lockup card
        let mut k3 = GameCommand::new();
        k3.reply = Some(0);
        tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);

        //assert 5, receive the board update after declaring the card to lockup
        let mut k4 = GameCommand::new();
        k4.lockup = Some((true, 0));
        k4.killserver = Some(true);
        tx.send((0, k4)).unwrap();
        std::thread::sleep(three_seconds);

    });

    let mut iter_o = con_rx.iter().enumerate().map(|(index, x)| {
        let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, request, turn_index, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate:{:?}", index + 1);
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
    p.coin = 10;
    p.arranged = vec![(76, false, Some("h".to_owned()), false),
                      (83, false, Some("o".to_owned()), false),
                      (89, false, Some("u".to_owned()), false),
                      (87, false, None, false),
                      (73, false, Some("e".to_owned()), false)];
    p.hand = vec![76, 83, 89, 87, 73];
    p.draft = vec![];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));


    p.coin += 1;
    p.skip_cards.push(87);
    //assert 2
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //assert 3
    assert_eq!(iter_o.next(),
               Some(ShortRec::Request((0,
                                       87,
                                       "Do you want to lock up any offer row card?".to_owned(),
                                       vec!["Yes".to_owned(), "No".to_owned()],
                                       None))));

    //assert 4
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::LockUp],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    p.lockup.push(26);
    //assert 5
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone()],
                                        gamestates: vec![GameState::Buy],
                                        offer_row: vec![23, 38, 80, 94, 98, 119, 1],
                                        turn_index: 0,
                                        ticks: None,
                                    })));

}
