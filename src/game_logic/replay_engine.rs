use std::sync::mpsc;
use codec_lib::codec::*;
use codec_lib::cards;
use game_logic::board::BoardStruct;
use std::collections::HashMap;
use std;
pub trait GameConReplay {
    fn tx_send(&self, ClientReceivedMsg);
}
pub struct GameEngine<T: GameConReplay> {
    players: Vec<Player>,
    connections: HashMap<usize, T>,
    gamestates: Vec<GameState>,
    turn_index: usize,
}

impl<T> GameEngine<T>
    where T: GameConReplay
{
    pub fn new(log: &Vec<ClientReceivedMsg>, connections: HashMap<usize, T>) -> Self {
        //init
        let mut g = GameEngine {
            players: vec![],
            connections: HashMap::new(),
            gamestates: vec![],
            turn_index: 0,
        };
        if let Some(&ClientReceivedMsg { ref boardstate, .. }) = log.first() {
            if let &Some(Some(Ok(ref _boardstate))) = boardstate {
                let b = _boardstate.clone();
                g = GameEngine {
                    players: b.players,
                    connections: connections,
                    gamestates: b.gamestates,
                    turn_index: b.turn_index,
                }
            }
        }

        g
    }
    pub fn run(&mut self,
               rx: mpsc::Receiver<Replay>, //is_play,ticks
               log: &Vec<ClientReceivedMsg>) {
        let last_update = std::time::Instant::now();
        let _cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
        let mut ticks: u16 = 0;
        let mut log_iter = log.iter();
        log_iter.next();
        let mut is_play = true;
        'game: loop {
            let sixteen_ms = std::time::Duration::new(1, 0);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);

            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }
            if let Some(&ClientReceivedMsg { ref boardstate, ref request, .. }) =
                log_iter.clone().next() {
                if let &Some(Some(Ok(ref _boardstate))) = boardstate {
                    if Some(ticks) == _boardstate.ticks {
                        if let Some(ref z) = log_iter.next() {
                            broadcast::<T>(z.clone(), &self.connections);
                        }
                    }
                } else if let &Some(Some((_, _, _, _, ref _ticks))) = request {
                    if _ticks == &Some(ticks) {
                        if let Some(ref z) = log_iter.next() {
                            broadcast::<T>(z.clone(), &self.connections);
                        }
                    }
                }
            }
            while let Ok(_replay) = rx.try_recv() {
                match _replay {
                    Replay::Play(_ticks) => {
                        is_play = true;
                        ticks = _ticks;
                    }
                    Replay::Pause(_ticks) => {
                        is_play = false;
                    }
                    Replay::Exit => {
                        break 'game;
                    }
                }
            }
            if is_play {
                ticks += 1;
            }

        }
    }
}
pub fn broadcast<T: GameConReplay>(msg: &ClientReceivedMsg, connections: &HashMap<usize, T>) {
    for (_, _con) in connections {
        _con.tx_send(msg.clone());
    }
}
