use server_lib::cards::*;
use server_lib::cards;
use game_logic::board::BoardStruct;
pub fn trash_for_something( mut _board: &mut BoardStruct, player_id: usize,card_index:usize,cardmeta:&[],wait_for_input:&mut [Option<Vec<Box<Fn(&mut Player, &mut Vec<usize>)>>>; 4] ){
    if let Some(_p) = _board.players.get_mut(player_id){
        match _p.
    }
}
pub fn convert_trash_giveable_wait_for_input(trash:GIVEABLE,player_id:usize,wait_for_input:&mut [Option<Vec<Box<Fn(&mut Player, &mut Vec<usize>)>>>; 4]){
    match trash{
        GIVEABLE::VP(z)=>{
            wait_for_input[player_id]=Some(vec![Box::new()])
        }
    }
}