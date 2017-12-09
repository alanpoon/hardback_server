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
use hardback_server::drafttest::TheTimelessDraftStruct;

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
fn timeless2player() {
    let (tx, rx) = mpsc::channel();
    let (con_tx1, con_rx1) = mpsc::channel();
    let (con_tx2, con_rx2) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let p2 = Player::new("Player 2".to_owned());
    let connections = vec![Connection {
                               name: "DefaultPlayer".to_owned(),
                               player_num: Some(0),
                               sender: con_tx1,
                           },
                           Connection {
                               name: "Player 2".to_owned(),
                               player_num: Some(1),
                               sender: con_tx2,
                           }];
    std::thread::spawn(|| {
                           let mut log: Vec<ClientReceivedMsg> = vec![];
                           GameEngine::new(vec![p, p2], connections).run(rx,
                                                                         TheTimelessDraftStruct {},
                                                                         &mut log);
                       });
    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(101, false, None, true),
                                (135, false, Some("a".to_owned()), true),
                                (110, false, Some("a".to_owned()), false),
                                (124, false, Some("a".to_owned()), false)]);
        /*
       (101,"t",8,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,true,None,Some(Box::new(|ref mut b, p,c,w| {
            //mystery, gen:lockup offer row
            b.lockup_offer(p,c,w);
        }))),
     */
        tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);

        //assert 2(board update) + assert 3, receive the request whether to lockupcar
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        k2.killserver = Some(true);
        tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);

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

    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.arranged = vec![(101, false, None, true),
                      (135, false, Some("a".to_owned()), true),
                      (110, false, Some("a".to_owned()), false),
                      (124, false, Some("a".to_owned()), false)];
    p.coin = 10;
    p.hand = vec![105, 99, 108, 110, 124];
    p.timeless_classic = vec![136, 96, 135];
    p.draft = vec![];
    p.skip_cards = vec![];
    let mut p2 = Player::new("Player 2".to_owned());
    p2.hand = vec![90, 49, 2, 75, 77];
    p2.draft = vec![];
    p2.timeless_classic = vec![101];
    p2.coin = 10;
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::TurnToSubmit, GameState::Spell],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));
    //assert 2
    //101,other timeless not added to benefit
    p.skip_cards = vec![];
    p2.discard.push(101);
    p2.timeless_classic = vec![];
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::Buy, GameState::Spell],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));

}
