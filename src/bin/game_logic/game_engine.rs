use std::sync::mpsc;
use server_lib::codec::*;
use server_lib::cards;
use websocket::message::OwnedMessage;
use futures::{Future, Sink};
use game_logic::board::BoardStruct;
use game::Connection;
use rand::Rng;
use rand;
use std;
pub enum GameState {
    SubmitWord,
    Buy,
    DrawCard,
    Spell(Action),
}
pub enum Action {
    UseInk(usize),
}

pub struct GameEngine {
    players: Vec<Player>,
    cardmeta: [cards::ListCard<BoardStruct>; 180],
    connections: Vec<Connection>,
    gamestates: Vec<GameState>,
}
impl GameEngine {
    pub fn new(players: Vec<Player>, connections: Vec<Connection>) -> Self {
        let mut gamestates_v = vec![];
        for _ in &players {
            gamestates_v.push(GameState::DrawCard);
        }
        GameEngine {
            players: players,
            cardmeta: cards::populate::<BoardStruct>(),
            connections: connections,
            gamestates: gamestates_v,
        }
    }
    pub fn run(&mut self, rx: mpsc::Receiver<(usize, GameCommand)>) {
        let mut last_update = std::time::Instant::now();
        let turn_index = 0;
        give_outstarting(&mut self.players, &self.cardmeta);

        'game: loop {
            let sixteen_ms = std::time::Duration::from_millis(1000);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);

            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }
            let mut temp_players = vec![];
            for mut it in self.players.iter_mut().zip(self.gamestates.iter_mut()) {
                let (ref mut _p, ref mut game_state) = it;
                //((x,y), z)
                match game_state {
                    &mut &mut GameState::DrawCard => {
                        for _ in 0usize..5 - _p.hand.len() {
                            if let Some(n) = _p.draft.pop() {
                                _p.hand.push(n);
                            } else {
                                let mut rng = rand::thread_rng();
                                _p.draft = _p.discard.clone();
                                _p.discard = vec![];
                                rng.shuffle(&mut _p.draft);
                                if let Some(n) = _p.draft.pop() {
                                    _p.hand.push(n);
                                }
                            }
                        }
                        _p.arranged = vec![];
                        _p.wild = vec![];
                        _p.inked_cards = vec![];
                    }
                    _ => {}
                }
                temp_players.push(_p.clone());
            }
            for it in self.connections.iter() {
                let ref con = it;
                let k: Result<BoardCodec, String> =
                    Ok(BoardCodec { players: temp_players.clone() });
                let g = json!({
                        "boardstate":k
                    });
                con.sender
                    .clone()
                    .send(OwnedMessage::Text(g.to_string()))
                    .wait()
                    .unwrap();
            }

            while let Ok((player_id, game_command)) = rx.recv() {
                match self.gamestates.get(player_id) {
                    Some(&GameState::Spell(ref a)) => {
                        if let (GameCommand { use_ink, use_remover, .. },
                                Some(ref mut _p),
                                Some(ref con)) =
                            (game_command,
                             self.players.get_mut(player_id),
                             self.connections.get(player_id)) {
                            if let Some(z) = use_ink {
                                _p.inked_cards.push(z);
                            } else if let Some(z) = use_remover {
                                if _p.inked_cards.contains(&z) {
                                    _p.hand.push(_p.inked_cards.remove(z));
                                } else {
                                    let k: Result<BoardCodec, String> =
                                        Err("cannot remove a card that is not inked".to_owned());
                                    let g = json!({
                        "boardstate":k
                    });
                                    con.sender
                                        .clone()
                                        .send(OwnedMessage::Text(g.to_string()))
                                        .wait()
                                        .unwrap();

                                }
                            }
                        }
                    }
                    Some(&GameState::SubmitWord) => {}
                    Some(&GameState::Buy) => {}
                    _ => {}
                }
            }
            last_update = std::time::Instant::now();
        }
    }
}
pub fn give_outstarting(players: &mut Vec<Player>,
                        cardmeta: &[cards::ListCard<BoardStruct>; 180]) {
    let mut remaining_deck = vec![];
    for _p in players {
        _p.starting::<BoardStruct>(cardmeta, &mut remaining_deck);
    }
}
