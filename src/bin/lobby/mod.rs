#[allow(non_snake_case)]
pub mod table;
pub use self::table::Table;
use game::Connection;
use std::collections::HashMap;
use websocket::message::OwnedMessage;
use futures::{Future, Sink};
use game_logic::board::BoardStruct;
use server_lib::codec::*;
use server_lib::cards;
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
                     tables: &mut HashMap<i32, Table>,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180]) {

        if let OwnedMessage::Text(z) = msg {
            match ServerReceivedMsg::deserialize_receive(&z) {
                Ok(ServerReceivedMsg { gamecommand,
                                       newTable,
                                       ready,
                                       joinTable,
                                       changePlayers,
                                       leaveTable,
                                       joinLobby,
                                       namechange,
                                       chat,
                                       location }) => {
                    if let Some(Some(_)) = newTable {
                        let con_c = self.clone();
                        if let Some(con) = con_c.connections.get(&addr) {
                            self.make_table(con.clone(), tables);
                        }
                    } else if let Some(Some(_ready)) = ready {
                        let mut tl = None;
                        let mut number_of_player = 0;
                        if let Some(con) = self.connections.get_mut(&addr) {
                            tl = con.table;
                            number_of_player = con.number_of_player;
                            con.ready = _ready;
                        }
                        if _ready {
                            let iter_lobby = self.connections.iter();
                            if let Some(table_n) = tl {
                                let mut vec_z = vec![];
                                if iter_lobby.filter(|&(_, c)| {
                                                         vec_z.push(c.clone());
                                                         (c.table == tl)
                                                     })
                                       .filter(|&(_, c)| c.ready == false)
                                       .count() == 0 {
                                    tables.insert(table_n, Table::new(vec_z, number_of_player));
                                    if let Some(t) = tables.get_mut(&table_n) {
                                        t.start_game(cardmeta);
                                    }


                                }
                            }


                        }
                    } else if let Some(Some(_joinTable)) = joinTable {
                        if let Some(con) = self.connections.get_mut(&addr) {
                            con.table = Some(_joinTable);
                        }
                    } else if let Some(Some(_changePlayers)) = changePlayers {
                        let mut tl = None;
                        if let Some(con) = self.connections.get(&addr) {
                            tl = con.table;
                        }
                        let iter_lobby = self.connections.iter_mut();
                        iter_lobby.filter(|&(_, ref c)| c.table == tl)
                            .map(|(_, c)| c.number_of_player = _changePlayers.clone())
                            .collect::<Vec<()>>();
                    } else if let Some(Some(_leaveTable)) = leaveTable {
                        if let Some(con) = self.connections.get_mut(&addr) {
                            con.table = None;
                            con.player_num = None;
                            con.number_of_player = 3;
                        }

                    } else if let Some(Some(_namechange)) = namechange {
                        if let Some(con) = self.connections.get_mut(&addr) {
                            con.name = _namechange;
                        }
                    }
                    if let (Some(Some(_chat)), Some(Some(_location))) = (chat, location) {
                        let mut table_n = None;
                        let mut sender_n = "defaultname";
                        if let Some(con) = self.connections.get(&addr) {
                            table_n = con.table;
                            sender_n = &con.name
                        }
                        let g = json!({
                            "chat": _chat.clone(),
                            "location":_location.clone(),
                            "type_name":"chat",
                            "sender":sender_n
                            });
                        let iter_lobby = self.connections.iter();
                        iter_lobby.filter(|&(_, c)| if _location == "lobby" {
                                              c.table == None
                                          } else {
                                              c.table == table_n
                                          })
                            .map(|(_, c)| {
                                println!("uii{}", c.addr);
                                c.sender
                                    .clone()
                                    .send(OwnedMessage::Text(g.to_string()))
                                    .wait()
                                    .unwrap();
                            })
                            .collect::<Vec<()>>();

                    } else if let Some(Some(_gamecommand)) = gamecommand {
                        if let Some(con) = self.connections.get_mut(&addr) {
                            if let (Some(table_num), Some(_player_num)) =
                                (con.table, con.player_num) {
                                if let Some(t) = tables.get_mut(&table_num) {
                                    if let Some(ref txx) = t.tx {
                                        txx.send((_player_num as i32, _gamecommand)).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("e{:?}", e);
                }
            }

        }
    }
}
