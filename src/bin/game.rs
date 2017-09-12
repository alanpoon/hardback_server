use futures::sync::mpsc;
use futures::sync::oneshot;
use websocket::message::OwnedMessage;
use std::collections::HashMap;
use lobby::Lobby;
use hardback_server::RealDecisionMaker;
use std;
use std::net::SocketAddr;
pub enum Game_Rx_Type {
    Sender(SocketAddr, mpsc::Sender<OwnedMessage>),
    Message(SocketAddr, OwnedMessage),
}
pub fn run(game_rx: std::sync::mpsc::Receiver<Game_Rx_Type>) {
    let mut server_data = ServerData::new();
    loop {
        while let Ok(Game_Rx_Type::Sender(addr, _sender)) = game_rx.try_recv() {
            let j = format!("{}", addr);
            let con = Connection::new(_sender);
            server_data.connections.insert(j, con);
        }
    }
}
pub struct ServerData {
    connections: HashMap<String, Connection>,
    lobbies: HashMap<String, Lobby>,
}
impl ServerData {
    pub fn new() -> Self {
        ServerData {
            connections: HashMap::new(),
            lobbies: HashMap::new(),
        }

    }
}
pub struct Connection {
    name: String,
    table: Option<i32>,
    ready: bool,
    decider: Option<RealDecisionMaker>,
    sender: mpsc::Sender<OwnedMessage>,
}
impl Connection {
    pub fn new(sender: mpsc::Sender<OwnedMessage>) -> Connection {
        Connection {
            name: "defaultname".to_owned(),
            table: None,
            ready: false,
            decider: None,
            sender: sender,
        }
    }

    pub fn sendLobby(&self) {}
}
