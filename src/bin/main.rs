extern crate websocket;
extern crate futures;
extern crate tokio_core;
extern crate rust_wordnik;
pub extern crate hardback_server;

mod handler;
pub mod game;
#[allow(non_snake_case)]
pub mod lobby;
pub mod game_logic;
pub use hardback_server as server_lib;
const CONNECTION: &'static str = "127.0.0.1:8080";
fn main() {

    let (game_tx, game_rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
                           println!("running handler");
                           handler::run(CONNECTION, game_tx);
                       });
    game::run(game_rx);

}
