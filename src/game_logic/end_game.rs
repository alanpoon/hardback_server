use codec_lib::codec::*;
use game_logic::game_engine::GameCon;
use std::collections::HashMap;
pub fn first_to_60<T: GameCon>(players: &Vec<Player>,
                               connections: &HashMap<usize, T>,
                               player_60: &mut Option<usize>,
                               game_end_notified: &mut bool,
                               log: &mut Vec<ClientReceivedMsg>) {
    //with winner
    if player_60.is_none() {
        for it in players.iter().enumerate() {
            let (ref _index, ref _p) = it;
            if _p.vp + _p.literacy_award >= 60 {
                *player_60 = Some(_index.clone());
            }
        }
    } else if !*game_end_notified {
        let mut _st = "Player ".to_owned();
        _st.push_str(&(player_60.unwrap() + 1).to_string());
        _st.push_str(" has reached [60 vp] The game will end in this round.");
        for (_, _con) in connections {
            let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
            h.set_notification(_st.clone());
            _con.tx_send(h, log);
        }
        *game_end_notified = true;
    }

}
pub fn show_result<T: GameCon>(players: &mut Vec<Player>,
                               gamestates: &mut Vec<GameState>,
                               cons: &HashMap<usize, T>,
                               player_60: &Option<usize>,
                               turn_after_winner: &mut usize,
                               turn_index: usize,
                               ticks: Option<u16>,
                               log: &mut Vec<ClientReceivedMsg>) {
    if let &Some(player_index) = player_60 {
        println!("end bah0");
        let still_processing = !gamestates.iter()
                                    .filter(|x| x == &&GameState::PreDrawCard)
                                    .collect::<Vec<&GameState>>()
                                    .is_empty();
        if still_processing {
            if *turn_after_winner == 1 {
                let mut top_player = 0;
                let mut highest_score: (usize, usize) = (0, 0); //vp,ink
                for (_i, _p) in players.iter_mut().enumerate() {
                    println!("i {:?}", _i);
                    _p.hand = vec![];
                    _p.arranged = vec![];
                    _p.draftlen = 0;
                    _p.skip_cards = vec![];
                    if _p.vp + _p.literacy_award >= highest_score.0 {
                        if _p.vp + _p.literacy_award == highest_score.0 {
                            if _p.ink > highest_score.1 {
                                top_player = _i;
                                highest_score = (_p.vp + _p.literacy_award, _p.ink);
                            }
                        } else {
                            top_player = _i;
                            highest_score = (_p.vp + _p.literacy_award, _p.ink);
                        }
                    }
                }
                println!("top player {:?}", top_player);
                for _g in gamestates.iter_mut() {
                    *_g = GameState::ShowResult(top_player);
                }
                for (_, _con) in cons.iter() {
                    let k: Result<BoardCodec, String> = Ok(BoardCodec {
                                                               players: players.clone(),
                                                               gamestates: gamestates.clone(),
                                                               offer_row: vec![],
                                                               turn_index: turn_index.clone(),
                                                               ticks: ticks,
                                                           });

                    let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
                    h.set_boardstate(k);
                    _con.tx_send(h, log);
                }
            }
            if (*turn_after_winner == 0) & (turn_index == player_index) {
                *turn_after_winner += 1;
            }

        }
    }
}
