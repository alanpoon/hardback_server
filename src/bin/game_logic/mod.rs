pub mod game_engine;
pub use self::game_engine::GameEngine;
pub mod wordapi;
use server_lib::cards::Card;
pub struct Player{
    hands:Vec<Card>
}
pub struct SharedData{
    players:Vec<Player>
}