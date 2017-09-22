use game_logic::game_engine::*;
use server_lib::codec::*;
use std;
use std::sync::mpsc;
use websocket::message::OwnedMessage;
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
fn arrange_card() {
    let (tx, rx) = mpsc::channel();
    let (con_tx, con_rx) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let connections = vec![Connection {
                               name: "DefaultPlayer".to_owned(),
                               player_num: Some(0),
                               sender: con_tx,
                           }];
    println!("aaa");

    std::thread::spawn(|| { GameEngine::new(vec![p], connections).run(rx); });
    let mut k = GameCommand::new();
    k.arranged = Some(vec![145, 152, 158, 172, 159]);
    k.killserver = Some(true);
    tx.send((0, k)).unwrap();

    while let Ok(_) = con_rx.recv() {
        println!("zz");
        //println!("rec..{:?}",_str);
    }

}
