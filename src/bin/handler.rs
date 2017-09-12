use futures::sync::mpsc;
use futures::{Stream, Future, Sink};
use tokio_core::reactor::Core;
use websocket::message::OwnedMessage;
use websocket::server::InvalidConnection;
use websocket::async::Server;
use websocket::Message;
use std::net::SocketAddr;
use std;
pub fn run(con: &'static str,
           game_rx: std::sync::mpsc::Sender<(SocketAddr,
                                             mpsc::Sender<OwnedMessage>,
                                             Option<OwnedMessage>)>) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // bind to the server
    let server = Server::bind(con, &handle).unwrap();

    // time to build the server's future
    // this will be a struct containing everything the server is going to do

    // a from_client of incoming connections
    let f = server.incoming()
        // we don't wanna save the from_client if it drops
       // .map_err(|InvalidConnection { error, .. }| error)
        .for_each(|(upgrade, addr)| {
            // accept the request to be a ws connection

            let (ch_sender, ch_receiver) = mpsc::channel(2);
              game_rx.send((addr,ch_sender,None)).unwrap();
            let f = upgrade
                .accept()
                .and_then(|(duplex, _)| {
                    // simple echo server impl
                    let (to_client, from_client) = duplex.split();
                    let reader = from_client.for_each(move |msg| {
                    // ... convert it to a string for display in the GUI...
                    let content = match msg {
                        OwnedMessage::Close(e) => Some(Message::from(OwnedMessage::Close(e))),
                        OwnedMessage::Ping(d) => Some(Message::from(OwnedMessage::Ping(d))),
                        OwnedMessage::Text(f) => {
                            game_rx.send((addr,ch_sender.clone(),Some(OwnedMessage::Text(f)))).unwrap();
                            None
                        }
                        _ => None,
                    };
                    // ... and send that string _to_ the GUI.

                    Ok(())
                });
         let writer = ch_receiver.map_err(|()| unreachable!("rx can't fail"))
            .fold(to_client, |to_client, msg| {
                let h= msg.clone();
               // h.add_private(addr);
                 to_client.send(h)
            })
            .map(|_| ());
                reader.select(writer).map(|_| ()).map_err(|(err, _)| err)
                });

	        /*  handle.spawn(f.map_err(move |e| println!("{}: '{:?}'", addr, e))
	                       .map(move |_| println!("{} closed.", addr)));
                           */
         
        });

    core.run(f).unwrap();
}
