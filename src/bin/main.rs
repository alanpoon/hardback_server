extern crate websocket;
extern crate futures;
extern crate tokio_core;
extern crate rust_wordnik;
extern crate serde_json;
extern crate rand;
pub extern crate hardback_server;
pub extern crate hardback_boardstruct;

#[allow(non_snake_case)]
pub use hardback_boardstruct::codec_lib as codec_lib;
pub use hardback_server as logic_lib;
const CONNECTION: &'static str = "127.0.0.1:8080";

fn main() {

    let (game_tx, game_rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
                           println!("running handler");
                           logic_lib::lobby::handler::run(CONNECTION, game_tx);
                       });
    logic_lib::lobby::game::run(game_rx);

}
