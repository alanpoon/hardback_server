use board::Board;
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

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum GIVEABLE {
    NONE
    VP(i32),
    COIN(i32),
    VPCOIN(i32,i32),
    COININK(i32),
    VPINK(i32),
    VPORCOIN(i32),
    VPORCOININK(i32)
  
}
#[derive(Clone,Debug)]
pub enum Genre{
    MYSTERY,
    HORROR,
    ADVENTURE,
    ROMANCE
}
#[derive(Clone,Debug)]
pub struct ListCard {
    pub id:i32,
    pub letter:&'static str,
    pub cost:i32,
    pub purchase_giveables:GIVEABLE,
    pub giveables:GIVEABLE,
    pub genre_giveables:GIVEABLE,
    pub thrash::GIVEABLE,
    pub genre:Genre,
    pub rotated:bool
    pub customfn:Option<Box<Fn(&mut Board, i32)>>
}

macro_rules! listcard_map {
    ($(($id:expr,$letter:expr,$cost:expr,$giveables:expr,$genre_giveables:expr,$thrash:expr,$genre:expr,$rotated:expr,$customfn:expr)),* $(,)*) => {{
         let cards:HashMap<i32,ListCard> =[
             $(($id,ListCard{
                  id:$id,
                  letter:$letter,
                  cost:$cost,
                  giveables:$giveables,
                  genre_giveables:$genre_giveables,
                  thrash:$thrash,
                  genre:$genre,
                  rotated:$rotated,
                  customfn:$customfn
             }),)*
         ].iter().cloned().collect();
         cards
    }}
pub fn populate() -> (HashMap<i32, ListCard>, HashMap<i32, BlowupCard>) {
    let l = listcard_map!{
        (0,"a",7,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (1,"b",4,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (2,"c",3,GIVEABLE::VP(1),GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None),
        (3,"d",4,GIVEABLE::VP(1),GIVEABLE::COIN(2),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (4,"e",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None),
        (5,"f",8,GIVEABLE::VP(1),GIVEABLE::VP(5),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (6,"g",6,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::COIN(1),GIVEABLE::COIN(4),Genre::ADVENTURE,false,None),
        (7,"h",3,GIVEABLE::VP(3),GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(1),Genre::ADVENTURE,false,None),
        (8,"i",6,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (9,"j",5,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::VP(2),Genre::ADVENTURE,false,None),
        (10,"k",9,GIVEABLE::VP(2),GIVEABLE::VP(5),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (11,"l",4,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (12,"m",6,GIVEABLE::VP(3),GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (13,"n",4,GIVEABLE::VP(1),GIVEABLE::COIN(2),GIVEABLE::VPCOIN(1,1),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (14,"o",6,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,,GIVEABLE::NONE,Genre::ADVENTURE,false,Box::new(|ref mut b, p| {
            //genre, 2cents for every adv
        })),
        (15,"p",4,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (16,"q",7,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(4),GIVEABLE::VP(3),Genre::ADVENTURE,false,None),
        (17,"r",3,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (18,"s",5,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::VP(2),Genre::ADVENTURE,false,None),
        (19,"t",4,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (20,"u",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(3),GIVEABLE::VP(2),Genre::ADVENTURE,false,None),
        (21,"v",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (22,"w",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None),
        (23,"x",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::VP(2),Genre::ADVENTURE,false,None),
        (24,"y",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::COIN(1),Genre::ADVENTURE,false,None),
        (25,"z",5,GIVEABLE::VP(3),GIVEABLE::VP(4),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (26,"a",5,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::COIN(3),Genre::ADVENTURE,false,None),
        (27,"c",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,true,None),
        (28,"g",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (29,"i",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ADVENTURE,false,None),
        (30,"j",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None),
        (31,"p",8,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,true,None),
        (32,"l",2,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(1),Genre::ADVENTURE,false,None),
        (33,"w",5,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ADVENTURE,true,None),
        (34,"y",4,GIVEABLE::VP(4),GIVEABLE::COIN(2),GIVEABLE::VP(2),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None),
        (35,"b",6,GIVEABLE::NONE,GIVEABLE::COIN(3),GIVEABLE::COININK(2),GIVEABLE::NONE,Genre::HORROR,false,None),
        (36,"c",5,GIVEABLE::NONE,GIVEABLE::VPINK(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None),
        (37,"d",9,GIVEABLE::NONE,GIVEABLE::VPINK(3),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::HORROR,false,None),
        (38,"e",8,GIVEABLE::NONE,GIVEABLE::COININK(2),GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,Genre::HORROR,false,None),
        (39,"f",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPORCOININK(2),GIVEABLE::NONE,Genre::HORROR,false,None),
        (40,"g",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPINK(2),GIVEABLE::NONE,Genre::HORROR,false,None),
        (41,"h",7,GIVEABLE::NONE,GIVEABLE::VPCOIN(1,2),GIVEABLE::VP(2,1),GIVEABLE::NONE,Genre::HORROR,false,Box::new(|ref mut b, p| {
            //horror, other player -1 ink/remover
        })),
        (42,"i",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,Genre::HORROR,false,None),
        (43,"j",5,GIVEABLE::NONE,GIVEABLE::VPINK(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::HORROR,false,None),
        (44,"k",2,GIVEABLE::NONE,GIVEABLE::VPORCOIN(1),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::HORROR,false,None),

    }

}