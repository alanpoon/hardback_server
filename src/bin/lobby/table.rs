use game::Connection;
use lobby::Lobby;
use std::sync::mpsc;
use server_lib::json_gen::*;
use server_lib::game_engine::GameEngine;
use std;
pub struct Table {
    pub players: Vec<Connection>,
    pub numberOfPlayer: i32,
    pub tx: Option<mpsc::Sender<(i32, GameCommand)>>,
}
impl Table {
    pub fn new(player: Connection) -> Table {
        Table {
            players: vec![player],
            numberOfPlayer: 3,
            tx: None,
        }
    }
    pub fn start_game(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.tx = Some(tx);
        std::thread::spawn(|| { GameEngine::new().run(rx); });
    }
}
