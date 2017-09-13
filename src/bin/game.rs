use futures::sync::mpsc;
use futures::{Future, Sink};
use websocket::message::OwnedMessage;
use lobby::{Table, Lobby};
use server_lib::RealDecisionMaker;
use std;
use server_lib::json_gen::*;
pub enum GameRxType {
    Sender(String, mpsc::Sender<OwnedMessage>),
    Message(String, OwnedMessage),
}
pub fn run(game_rx: std::sync::mpsc::Receiver<GameRxType>) {
    let mut lobby = Lobby::new();
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
        last_update = std::time::Instant::now();
        println!("connections len:{:?}", lobby.connections.len());
    }
}
#[derive(Clone)]
pub struct Connection {
    pub addr: String,
    pub name: String,
    pub table: Option<Table>,
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
            ready: false,
            decider: None,
            sender: sender,
        }
    }
    pub fn get_table(&self) -> Option<Table> {
        self.table.clone()
    }
    pub fn set_table(&mut self, table: Option<Table>) {
        self.table = table;
    }
    pub fn is_ready(&self) -> bool {
        self.ready
    }
    pub fn set_ready(&mut self, ready: bool) {
        self.ready = ready;
        if ready {
            let mut starting = true;
            if let Some(ref mut t) = (*self).table {
                for (_, con) in t.get_players() {
                    starting = con.is_ready();
                }
                if starting {}
            }
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn send_lobby(&self, chat_str: String) {
        let g = format!("{{chat:{},location:'lobby'}}", chat_str);
        self.sender
            .clone()
            .send(OwnedMessage::Text(g))
            .wait()
            .unwrap();
    }
    pub fn send_chat(&self, chat_str: String) {
        let g = format!("{{chat:{},location:'table'}}", chat_str);
        self.sender
            .clone()
            .send(OwnedMessage::Text(g))
            .wait()
            .unwrap();
    }
}
