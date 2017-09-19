use server_lib::codec::Player;
use server_lib::cards::Board;
pub struct BoardStruct {
    pub players: Vec<Player>,
}
impl Board for BoardStruct {}
impl BoardStruct {
    pub fn new(players: Vec<Player>) -> BoardStruct {
        BoardStruct { players: players }
    }
}
