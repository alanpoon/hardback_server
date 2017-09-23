use game_logic::game_engine::*;
use server_lib::codec::*;
use server_lib::cards;
use server_lib::cards::*;
use game_logic::board::BoardStruct;
use game_logic;
use std;
use std::sync::mpsc;
use websocket::message::OwnedMessage;
#[derive(Clone)]
pub struct Connection {
    pub name: String,
    pub player_num: Option<usize>,
    pub sender: mpsc::Sender<OwnedMessage>,
}
impl GameCon for Connection {
    fn tx_send(&self, msg: OwnedMessage) {
        self.sender
            .clone()
            .send(msg)
            .unwrap();
    }
}

#[test]
fn check_cardmeta() {
    let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
    for _k in cardmeta.iter().enumerate() {
        assert_eq!(_k.0, _k.1.id);
    }
}
#[test]
fn player_starting() {
    let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
    let mut remaining_deck = vec![];
    let mut _p = Player::new("defaultname".to_owned());
    game_logic::draw_card::player_starting::<BoardStruct>(&mut _p, &cardmeta, &mut remaining_deck);
    assert_eq!(_p.draft.len(), 5);
    assert_eq!(_p.hand.len(), 5);
}

#[test]
fn arrange_card() {
    let (tx, rx) = mpsc::channel();
    let (con_tx, con_rx) = mpsc::channel();
    let p = Player::new("DefaultPlayer".to_owned());
    let connections = vec![Connection {
                               name: "DefaultPlayer".to_owned(),
                               player_num: Some(0),
                               sender: con_tx,
                           }];
    std::thread::spawn(|| { GameEngine::new(vec![p], connections).run(rx); });
    std::thread::spawn(move || {
        let five_seconds = std::time::Duration::new(3, 0);
        //assert 1
        let mut k = GameCommand::new();
        k.arranged = Some(vec![(147, None), (154, None), (160, None), (174, None), (161, None)]);
        //  k.killserver = Some(true);
        tx.send((0, k)).unwrap();
        std::thread::sleep(five_seconds);
        //assert 2
        println!("k1");
        let mut k1 = GameCommand::new();
        k1.submit_word = Some(true);
        tx.send((0, k1)).unwrap();
        std::thread::sleep(five_seconds);
    });

    let mut c = 0;
    let mut iter_o = con_rx.iter().map(|x| {
        println!("con_rx_c {}", c);
        c += 1;
        let mut y = None;
        if let OwnedMessage::Text(z) = x {
            if let Ok(ClientReceivedMsg { boardstate, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                y = Some(boardstate.unwrap().unwrap().unwrap());
            }
        }
        y
    });
    let h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
    let mut p = Player::new("DefaultPlayer".to_owned());
    //Test arranged
    p.arranged = vec![(147, None), (154, None), (160, None), (174, None), (161, None)];
    p.hand = vec![147, 154, 160, 174, 161];
    p.draft = vec![141, 148, 150, 177, 70];

    assert_eq!(iter_o.next(),
               Some(Some(BoardCodec {
                             players: vec![p.clone()],
                             gamestates: vec![GameState::TurnToSubmit],
                         })));
    //Test submit word
    p.vp = 3;
    p.coin = 2;
    assert_eq!(iter_o.next(),
               Some(Some(BoardCodec {
                             players: vec![p],
                             gamestates: vec![GameState::Buy],
                         })));

}
