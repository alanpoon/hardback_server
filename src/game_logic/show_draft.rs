use codec_lib::codec::*;
use game_logic::board::BoardStruct;
use game_logic::game_engine::GameCon;
use rand::{thread_rng, Rng, SeedableRng, StdRng};
use std::collections::HashMap;
pub fn go_to_shuffle<T: GameCon>(randseedbool: bool,
                                 _board: &mut BoardStruct,
                                 player_id: usize,
                                 con: &T,
                                 _gamestate: &mut GameState,
                                 go_to_shuffle: &Option<bool>,
                                 unknown: &mut Vec<usize>,
                                 ticks: Option<u16>,
                                 log: &mut Vec<ClientReceivedMsg>) {
    if let &Some(true) = go_to_shuffle {
        if let Some(ref mut _p) = _board.players.get_mut(player_id) {
            *unknown = _p.draft.clone();
            if randseedbool.clone() {
                let seed: &[_] = &[1, 2, 3, 4];
                let mut rng: StdRng = SeedableRng::from_seed(seed);
                rng.shuffle(unknown);
            } else {
                let mut rng = thread_rng();
                rng.shuffle(unknown);
            }
            _p.hand = unknown.split_off(5);
            _p.draft = vec![];
        }

        *_gamestate = GameState::Shuffle;
        let gamestate_dummy_vec = _board.players
            .iter()
            .map(|x| GameState::Shuffle)
            .collect::<Vec<GameState>>();
        let k: Result<BoardCodec, String> = Ok(BoardCodec {
                                                   players: _board.players.clone(),
                                                   gamestates: gamestate_dummy_vec,
                                                   offer_row: _board.offer_row.clone(),
                                                   turn_index: 0,
                                                   ticks: ticks,
                                               });

        let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
        h.set_boardstate(k);
        con.tx_send(h, log);
        if player_id == 0 {
            *_gamestate = GameState::TurnToSubmit;
        } else {
            *_gamestate = GameState::Spell;
        }
    }
}
pub fn broadcast<T: GameCon>(randseedbool: bool,
                             gamestates: &mut Vec<GameState>,
                             cons: &HashMap<usize, T>,
                             players: &Vec<Player>,
                             unknown: &mut [Vec<usize>; 4], //player's draft
                             log: &mut Vec<ClientReceivedMsg>) {
    for (_index, _con) in cons.iter() {
        println!("broadcast_show_draft");
        let k: Result<BoardCodec, String> = Ok(BoardCodec {
                                                   players: players.clone(),
                                                   gamestates: gamestates.clone(),
                                                   offer_row: vec![],
                                                   turn_index: 0,
                                                   ticks: None,
                                               });

        let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
        h.set_boardstate(k);
        _con.tx_send(h, log);

    }

}
pub fn give_player_index<T: GameCon>(cons: &HashMap<usize, T>, log: &mut Vec<ClientReceivedMsg>) {
    let mut c = 0;
    for (_, _con) in cons.iter() {
        let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
        h.set_player_index(c);
        _con.tx_send(h, log);
        c += 1;
    }

}
