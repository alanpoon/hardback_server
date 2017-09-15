#[allow(non_snake_case)]
pub mod table;
pub use self::table::Table;
use game::Connection;
use std::collections::HashMap;
use websocket::message::OwnedMessage;
use futures::{Future, Sink};
use server_lib::codec::*;
#[derive(Clone)]
pub struct Lobby {
    pub connections: HashMap<String, Connection>,
    pub table_index: i32,
}
impl Lobby {
    pub fn new() -> Self {
        Lobby {
            connections: HashMap::new(),
            table_index: 0,
        }
    }
    pub fn make_table(&mut self, player: Connection, tables: &mut HashMap<i32, Table>) {
        let table_n = self.table_index.clone();
        if let Some(mut c) = self.connections.get_mut(&player.addr) {
            c.table = Some(table_n);
            c.player_num = Some(0);
        }
        let new_table = Table::new(player.clone());
        tables.insert(self.table_index, new_table);
        self.table_index += 1;
    }
    pub fn remove_table(&mut self, table_num: i32, tables: &mut HashMap<i32, Table>) {
        let iter_mut = self.connections.iter_mut();
        iter_mut.filter(|&(_, ref con)| con.table == Some(table_num))
            .map(|(_, con)| {
                     con.table = None;
                     con.player_num = None;
                 })
            .collect::<Vec<()>>();
        tables.remove(&table_num);
    }
    pub fn remove_connection(&mut self, player: Connection) {
        self.connections.remove(&player.addr);
    }
    pub fn add_connection(&mut self, player: Connection) {
        self.connections.insert(player.addr.clone(), player);
    }

    pub fn from_json(&mut self,
                     addr: String,
                     msg: OwnedMessage,
                     tables: &mut HashMap<i32, Table>) {
        if let OwnedMessage::Text(z) = msg {
            if let Ok(ServerReceivedMsg { gamecommand,
                                    newTable,
                                    ready,
                                    joinTable,
                                    changePlayers,
                                    leaveTable,
                                    joinLobby,
                                    namechange,
                                    chat,
                                    location }) = ServerReceivedMsg::deserialize_receive(&z) {

                if let Some(Some(_)) = newTable {
                    let con_c = self.clone();
                    if let Some(con) = con_c.connections.get(&addr) {
                        self.make_table(con.clone(), tables);
                    }
                } else if let Some(Some(_ready)) = ready {
                    let selfc = self.clone();
                    if let Some(con) = self.connections.get_mut(&addr) {
                        con.ready = _ready;
                        if _ready {
                            let iter_lobby = selfc.connections.iter();
                            if iter_lobby.filter(|&(_, c)| {
                                                     (c.table == con.table) & (c.ready == false)
                                                 })
                                   .count() == 0 {
                                if let Some(table_n) = con.table {
                                    if let Some(t) = tables.get_mut(&table_n) {
                                        t.start_game();
                                    }
                                }

                            }
                        }
                    }
                } else if let Some(Some(_joinTable)) = joinTable {
                    if let Some(con) = self.connections.get_mut(&addr) {
                        con.table = Some(_joinTable);
                        if let Some(t) = tables.get_mut(&_joinTable) {
                            con.player_num = Some(t.players.len() - 1);
                            t.players.push(con.clone());
                        }
                    }
                } else if let Some(Some(_changePlayers)) = changePlayers {
                    if let Some(con) = self.connections.get(&addr) {
                        if let Some(table_num) = con.table {
                            if let Some(t) = tables.get_mut(&table_num) {
                                t.numberOfPlayer = _changePlayers;
                            }
                        }
                    }
                } else if let Some(Some(_leaveTable)) = leaveTable {
                    if let Some(con) = self.connections.get_mut(&addr) {
                        if let (Some(table_num), Some(player_num)) = (con.table, con.player_num) {
                            if let Some(t) = tables.get_mut(&table_num) {
                                t.players.remove(player_num);
                            }
                        }
                        con.table = None;
                        con.player_num = None;

                    }

                } else if let Some(Some(_namechange)) = namechange {
                    if let Some(con) = self.connections.get_mut(&addr) {
                        con.name = _namechange.clone();
                        if let (Some(table_num), Some(player_num)) = (con.table, con.player_num) {
                            if let Some(t) = tables.get_mut(&table_num) {
                                t.players[player_num].name = _namechange;
                            }
                        }
                    }
                } else if let (Some(Some(_chat)), Some(Some(_location))) = (chat, location) {
                    if let Some(con) = self.connections.get(&addr) {
                        let iter_lobby = (*self).connections.iter();
                        iter_lobby.filter(|&(_, c)| if _location == "lobby" {
                                              c.table == None
                                          } else {
                                              c.table == con.table
                                          })
                            .map(|(_, c)| {
                                c.sender
                                    .clone()
                                    .send(OwnedMessage::Text(_chat.clone()))
                                    .wait()
                                    .unwrap();
                            })
                            .collect::<Vec<()>>();
                    }

                } else if let Some(Some(_gamecommand)) = gamecommand {
                    if let Some(con) = self.connections.get_mut(&addr) {
                        if let (Some(table_num), Some(_player_num)) = (con.table, con.player_num) {
                            if let Some(t) = tables.get_mut(&table_num) {
                                if let Some(ref txx) = t.tx {
                                    txx.send((_player_num as i32, _gamecommand)).unwrap();
                                }
                            }
                        }
                    }
                }
            }

        }
    }
}
