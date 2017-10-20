#[allow(non_snake_case)]
pub mod table;
pub use self::table::Table;
use game::Connection;
use std::collections::HashMap;
use websocket::message::OwnedMessage;
use futures::{Future, Sink};
use codec_lib::codec::*;
use codec_lib::cards;
use itertools::Itertools;
#[derive(Clone)]
pub struct Lobby {
    pub connections: HashMap<String, Connection>,
    pub table_index: usize,
}
impl Lobby {
    pub fn new() -> Self {
        Lobby {
            connections: HashMap::new(),
            table_index: 0,
        }
    }
    pub fn make_table(&mut self, player: Connection) {
        let table_n = self.table_index;
        if let Some(mut c) = self.connections.get_mut(&player.addr) {
            c.table = Some(table_n);
            c.player_num = Some(0);
        }
        let mut temp_tables = vec![];
        for (key, mut group) in &(self.connections.clone()).into_iter().group_by(|elt| {
                                                                                     (*elt).1.table
                                                                                 }) {
            if let (Some(t_i), false) =
                (key,
                 group.nth(0)
                     .unwrap()
                     .1
                     .game_started) {
                temp_tables.push(t_i);
            }
        }
        for _i in 0..self.table_index {
            if !temp_tables.contains(&_i) {
                self.table_index = _i;
            } else {
                self.table_index += 1;
            }
        }

        self.broadcast_tableinfo();
    }
    pub fn remove_table(&mut self, table_num: usize) {
        self.connections
            .iter_mut()
            .filter(|&(_, ref con)| con.table == Some(table_num))
            .map(|(_, con)| {
                     con.table = None;
                     con.player_num = None;
                 })
            .collect::<Vec<()>>();

        self.broadcast_tableinfo();
    }
    pub fn remove_connection(&mut self, addr: String) {
        self.connections.remove(&addr);
        self.broadcast_tableinfo();
    }
    pub fn add_connection(&mut self, player: Connection) {
        self.connections.insert(player.addr.clone(), player);
    }
    pub fn broadcast_tableinfo(&self) {
        println!("{:?}", self.connections.clone());
        let mut table_infos: Vec<TableInfo> = vec![];
        for (key, mut group) in &(self.connections.clone()).into_iter().group_by(|elt| {
                                                                                     (*elt).1.table
                                                                                 }) {
            // Check that the sum of each group is +/- 4.

            let mut number_of_player = 3;
            let mut game_started = false;
            let mut player_vec = vec![];

            for _g in group {
                number_of_player = _g.1.number_of_player;
                game_started = _g.1.game_started;
                player_vec.push(_g.1.name.clone());
            }
            if let (Some(table_num), false) = (key, game_started) {
                let t = TableInfo::new(player_vec, number_of_player);
                table_infos.push(t);
            }

        }

        self.connections
            .iter()
            .filter(|&(_, ref con)| con.game_started == false)
            .map(|(_, con)| {
                let g = json!({
                            "tables": table_infos,
                            "type":"lobby",
                            "tablenumber":con.table,
                            "sender":con.name,
                            });
                con.sender
                    .clone()
                    .send(OwnedMessage::Text(g.to_string()))
                    .wait()
                    .unwrap();
            })
            .collect::<Vec<()>>();
    }
    pub fn from_json(&mut self,
                     addr: String,
                     msg: OwnedMessage,
                     tables: &mut HashMap<usize, Table>) {

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
                                       message,
                                       location }) => {
                    if let Some(Some(_)) = newTable {
                        let con_c = self.clone();
                        if let Some(con) = con_c.connections.get(&addr) {
                            self.make_table(con.clone());
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
                            if let Some(table_n) = tl {
                                println!("tl");
                                let mut vec_z = vec![];
                                let mut game_can_be_started = false;
                                if self.connections
                                       .iter()
                                       .filter(|&(_, c)| {
                                                   vec_z.push(c.clone());
                                                   (c.table == tl)
                                               })
                                       .filter(|&(_, c)| c.ready == false)
                                       .count() == 0 {
                                    tables.insert(table_n, Table::new(vec_z, number_of_player));
                                    if let Some(t) = tables.get_mut(&table_n) {
                                        t.start_game();
                                        game_can_be_started = true;
                                    }
                                }
                                if game_can_be_started {
                                    self.connections
                                        .iter_mut()
                                        .filter(|&(_, ref c)| c.table == tl)
                                        .map(|(_, c)| c.game_started = true)
                                        .collect::<Vec<()>>();
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
                    if let (Some(Some(_chat)), Some(Some(_location))) = (message, location) {
                        let mut table_n = None;
                        let mut sender_n = "defaultname";
                        if let Some(con) = self.connections.get(&addr) {
                            table_n = con.table;
                            sender_n = &con.name
                        }
                        let g = json!({
                            "message": _chat.clone(),
                            "location":_location.clone(),
                            "type":"chat",
                            "sender":sender_n
                            });
                        let iter_lobby = self.connections.iter();
                        iter_lobby.filter(|&(_, c)| if _location == "lobby" {
                                              c.game_started == false
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
                            if let (Some(table_num), Some(_player_num), true) =
                                (con.table, con.player_num, con.game_started) {
                                if let Some(t) = tables.get_mut(&table_num) {
                                    if let Some(ref txx) = t.tx {
                                        txx.send((_player_num, _gamecommand)).unwrap();
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
