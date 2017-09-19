use server_lib::codec::Player;
use server_lib::cards::Board;
pub struct BoardStruct {
    pub players: Vec<Player>,
    pub resolve_option:Option<Vec<Option<usize>>>, //if some, wait for user's input
}
impl Board for BoardStruct {
    fn two_cent_per_adv(&mut self,player_id:usize){

    }
    fn minus_other_ink(&mut self,player_id:usize){

    }
    fn lockup_offer(&mut self,player_id:usize){

    }
    fn uncover_adjacent(&mut self,player_id:usize){

    }
    fn double_adjacent(&mut self,player_id:usize){

    }
    fn trash_other(&mut self,player_id:usize){

    }
    fn one_vp_per_wild(&mut self,player_id:usize){

    }
    fn keep_or_discard_three(&mut self,player_id:usize){

    }
}
impl BoardStruct {
    pub fn new(players: Vec<Player>) -> BoardStruct {
        BoardStruct { players: players,resolve_option:None }
    }
}
