use std::sync::mpsc;
use server_lib::codec::*;
use server_lib::cards;
use lobby::table::Table;
use game_logic::board::BoardStruct;
pub struct GameEngine {}
impl GameEngine {
    pub fn new(players: Vec<Player>) -> Self {
        GameEngine {}
    }
    pub fn run(&mut self,
               rx: mpsc::Receiver<(i32, GameCommand)>,
               cardmeta: &[cards::ListCard<BoardStruct>; 180]) {

    }
}
