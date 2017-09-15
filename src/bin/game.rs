use futures::sync::mpsc;
use futures::{Future, Sink};
use websocket::message::OwnedMessage;
use lobby::{Table, Lobby};
use server_lib::RealDecisionMaker;
use std;
use std::collections::HashMap;
use server_lib::codec::*;
pub enum GameRxType {
    Sender(String, mpsc::Sender<OwnedMessage>),
    Message(String, OwnedMessage),
}
pub fn run(game_rx: std::sync::mpsc::Receiver<GameRxType>) {
    let mut lobby = Lobby::new();
    let mut tables = HashMap::new();
    let mut last_update = std::time::Instant::now();
    loop {
        let sixteen_ms = std::time::Duration::from_millis(16);
        let now = std::time::Instant::now();
        let duration_since_last_update = now.duration_since(last_update);

        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }
        while let Ok(GameRxType::Sender(addr, _sender)) = game_rx.try_recv() {
            let con = Connection::new(_sender, addr.clone());
            lobby.connections.insert(addr, con);
        }
        while let Ok(GameRxType::Message(addr, msg)) = game_rx.try_recv() {
            lobby.from_json(addr, msg, &mut tables);
        }
        last_update = std::time::Instant::now();
        println!("connections len:{:?}", lobby.connections.len());
    }
}
#[derive(Clone)]
pub struct Connection {
    pub addr: String,
    pub name: String,
    pub table: Option<i32>,
    pub player_num: Option<usize>,
    pub ready: bool,
    pub decider: Option<RealDecisionMaker>,
    pub sender: mpsc::Sender<OwnedMessage>,
}
impl Connection {
    pub fn new(sender: mpsc::Sender<OwnedMessage>, addr: String) -> Connection {
        Connection {
            addr: addr,
            name: "defaultname".to_owned(),
            table: None,
            player_num: None,
            ready: false,
            decider: None,
            sender: sender,
        }
    }
}
