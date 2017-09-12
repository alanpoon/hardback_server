extern crate websocket;
extern crate futures;
extern crate tokio_core;
extern crate hardback_server;

use websocket::message::OwnedMessage;
use websocket::server::InvalidConnection;
use websocket::async::Server;

use tokio_core::reactor::Core;
use futures::{Future, Sink, Stream};
use futures::sync::mpsc;
mod handler;
pub mod game;
pub mod lobby;
const CONNECTION: &'static str = "127.0.0.1:8080";
fn main() {

    let (game_tx, game_rx) = std::sync::mpsc::channel();
    game::run(game_rx);
    std::thread::spawn(move || { handler::run(CONNECTION, game_tx); });
}
