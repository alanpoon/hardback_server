use game_logic::game_engine::*;
use server_lib::codec::*;
use server_lib::cards;
use server_lib::cards::*;
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
pub struct BoardStruct {}
impl Board for BoardStruct {
    fn two_cent_per_adv(&mut self, player_id: usize, card_id: usize) {}
    fn minus_other_ink(&mut self, player_id: usize, card_id: usize) {}
    fn lockup_offer(&mut self, player_id: usize, card_id: usize) {}
    fn uncover_adjacent(&mut self, player_id: usize, card_id: usize) {}
    fn double_adjacent(&mut self, player_id: usize, card_id: usize) {}
    fn trash_other(&mut self, player_id: usize, card_id: usize) {}
    fn one_vp_per_wild(&mut self, player_id: usize, card_id: usize) {}
    fn keep_or_discard_three(&mut self, player_id: usize, card_id: usize) {}
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
    println!("aaa");

    std::thread::spawn(|| { GameEngine::new(vec![p], connections).run(rx); });
    let mut k = GameCommand::new();
    k.arranged = Some(vec![145, 152, 158, 172, 159]);
    k.killserver = Some(true);
    tx.send((0, k)).unwrap();
    /*
    while let Ok(OwnedMessage::Text(st)) = con_rx.recv() {
        println!("zz {:?}",st.clone());
        let h = ClientReceivedMsg::deserialize_receive(&st).unwrap();

    }
    */
    let mut iter_o = con_rx.iter().map(|x| {
        let mut y = None;
        println!("0");
        if let OwnedMessage::Text(z) = x {
            println!("1");
            if let Ok(ClientReceivedMsg { boardstate, .. }) =
                ClientReceivedMsg::deserialize_receive(&z) {
                println!("2 {:?}", boardstate);
                y = Some(boardstate.unwrap().unwrap().unwrap());
            }
        }
        y
    });
    let h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
    let mut p = Player::new("DefaultPlayer".to_owned());
    p.arranged = vec![145, 152, 158, 172, 159];
    p.hand = vec![145, 152, 158, 172, 159];
    p.draft = vec![141, 148, 150, 177, 70];
    assert_eq!(iter_o.next(), Some(Some(BoardCodec { players: vec![p] })));

}
