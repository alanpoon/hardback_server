use std::sync::mpsc;
use server_lib::codec::*;
use server_lib::cards;
use websocket::message::OwnedMessage;
use futures::{Future, Sink};
use game_logic::board::BoardStruct;
use game_logic::wordapi;
use game::Connection;
use rand::Rng;
use rand;
use std;
pub enum GameState {
    SubmitWord,
    Buy,
    DrawCard,
    Spell(Action),
    ResolveOption(Vec<Option<usize>>),
}
pub enum Action {
    UseInk(usize),
}

pub struct GameEngine {
    players: Vec<Player>,
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
            connections: connections,
            gamestates: gamestates_v,
        }
    }
    pub fn run(&mut self, rx: mpsc::Receiver<(usize, GameCommand)>) {
        let mut last_update = std::time::Instant::now();
        let turn_index = 0;
        let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
        give_outstarting(&mut self.players, &cardmeta);

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
                    &mut &mut GameState::ResolveOption(ref _v) => for _c in _v {},
                    _ => {}
                }
                temp_players.push(_p.clone());
            }
            for it in self.connections.iter() {
                let ref con = it;
                let k: Result<BoardCodec, String> =
                    Ok(BoardCodec { players: temp_players.clone() });
                let g = json!({
                                  "boardstate": k
                              });
                con.sender
                    .clone()
                    .send(OwnedMessage::Text(g.to_string()))
                    .wait()
                    .unwrap();
            }

            while let Ok((player_id, game_command)) = rx.recv() {
                let mut need_update = false;
                let mut temp_board = BoardStruct::new(self.players.clone());
                if let (GameCommand { use_ink,
                                      use_remover,
                                      ref arranged,
                                      ref wild,
                                      submit_word,
                                      .. },
                        Some(ref mut _p),
                        Some(ref con),
                        Some(ref mut _gamestate)) =
                    (game_command,
                     self.players.get_mut(player_id),
                     self.connections.get(player_id),
                     self.gamestates.get_mut(player_id)) {
                    match _gamestate {
                        &mut &mut GameState::Spell(ref a) => {
                            if let Some(z) = use_ink {
                                _p.inked_cards.push(z);
                            } else if let Some(z) = use_remover {
                                if _p.inked_cards.contains(&z) {
                                    _p.hand.push(_p.inked_cards.remove(z));
                                } else {
                                    let k: Result<BoardCodec, String> =
                                        Err("cannot remove a card that is not inked".to_owned());
                                    let g = json!({
                                                      "boardstate": k
                                                  });
                                    con.sender
                                        .clone()
                                        .send(OwnedMessage::Text(g.to_string()))
                                        .wait()
                                        .unwrap();

                                }
                            } else if let &Some(ref z) = arranged {
                                _p.arranged = z.to_vec();
                            } else if let &Some((card_index, ref replacement)) = wild {
                                let mut wild_vec = vec![];
                                for _c in &_p.arranged {
                                    if *_c == card_index {
                                        wild_vec.push(Some(replacement.clone()));
                                    } else {
                                        wild_vec.push(None);
                                    }
                                }
                                _p.wild = wild_vec;
                            }
                        }
                        &mut &mut GameState::SubmitWord => {
                            //uses tempboard
                            if let Some(true) = submit_word {
                                let mut word = "".to_owned();
                                let mut valid_card = vec![];
                                for it in _p.arranged.iter().zip(_p.wild.iter()) {
                                    let (&_a, _w) = it;
                                    if let &Some(ref __w) = _w {
                                        word.push_str(&__w);
                                        valid_card.push(None);
                                    } else {
                                        let letter = cardmeta[_a].letter;
                                        word.push_str(&letter);
                                        valid_card.push(Some(_a));
                                    }
                                }
                                if wordapi::there_such_word(&word) {
                                    let mut resolve_option = false;
                                    let mut adv_vec = vec![];
                                    let mut hor_vec = vec![];
                                    let mut mys_vec = vec![];
                                    let mut rom_vec = vec![];
                                    for t in &valid_card {
                                        if let &Some(_c) = t {
                                            track_genre(_c.clone(),
                                                        &cardmeta,
                                                        &mut adv_vec,
                                                        &mut hor_vec,
                                                        &mut mys_vec,
                                                        &mut rom_vec);
                                            resolve_giveable(_c.clone(),
                                                             &cardmeta,
                                                             player_id,
                                                             &mut temp_board,
                                                             &mut resolve_option);
                                        }
                                    }
                                    resolve_genre_giveable(player_id,
                                                           &mut temp_board,
                                                           &mut resolve_option,
                                                           &cardmeta,
                                                           &adv_vec,
                                                           &hor_vec,
                                                           &mys_vec,
                                                           &rom_vec);
                                    if resolve_option {
                                        *(*_gamestate) = GameState::ResolveOption(valid_card);
                                    }
                                }
                            }

                        }
                        &mut &mut GameState::Buy => {}
                        _ => {}
                    }
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
pub fn resolve_card_during_play(card_index: usize,
                                cardmeta: &[cards::ListCard<BoardStruct>; 180],
                                player_id: usize,
                                board: &mut BoardStruct) {

}
pub fn resolve_purchase_giveables(card_index: usize,
                                  cardmeta: &[cards::ListCard<BoardStruct>; 180],
                                  _p: &mut Player) {
    if let cards::GIVEABLE::VP(_vp) = cardmeta[card_index].purchase_giveables {
        _p.vp += _vp;
    }
}
pub fn resolve_giveable(card_index: usize,
                        cardmeta: &[cards::ListCard<BoardStruct>; 180],
                        player_id: usize,
                        board: &mut BoardStruct,
                        resolve_option: &mut bool) {
    if let Some(ref mut z) = board.players.get_mut(player_id) {
        giveable_match(z, &cardmeta[card_index].giveables, resolve_option);
    }
}
pub fn giveable_match(z: &mut Player, giveables: &cards::GIVEABLE, resolve_option: &mut bool) {
    match giveables {
        &cards::GIVEABLE::VP(_x) => {
            z.vp += _x;
        }
        &cards::GIVEABLE::COIN(_x) => {
            z.coin += _x;
        }
        &cards::GIVEABLE::VPCOIN(_x1, _x2) => {
            z.vp += _x1;
            z.coin += _x2;
        }
        &cards::GIVEABLE::COININK(_x) => {
            z.coin += _x;
            z.ink += 1;
        }
        &cards::GIVEABLE::VPINK(_x) => {
            z.vp += _x;
            z.ink += 1;
        }
        &cards::GIVEABLE::NONE => {}
        _ => {
            *resolve_option = true;
        }
    }
}
pub fn track_genre(card_index: usize,
                   cardmeta: &[cards::ListCard<BoardStruct>; 180],
                   adv: &mut Vec<usize>,
                   hor: &mut Vec<usize>,
                   mys: &mut Vec<usize>,
                   rom: &mut Vec<usize>) {
    match cardmeta[card_index].genre {
        cards::Genre::ADVENTURE => {
            adv.push(card_index);
        }
        cards::Genre::HORROR => {
            hor.push(card_index);
        }
        cards::Genre::MYSTERY => {
            mys.push(card_index);
        }
        cards::Genre::ROMANCE => {
            rom.push(card_index);
        }
        _ => {}
    }
}
pub fn resolve_genre_giveable(player_id: usize,
                              board: &mut BoardStruct,
                              resolve_option: &mut bool,
                              cardmeta: &[cards::ListCard<BoardStruct>; 180],
                              adv: &Vec<usize>,
                              hor: &Vec<usize>,
                              mys: &Vec<usize>,
                              rom: &Vec<usize>) {
    if let Some(ref mut z) = board.players.get_mut(player_id) {
        if adv.len() >= 2 {
            for &_c in adv {
                giveable_match(z, &cardmeta[_c].giveables, resolve_option);
            }
        }
        if hor.len() >= 2 {
            for &_c in hor {
                giveable_match(z, &cardmeta[_c].giveables, resolve_option);
            }
        }
        if mys.len() >= 2 {
            for &_c in mys {
                giveable_match(z, &cardmeta[_c].giveables, resolve_option);
            }
        }
        if rom.len() >= 2 {
            for &_c in rom {
                giveable_match(z, &cardmeta[_c].giveables, resolve_option);
            }
        }
    }

}
