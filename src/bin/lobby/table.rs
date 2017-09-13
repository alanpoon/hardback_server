use game::Connection;
use lobby::Lobby;
use std::collections::HashMap;
use server_lib::json_gen::*;

pub struct Table {
    pub players: Vec<String>,
    pub numberOfPlayer: i32,
}
impl Table {
    pub fn new(playername: String) -> Table {
        Table {
            players: vec![playername],
            numberOfPlayer: 3,
        }
    }
}
