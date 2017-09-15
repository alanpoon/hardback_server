use std::sync::mpsc;
use server_lib::codec::*;
pub struct GameEngine {}
impl GameEngine {
    pub fn new() -> Self {
        GameEngine {}
    }
    pub fn run(&mut self, rx: mpsc::Receiver<(i32, GameCommand)>) {}
}
