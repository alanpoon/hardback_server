#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card{
    pub letter:&'static str,
    pub index:i32,
    pub inked:bool,
}
impl Card{
    pub fn inked(&mut self){
        self.inked = true;
    }
    pub fn wild_with(&mut self,l:&'static str){
        self.letter = l;
    }

}