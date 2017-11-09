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
use websocket::ClientBuilder;
use codec_lib::codec::*;
use codec_lib::cards;
use codec_lib::cards::*;
use hardback_server::game_logic::game_engine::*;
use hardback_server::game_logic::board::BoardStruct;
use hardback_server::game_logic;
use hardback_server::lobby::{game, table, handler};
use std::sync::mpsc;
use websocket::message::OwnedMessage;
use websocket::ClientBuilder;
use tokio_core::reactor::Core;
use hardback_server::testdraft::TheStartingDraftStruct;
use futures::sync::mpsc;
const CONNECTION: &'static str = "127.0.0.1:8080";

#[derive(Clone)]
pub struct Connection {
    pub name: String,
    pub player_num: Option<usize>,
    pub sender: mpsc::Sender<OwnedMessage>,
}
#[derive(Serialize,Deserialize)]
pub enum ConnectionError {
    NotConnectedToInternet,
    CannotFindServer,
    InvalidDestination,
}
#[derive(Serialize,Deserialize)]
#[serde(tag = "connection_status", content = "c")]
pub enum ConnectionStatus {
    Error(ConnectionError),
    Ok,
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
    request((usize, usize, String, Vec<String>, Option<u16>)), //player_id,card_id,
    turn_index(usize),
    player_index(usize),
    None,
}
#[test]
fn lobby() {
    let (game_tx, game_rx) = std::sync::mpsc::channel();
    let (proxy_tx, proxy_rx) = std::sync::mpsc::channel();
    let (futures_tx, futures_rx) = mpsc::channel(3);
    std::thread::spawn(move || {
                           println!("running handler");
                           handler::run(CONNECTION, game_tx);
                       });
    std::thread::spawn(move || { game::run(game_rx); });

    let mut connected = false;
    match run_owned_message(CONNECTION, proxy_tx.clone(), futures_rx) {
        Ok(_) => connected = true,
        Err(err) => {
            println!("reconnecting");
            connected = false;
        }
    }


    loop {
        while let Ok(s) = proxy_rx.try_recv() {
            println!("s: {:?}", s);
        }
    }
}
pub fn run_owned_message(con: &'static str,
                         gui: std::sync::mpsc::Sender<OwnedMessage>,
                         rx: mpsc::Receiver<OwnedMessage>)
                         -> Result<(), ConnectionError> {
    println!("run");
    let gui_c = gui.clone();
    match ClientBuilder::new(con) {
        Ok(c) => {
            let mut core = Core::new().unwrap();
            let runner = ClientBuilder::new(con)
            .unwrap()
            .add_protocol("rust-websocket")
            .async_connect_insecure(&core.handle())
            .and_then(move |(duplex, _)| {
                let (to_server, from_server) = duplex.split();
                let reader = from_server.for_each(move |msg| {
                    // ... convert it to a string for display in the GUI...
                    let content = match msg {
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
                    println!("connected");
                    let g = serde_json::to_string(&ConnectionStatus::Ok).unwrap();
                    gui.clone().send(OwnedMessage::Text(g)).unwrap();
                    Ok(())
                }
                Err(_er) => {
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
