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
    pub numberOfPlayers: usize,
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
