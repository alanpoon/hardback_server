use server_lib::codec::Player;
use server_lib::cards::Board;
pub struct BoardStruct {}
impl Board for BoardStruct {}
impl BoardStruct {
    pub fn new() -> BoardStruct {
        BoardStruct {}
    }
}
