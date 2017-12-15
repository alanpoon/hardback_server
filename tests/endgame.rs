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
use hardback_server::drafttest::{ShortRec, TheEndGameDraftStruct, shortrec_process};
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

#[test]
fn endgame() {
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
                                                                         TheEndGameDraftStruct {},
                                                                         &mut log);
                       });

    std::thread::spawn(move || {
        let three_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k1 = GameCommand::new();
        k1.arranged = Some(vec![(143, false, None, false)]);
        _tx.send((0, k1)).unwrap();
        std::thread::sleep(three_seconds);

        //assert 2(board update) + assert 3, receive the request whether to lockupcar
        let mut k2 = GameCommand::new();
        k2.submit_word = Some(true);
        _tx.send((0, k2)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k3 = GameCommand::new();
        k3.buy_offer = Some((false, 0));
        _tx.send((0, k3)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k4 = GameCommand::new();
        k4.arranged = Some(vec![(159, false, None, false)]);
        k4.killserver = Some(true);
        _tx.send((1, k4)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k5 = GameCommand::new();
        k5.submit_word = Some(true);
        _tx.send((1, k5)).unwrap();
        std::thread::sleep(three_seconds);
        let mut k6 = GameCommand::new();
        k6.buy_offer = Some((false, 0));
        k6.killserver = Some(true);
        _tx.send((1, k6)).unwrap();
        std::thread::sleep(three_seconds);

    });
    let mut iter_o = con_rx1.iter().enumerate().map(|(index, x)| shortrec_process(index, x, 1));
    let mut iter_o2 = con_rx2.iter().enumerate().map(|(index, x)| shortrec_process(index, x, 2));
    let mut p = Player::new("DefaultPlayer".to_owned());
    p.hand = vec![143, 135, 108, 110, 111];
    p.vp = 59;
    let mut p2 = Player::new("Player 2".to_owned());
    p2.hand = vec![90, 49, 2, 75, 159];

    //Test arranged
    p.arranged = vec![(143, false, None, false)];
    //assert 1
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::TurnToSubmit, GameState::Spell],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));


    assert_eq!(iter_o2.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::TurnToSubmit, GameState::Spell],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));

    p.vp += 1;
    p.skip_cards.push(143);

    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::Buy, GameState::Spell],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));

    assert_eq!(iter_o2.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::Buy, GameState::Spell],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 0,
                                        ticks: None,
                                    })));

    assert_eq!(iter_o.next(),
               Some(ShortRec::PushNotification("Player 1 has reached [60 vp] The game will end in this round.".to_owned())));
    assert_eq!(iter_o2.next(),
               Some(ShortRec::PushNotification("Player 1 has reached [60 vp] The game will end in this round.".to_owned())));

    assert_eq!(iter_o.next(),
               Some(ShortRec::Hand(vec![70, 177, 7, 148, 141])));
    assert_eq!(iter_o.next(), Some(ShortRec::TurnIndex(1)));
    assert_eq!(iter_o2.next(), Some(ShortRec::TurnIndex(1)));

    p.hand = vec![70, 177, 7, 148, 141];
    p.arranged = vec![];
    p.skip_cards = vec![];

    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::Spell, GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 1,
                                        ticks: None,
                                    })));
    assert_eq!(iter_o2.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::Spell, GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 1,
                                        ticks: None,
                                    })));
    p2.arranged = vec![(159, false, None, false)];
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::Spell, GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 1,
                                        ticks: None,
                                    })));
    assert_eq!(iter_o2.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::Spell, GameState::TurnToSubmit],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 1,
                                        ticks: None,
                                    })));
    p2.coin += 1;
    p2.skip_cards.push(159);
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::Spell, GameState::Buy],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 1,
                                        ticks: None,
                                    })));
    assert_eq!(iter_o2.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::Spell, GameState::Buy],
                                        offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                        turn_index: 1,
                                        ticks: None,
                                    })));
    p.hand = vec![];
    p.arranged = vec![];
    p.draftlen = 0;
    p2.hand = vec![];
    p2.arranged = vec![];
    p2.draftlen = 0;
    p2.skip_cards = vec![];
    p2.ink = p2.coin;
    p2.coin = 0;
    assert_eq!(iter_o.next(),
               Some(ShortRec::Board(BoardCodec {
                                        players: vec![p.clone(), p2.clone()],
                                        gamestates: vec![GameState::ShowResult(0),
                                                         GameState::ShowResult(0)],
                                        offer_row: vec![],
                                        turn_index: 1,
                                        ticks: None,
                                    })));

}
