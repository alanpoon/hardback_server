use futures::sync::mpsc;
use futures::{Sink, Future};
use websocket::message::OwnedMessage;
use lobby::Lobby;
use codec_lib::RealDecisionMaker;
use codec_lib::codec::*;
use game_logic::game_engine::GameCon;
use std;
use std::fmt;
use std::collections::HashMap;
pub enum GameRxType {
    Sender(String, mpsc::Sender<OwnedMessage>),
    Message(String, OwnedMessage),
    Close(String),
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
        while let Ok(z) = game_rx.try_recv() {
            println!("zz,");
            match z {
                GameRxType::Sender(addr, _sender) => {
                    let con = Connection::new(_sender, addr.clone());
                    println!("found connection");
                    let mut j = vec![];
                    let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
                    //notify connection is successfull
                    h.set_connection_status(ConnectionStatus::Ok);
                    con.tx_send(h, &mut j);
                    lobby.connections.insert(addr, con);
                }
                GameRxType::Message(addr, msg) => {
                    lobby.from_json(addr, msg, &mut tables);
                }
                GameRxType::Close(addr) => {
                    lobby.remove_connection(addr);
                }
            }

        }

        last_update = std::time::Instant::now();
    }
}
#[derive(Clone)]
pub struct Connection {
    pub addr: String,
    pub name: String,
    pub table: Option<usize>,
    pub game_started: bool,
    pub player_num: Option<usize>,
    pub number_of_player: usize,
    pub ready: bool,
    pub decider: Option<RealDecisionMaker>,
    pub sender: mpsc::Sender<OwnedMessage>,
}
impl GameCon for Connection {
    fn tx_send(&self, msg: ClientReceivedMsg, log: &mut Vec<ClientReceivedMsg>) {
        self.sender
            .clone()
            .send(OwnedMessage::Text(ClientReceivedMsg::serialize_send(msg).unwrap()))
            .wait()
            .unwrap();
    }
}
impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Connection {{ table: {:?}, game_started: {},name:{},number_of_player:{},ready:{},player_num:{:?} }}",
               self.table,
               self.game_started,
               self.name,
               self.number_of_player,
               self.ready,
               self.player_num)
    }
}
impl Connection {
    pub fn new(sender: mpsc::Sender<OwnedMessage>, addr: String) -> Connection {
        Connection {
            addr: addr,
            name: "defaultname".to_owned(),
            table: None,
            game_started: false,
            player_num: None,
            number_of_player: 3,
            ready: false,
            decider: None,
            sender: sender,
        }
    }
}
