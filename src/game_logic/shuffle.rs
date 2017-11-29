use codec_lib::codec::*;
use codec_lib::cards::{WaitForInputType, WaitForSingleInput};
use game_logic::game_engine::GameCon;
pub update_into_spell<T: GameCon>(_board: &mut BoardStruct,
                               player_id: usize,
                               con: &T,
                               wait_for_input: &mut [WaitForInputType; 4],
                               log: &mut Vec<ClientReceivedMsg>){
    let  (state2,word2) =
            if _index == 0 {                
                 (GameState::TurnToSubmit,"It is your turn to submit word.".to_owned())
            } else {
                (GameState::Spell,"It is player 1's turn to submit word.".to_owned())
            };
         let _g2: WaitForSingleInput =
            (0,
             word2,
             vec![(state2,
                   "Continue".to_owned(),
                   Box::new(move |ref mut _p, ref mut _rmcards, mut _unknown| {
                   }))]);
        wait_for_input[_index].push(Some(_g2));
        wait_for_input[_index].push(None);
}