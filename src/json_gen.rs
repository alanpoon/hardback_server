use serde_json;
use serde::{Deserialize, Deserializer};
fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
    where D: Deserializer<'de>,
          T: Deserialize<'de>
{
    Ok(Some(Option::deserialize(deserializer)?))
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableInfo {
    pub numberOfPlayers: i32,
    pub players: Vec<String>,
}
#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct Player {
    name: String,
    cash: i32,
    cars: i32,
    guns: i32,
    keys: i32,
    hearts: i32,
    bottles: i32,
    wrenches: i32,
    holdings: Vec<i32>,
    thugs: Vec<i32>,
    actions: Vec<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameCommand {
    pub words: Vec<i32>,
    pub inks: Vec<i32>,
    pub inkremovers: Vec<i32>,
    pub buy: Option<i32>,
}

CGM_receive_msg!{
    rename:{
    },optional:{
    (gamecommand,set_gamecommand,GameCommand),
   (newTable,set_new_table,bool),
    (ready,set_ready,bool),
    (joinTable,set_join_table,i32),
    (changePlayers,set_change_player,i32),
    (leaveTable,set_leave_table,bool),
    (joinLobby,set_join_lobby,bool),
    (namechange,set_name_change,String),
    (chat,set_chat,String),
    (location,set_location,String),
    },rename_optional:{},else:{}
}
