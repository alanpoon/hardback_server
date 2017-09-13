#[allow(non_snake_case)]
pub mod table;
pub use self::table::Table;
use game::Connection;
use std::collections::HashMap;
#[derive(Clone)]
pub struct Lobby {
    pub tables: HashMap<String, Table>,
    pub connections: HashMap<String, Connection>,
}
impl Lobby {
    pub fn new() -> Self {
        Lobby {
            tables: HashMap::new(),
            connections: HashMap::new(),
        }
    }
    pub fn make_table(&mut self, player: Connection) {
        let new_table = Table::new(player.clone());
        self.tables.insert(player.addr, new_table);
    }
    pub fn remove_table(&mut self, table: Table) {
        self.tables.remove(&table.host);
        for (addr, _) in table.players {
            self.connections.remove(&addr);
        }
    }
    pub fn remove_connection(&mut self, player: Connection) {
        self.connections.remove(&player.addr);
    }
    pub fn add_connection(&mut self, player: Connection) {
        self.connections.insert(player.addr.clone(), player);
    }
    pub fn send_chat(&mut self, chat: String) {
        for (_, con) in &self.connections {
            con.send_chat(chat.clone());
        }
    }
    pub fn get_tables(&self) -> &HashMap<String, Table> {
        &self.tables
    }
}
