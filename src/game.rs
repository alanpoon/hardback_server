use websocket::OwnedMessage;
pub struct GameData{

}
impl GameData {
    pub fn new(){

    }
    pub fn run(){

    }
}
pub enum Message{
    OwnedMessage(OwnedMessage),
    AddConnection(&str),
    RemoveConnection(&str)
}