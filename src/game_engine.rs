use std::sync::mpsc;
use json_gen::*;
pub struct GameEngine {}
impl GameEngine {
    pub fn new() -> Self {
        GameEngine {}
    }
    pub fn run(&mut self, rx: mpsc::Receiver<(i32, GameCommand)>) {}
}
