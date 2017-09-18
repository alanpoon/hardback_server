use game::Connection;
use lobby::Lobby;
use std::sync::mpsc;
use server_lib::codec::*;
use server_lib::cards;
use game_logic::GameEngine;
use game_logic::board::BoardStruct;
use std;

pub struct Table {
    pub players: Vec<Connection>,
    pub numberOfPlayer: usize,
    pub tx: Option<mpsc::Sender<(i32, GameCommand)>>,
}
impl Table {
    pub fn new(players: Vec<Connection>, numberOfPlayer: usize) -> Table {
        Table {
            players: players,
            numberOfPlayer: numberOfPlayer,
            tx: None,
        }
    }
    pub fn start_game(&mut self, cardmeta: &[cards::ListCard<BoardStruct>; 180]) {
        let (tx, rx) = mpsc::channel();
        self.tx = Some(tx);
        let mut player_vec = vec![];
        for _p in self.players {
            let p = Player::new(_p.name.clone());
            player_vec.push(p);
        }
        std::thread::spawn(|| { GameEngine::new(player_vec).run(rx, &cardmeta); });
    }
}
