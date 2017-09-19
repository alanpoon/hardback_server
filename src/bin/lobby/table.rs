use game::Connection;
use std::sync::mpsc;
use server_lib::codec::*;
use game_logic::GameEngine;
use std;

pub struct Table {
    pub players: Vec<Connection>,
    pub numberOfPlayer: usize,
    pub tx: Option<mpsc::Sender<(usize, GameCommand)>>,
}
impl Table {
    pub fn new(players: Vec<Connection>, numberOfPlayer: usize) -> Table {
        Table {
            players: players,
            numberOfPlayer: numberOfPlayer,
            tx: None,
        }
    }
    pub fn start_game(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.tx = Some(tx);
        let mut player_vec = vec![];
        for _p in &self.players {
            let p = Player::new(_p.name.clone());
            player_vec.push(p);
        }
        let connections = (*self).players.clone();
        std::thread::spawn(|| { GameEngine::new(player_vec, connections).run(rx); });
    }
}
