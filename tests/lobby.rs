extern crate websocket;
extern crate futures;
extern crate tokio_core;
extern crate rust_wordnik;
extern crate rand;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
pub extern crate hardback_codec;
pub extern crate hardback_server;
pub use hardback_codec as codec_lib;

use codec_lib::codec::*;
use hardback_server::lobby::{game, handler};
use websocket::message::OwnedMessage;
use websocket::ClientBuilder;
use tokio_core::reactor::Core;
use futures::sync::mpsc;
use futures::{Sink, Future, Stream};
use std::error::Error;
//use hardback_server::testdraft::{ShortRec};

const CONNECTION_SERVER: &'static str = "127.0.0.1:8080";
const CONNECTION_CLIENT: &'static str = "ws://127.0.0.1:8080";
#[derive(Clone)]
pub struct Connection {
    pub name: String,
    pub player_num: Option<usize>,
    pub sender: mpsc::Sender<OwnedMessage>,
}

impl Connection {
    fn tx_send(&self, msg: ServerReceivedMsg) {
        self.sender
            .clone()
            .send(OwnedMessage::Text(ServerReceivedMsg::serialize_send(msg).unwrap()))
            .wait()
            .unwrap();
    }
}
pub fn run_owned_message(con: &'static str,
                         gui: std::sync::mpsc::Sender<OwnedMessage>,
                         rx: mpsc::Receiver<OwnedMessage>)
                         -> Result<(), ConnectionError> {
    println!("run");
    let gui_c = gui.clone();
    match ClientBuilder::new(con) {
        Ok(_c) => {
            let mut core = Core::new().unwrap();
            let runner = ClientBuilder::new(con)
            .unwrap()
            .add_protocol("rust-websocket")
            .async_connect_insecure(&core.handle())
            .and_then(move |(duplex, _)| {
                let (to_server, from_server) = duplex.split();
                let reader = from_server.for_each(move |msg| {
                    // ... convert it to a string for display in the GUI...
                    let _content = match msg {
                        OwnedMessage::Close(e) => Some(OwnedMessage::Close(e)),
                        OwnedMessage::Ping(d) => Some(OwnedMessage::Ping(d)),
                        OwnedMessage::Text(f) => {
                            gui_c.send(OwnedMessage::Text(f)).unwrap();
                            None
                        }
                        _ => None,
                    };
                    // ... and send that string _to_ the GUI.

                    Ok(())
                });
                let writer = rx
            .map_err(|()| unreachable!("rx can't fail"))
            .fold(to_server, |to_server, msg| {
                let h= msg.clone();
                 to_server.send(h)
            })
            .map(|_| ());

                // Use select to allow either the reading or writing half dropping to drop the other
                // half. The `map` and `map_err` here effectively force this drop.
                reader.select(writer).map(|_| ()).map_err(|(err, _)| err)
            });
            match core.run(runner) {
                Ok(_) => {
                    println!("_connected");
                    let g = serde_json::to_string(&ConnectionStatus::Ok).unwrap();
                    gui.clone().send(OwnedMessage::Text(g)).unwrap();
                    Ok(())
                }
                Err(_er) => {
                    println!("_not_connected");
                    let g = serde_json::to_string(&ConnectionStatus::Error(ConnectionError::CannotFindServer)).unwrap();
                    gui.clone().send(OwnedMessage::Text(g)).unwrap();
                    Err(ConnectionError::CannotFindServer)
                }
            }
        }
        Err(er) => {
            gui.clone().send(OwnedMessage::Text(er.clone().description().to_owned())).unwrap();
            Err(ConnectionError::InvalidDestination)
        }
    }

}
#[derive(Debug,PartialEq,Clone)]
pub enum ShortRec {
    Board(BoardCodec),
    Request((usize, usize, String, Vec<String>, Option<u16>)),
    TurnIndex(usize),
    PlayerIndex(usize),
    Tables,
    ConnectionOk,
    None,
}
#[test]
fn lobby() {
    let (game_tx, game_rx) = std::sync::mpsc::channel();
    let (proxy_tx, proxy_rx) = std::sync::mpsc::channel();
    let (_futures_tx, futures_rx) = mpsc::channel(3);

    std::thread::spawn(move || {
                           println!("running handler");
                           handler::run(CONNECTION_SERVER, game_tx);
                       });
    std::thread::spawn(move || { game::run(game_rx); });

    std::thread::spawn(move || {
        let mut _connected = false;
        match run_owned_message(CONNECTION_CLIENT, proxy_tx.clone(), futures_rx) {
            Ok(_) => _connected = true,
            Err(_err) => {
                println!("reconnecting");
                _connected = false;
            }
        }
        //sleep if not connection is drop
        let ten_seconds = std::time::Duration::new(10, 0);
        std::thread::sleep(ten_seconds);
    });
    std::thread::spawn(move || {
        let _con = Connection {
            name: "defaultplayer".to_owned(),
            player_num: Some(0),
            sender: _futures_tx,
        };
        let three_seconds = std::time::Duration::new(3, 0);
        let ten_seconds = std::time::Duration::new(10, 0);
        std::thread::sleep(three_seconds);
        let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
        h.set_new_table(true);
        _con.tx_send(h);
        //sleep if not connection is drop
        std::thread::sleep(ten_seconds);
    });

    let mut iter_o = proxy_rx.iter().enumerate().map(|(index, x)| {
        let mut y = ShortRec::None;
        if let OwnedMessage::Text(z) = x {
            println!("z {:?}", z.clone());
            if let Ok(ClientReceivedMsg { boardstate,
                                          request,
                                          turn_index,
                                          player_index,
                                          type_name,
                                          tables,
                                          tablenumber,
                                          connection_status,
                                          .. }) = ClientReceivedMsg::deserialize_receive(&z) {
                println!("iterenumerate:{:?}", index + 1);
                if let Some(Some(Ok(_boardstate))) = boardstate {
                    y = ShortRec::Board(_boardstate);
                } else if let Some(Some(_request)) = request {
                    y = ShortRec::Request(_request);
                } else if let Some(Some(_turn_index)) = turn_index {
                    y = ShortRec::TurnIndex(_turn_index);
                } else if let Some(Some(_player_index)) = player_index {
                    y = ShortRec::PlayerIndex(_player_index);
                } else if let (Some(Some(_type_name)), Some(Some(_tables)), Some(_tablenumber)) =
                    (type_name, tables, tablenumber) {
                    if _type_name == "lobby" {
                        y = ShortRec::Tables;
                    }
                } else if let Some(Some(_connection_status)) = connection_status {
                    if let ConnectionStatus::Ok = _connection_status {
                        y = ShortRec::ConnectionOk;
                    }
                }
            }
        }
        y
    });
    assert_eq!(iter_o.next(), Some(ShortRec::ConnectionOk));
    assert_eq!(iter_o.next(), Some(ShortRec::Tables));
    /*assert_eq!(iter_o.next(),
            Some(ShortRec::Board(BoardCodec {
                                    players: vec![p.clone()],
                                    gamestates: vec![GameState::TurnToSubmit],
                                    offer_row: vec![26, 23, 38, 80, 94, 98, 119],
                                    turn_index: 0,
                                    ticks: None,
                                })));
      */

}
