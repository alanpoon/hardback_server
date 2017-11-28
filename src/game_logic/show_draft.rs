use codec_lib::codec::*;
use codec_lib::cards::{WaitForInputType, WaitForSingleInput};
use game_logic::game_engine::GameCon;
use rand::{thread_rng, Rng, SeedableRng, StdRng};

pub fn broadcast<T: GameCon>(randseedbool: bool,
                             gamestates: &mut Vec<GameState>,
                             cons: &Vec<T>,
                             players: &Vec<Player>,
                             unknown:&mut [Vec<usize>;4],//player's draft
                             wait_for_input: &mut [WaitForInputType; 4],
                             log: &mut Vec<ClientReceivedMsg>) {
    for (_index, _con) in cons.iter().enumerate() {
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
        let (state, word) =
            if _index == 0 {
                (GameState::TurnToSubmit,
                 "Let's Start! You drew 5 cards into your hand. It is your turn to submit word."
                     .to_owned())
            } else {
                (GameState::Spell,"Let's Start! You drew 5 cards into your hand. It is player 1's turn to submit word.".to_owned())
            };
        let _g: WaitForSingleInput = (0,
                                      word,
                                      vec![(state,
                                            "Continue".to_owned(),
                                            Box::new(move |ref mut _p, ref mut _rmcards| {
            if randseedbool.clone() {
                let seed: &[_] = &[1, 2, 3, 4];
                let mut rng: StdRng = SeedableRng::from_seed(seed);
                rng.shuffle(&mut unknown[_index]);
            } else {
                let mut rng = thread_rng();
                rng.shuffle(&mut unknown[_index]);
            }
            let vecdraft = unknown[_index].split_off(5);
            _p.hand = vecdraft;
        }))]);
        wait_for_input[_index].push(Some(_g));
        wait_for_input[_index].push(None);
    }

}
pub fn give_player_index<T: GameCon>(cons: &Vec<T>, log: &mut Vec<ClientReceivedMsg>) {
    let mut c = 0;
    for _con in cons.iter() {
        let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
        h.set_player_index(c);
        _con.tx_send(h, log);
        c += 1;
    }

}
