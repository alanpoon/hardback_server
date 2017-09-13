use game::Connection;
use lobby::Lobby;
use std::collections::HashMap;
use server_lib::json_gen::*;
#[derive(Clone)]
pub struct Table {
    pub players: HashMap<String, Connection>,
    pub numberOfPlayers: usize,
    pub host: String,
}
impl Table {
    pub fn new(player: Connection) -> Table {
        let mut player_hash = HashMap::new();
        player_hash.insert(player.addr.clone(), player.clone());
        Table {
            players: player_hash,
            numberOfPlayers: 3,
            host: player.addr.clone(),
        }
    }
    pub fn start_game(&mut self, lobby: &mut Lobby) {
        self.unready_player();
        lobby.remove_table(self.clone());

    }
    pub fn add_player(&mut self, lobby: &mut Lobby, mut player: Connection) -> bool {
        self.unready_player();
        if self.players.len() < self.numberOfPlayers {
            self.players.insert(player.addr.clone(), player.clone());
            if let Some(mut x) = player.get_table() {
                x.remove_player(lobby, &mut player);
            }
            player.set_table(Some(self.clone()));
            true
        } else {
            false
        }
    }
    pub fn remove_player(&mut self, lobby: &mut Lobby, player: &mut Connection) {
        self.players.remove(&player.addr);
        player.set_table(None);
        if self.players.len() == 0 {
            lobby.remove_table(self.clone());
        }
        self.unready_player();
    }
    pub fn get_number_of_players(&self) -> usize {
        self.numberOfPlayers
    }
    pub fn get_players(&self) -> &HashMap<String, Connection> {
        &self.players
    }
    pub fn set_number_of_players(&mut self, n: usize) {
        if (n >= 2) & (n <= 5) & (n >= self.players.len()) {
            self.numberOfPlayers = n;
            self.unready_player();
        }
    }
    fn unready_player(&mut self) {
        for _player in self.players.values_mut() {
            _player.set_ready(false);
        }
    }
    pub fn send_chat(&self, chat: String) {
        for (_, ref _player) in &self.players {
            _player.send_chat(chat.clone());
        }
    }
    pub fn to_json(&self) -> TableInfo {
        let play_iter = self.players.iter();
        let play_v = play_iter.map(|(_, con)| con.name.clone()).collect();
        TableInfo {
            numberOfPlayers: self.numberOfPlayers,
            players: play_v,
        }
    }
}
