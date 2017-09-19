#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub letter: String,
    pub index: usize,
    pub inked: bool,
}
impl Card {
    pub fn inked(&mut self) {
        self.inked = true;
    }
    /*   pub fn wild_with(&mut self,l:& str){
        self.letter = l;
    }
*/
}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum GIVEABLE {
    NONE,
    VP(usize),
    COIN(usize),
    VPCOIN(usize, usize),
    COININK(usize),
    VPINK(usize),
    VPORCOIN(usize),
    VPORCOININK(usize),
    INK,
}
#[derive(Clone,Debug)]
pub enum Genre {
    MYSTERY,
    HORROR,
    ADVENTURE,
    ROMANCE,
    NONE,
}

pub struct ListCard<T> {
    pub id: usize,
    pub letter: &'static str,
    pub cost: usize,
    pub purchase_giveables: GIVEABLE,
    pub giveables: GIVEABLE,
    pub genre_giveables: GIVEABLE,
    pub trash: GIVEABLE,
    pub genre: Genre,
    pub rotated: bool,
    pub giveablefn: Option<Box<Fn(&mut T, usize)>>,
    pub genrefn: Option<Box<Fn(&mut T, usize)>>,
}
pub trait Board {}
macro_rules! listcard_map {
    (structtype:$s_alias:ty,
cards:{  $(($id:expr,$letter:expr,$cost:expr,$purchase_giveables:expr,$giveables:expr,$genre_giveables:expr,$trash:expr,$genre:expr,$rotated:expr,$giveablefn:expr,$genrefn:expr)),* $(,)*
})
        => {
         let cards:[ListCard<$s_alias>;180] =[
             $(ListCard{
                  id:$id,
                  letter:$letter,
                  cost:$cost,
                  purchase_giveables:$purchase_giveables,
                  giveables:$giveables,
                  genre_giveables:$genre_giveables,
                  trash:$trash,
                  genre:$genre,
                  rotated:$rotated,
                  giveablefn:$giveablefn,
                  genrefn:$genrefn
             },)*
         ];
         cards
    }}
pub fn populate<T: Board>() -> [ListCard<T>; 180] {
    listcard_map!{
        structtype:T,
        cards:{
        (0,"a",7,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (1,"b",4,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (2,"c",3,GIVEABLE::VP(1),GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None,None),
        (3,"d",4,GIVEABLE::VP(1),GIVEABLE::COIN(2),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (4,"e",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None,None),
        (5,"f",8,GIVEABLE::VP(1),GIVEABLE::VP(5),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (6,"g",6,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::COIN(1),GIVEABLE::COIN(4),Genre::ADVENTURE,false,None,None),
        (7,"h",3,GIVEABLE::VP(3),GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(1),Genre::ADVENTURE,false,None,None),
        (8,"i",6,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (9,"j",5,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::VP(2),Genre::ADVENTURE,false,None,None),
        (10,"k",9,GIVEABLE::VP(2),GIVEABLE::VP(5),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (11,"l",4,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (12,"m",6,GIVEABLE::VP(3),GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (13,"n",4,GIVEABLE::VP(1),GIVEABLE::COIN(2),GIVEABLE::VPCOIN(1,1),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (14,"o",6,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ADVENTURE,false,None,Some(Box::new(|ref mut b, p| {
            //genre, 2cents for every adv
        }))),
        (15,"p",4,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (16,"q",7,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(4),GIVEABLE::VP(3),Genre::ADVENTURE,false,None,None),
        (17,"r",3,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (18,"s",5,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::VP(2),Genre::ADVENTURE,false,None,None),
        (19,"t",4,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (20,"u",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(3),GIVEABLE::VP(2),Genre::ADVENTURE,false,None,None),
        (21,"v",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (22,"w",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None,None),
        (23,"x",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::VP(2),Genre::ADVENTURE,false,None,None),
        (24,"y",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::COIN(1),Genre::ADVENTURE,false,None,None),
        (25,"z",5,GIVEABLE::VP(3),GIVEABLE::VP(4),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (26,"a",5,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::COIN(3),Genre::ADVENTURE,false,None,None),
        (27,"c",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ADVENTURE,true,None,None),
        (28,"g",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (29,"i",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ADVENTURE,false,None,None),
        (30,"j",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None,None),
        (31,"p",8,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ADVENTURE,true,None,None),
        (32,"l",2,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::VP(1),Genre::ADVENTURE,false,None,None),
        (33,"w",5,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ADVENTURE,true,None,None),
        (34,"y",4,GIVEABLE::VP(4),GIVEABLE::COIN(2),GIVEABLE::VP(2),GIVEABLE::COIN(2),Genre::ADVENTURE,false,None,None),
        (35,"b",6,GIVEABLE::NONE,GIVEABLE::COIN(3),GIVEABLE::COININK(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (36,"c",5,GIVEABLE::NONE,GIVEABLE::VPINK(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (37,"d",9,GIVEABLE::NONE,GIVEABLE::VPINK(3),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (38,"e",8,GIVEABLE::NONE,GIVEABLE::COININK(2),GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (39,"f",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPORCOININK(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (40,"g",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPINK(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (41,"h",7,GIVEABLE::NONE,GIVEABLE::VPCOIN(1,2),GIVEABLE::VPCOIN(2,1),GIVEABLE::NONE,Genre::HORROR,false,None,Some(Box::new(|ref mut b, p| {
            //horror, genre other player -1 ink/remover
        }))),
        (42,"i",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (43,"j",5,GIVEABLE::NONE,GIVEABLE::VPINK(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (44,"k",2,GIVEABLE::NONE,GIVEABLE::VPORCOIN(1),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (45,"l",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::INK,GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (46,"m",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (47,"n",5,GIVEABLE::NONE,GIVEABLE::VPINK(2),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (48,"o",4,GIVEABLE::NONE,GIVEABLE::VPORCOIN(2),GIVEABLE::VPORCOIN(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (49,"p",3,GIVEABLE::NONE,GIVEABLE::VPINK(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (50,"q",4,GIVEABLE::NONE,GIVEABLE::COIN(3),GIVEABLE::COININK(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (51,"r",4,GIVEABLE::NONE,GIVEABLE::VPORCOIN(1),GIVEABLE::COININK(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (52,"s",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (53,"t",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VPINK(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (54,"u",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (55,"v",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::VPINK(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (54,"w",4,GIVEABLE::NONE,GIVEABLE::VPINK(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (55,"x",6,GIVEABLE::NONE,GIVEABLE::VPINK(3),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (56,"y",3,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COININK(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (57,"z",3,GIVEABLE::NONE,GIVEABLE::VPORCOIN(2),GIVEABLE::VPORCOININK(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (58,"v",5,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COININK(1),GIVEABLE::NONE,Genre::HORROR,true,None,None),
        (59,"x",2,GIVEABLE::NONE,GIVEABLE::VPORCOIN(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (60,"w",5,GIVEABLE::NONE,GIVEABLE::COININK(2),GIVEABLE::COIN(3),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (61,"u",6,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (62,"n",6,GIVEABLE::NONE,GIVEABLE::VPORCOIN(1),GIVEABLE::VPINK(2),GIVEABLE::NONE,Genre::HORROR,true,None,None),
        (63,"s",7,GIVEABLE::NONE,GIVEABLE::VPINK(3),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (64,"c",8,GIVEABLE::NONE,GIVEABLE::COININK(2),GIVEABLE::COIN(3),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (65,"e",5,GIVEABLE::NONE,GIVEABLE::VPORCOIN(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (66,"d",4,GIVEABLE::NONE,GIVEABLE::VPORCOIN(1),GIVEABLE::VPCOIN(1,1),GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (67,"a",3,GIVEABLE::NONE,GIVEABLE::VPORCOIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::HORROR,false,None,None),
        (68,"b",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:Lockup offer rowcard
        })),None),
        (69,"c",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover
        })),Some(Box::new(|ref mut b, p| {
            //mystery,  gen:Lock up offer row
        }))),
        (70,"d",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover adjacent wild
        })),None),
        (71,"e",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery, gen:uncover adjacent wild
        }))),
        (72,"f",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:Lockup offer rowcard
        })),None),
        (73,"g",6,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery, gen:uncover adjacent
        }))),
        (74,"h",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery,) Non-gen:Lockup offer rowcard
        })),None),
        (75,"i",5,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover adjacent
        })),None),
        (76,"j",8,GIVEABLE::NONE,GIVEABLE::VP(5),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover adjacent
        })),None),
        (77,"k",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery, gen:uncover adjacent
        }))),
        (78,"l",6,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery, gen:uncover adjacent
        }))),
        (79,"m",3,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover adjacent
        })),None),
        (80,"n",7,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery, gen:all wild cards +vp
        }))),
        (81,"o",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:Lockup offer rowcard
        })),Some(Box::new(|ref mut b, p| {
            //mystery,  gen: uncover adjacent
        }))),
        (82,"p",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover adjacent
        })),None),
        (83,"q",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery, gen: uncover adjacent
        }))),
        (84,"r",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery, gen: uncover adjacent
        }))),
        (85,"s",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:Lockup offer rowcard
        })),None),
        (86,"t",6,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen: uncover adjacent
        })),None),
        (87,"u",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery,  gen: uncover adjacent
        }))),
        (88,"v",9,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::VP(4),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover adjacent
        })),None),
        (89,"w",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery,  gen: uncover adjacent
        }))),
        (90,"x",3,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:Lockup offer rowcard
        })),None),
        (91,"y",7,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery, gen: uncover adjacent
        }))),
        (92,"z",5,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery,  gen: uncover adjacent
        }))),
        (93,"i",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery,  gen: lockup after rowcard
        }))),
        (94,"a",5,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,true,None,None),
        (95,"f",5,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:Lockup offer rowcard
        })),None),
        (96,"m",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,true,None,Some(Box::new(|ref mut b, p| {
            //mystery,  gen: lockup offer row
        }))),
        (97,"k",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover adjacent
        })),None),
        (98,"q",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(3),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover adjacent
        })),None),
        (99,"t",8,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::MYSTERY,true,None,Some(Box::new(|ref mut b, p| {
            //mystery, gen:lockup offer row
        }))),
        (100,"r",6,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover adjacent
        })),Some(Box::new(|ref mut b, p| {
            //mystery, gen:Lockup
        }))),
        (101,"p",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::MYSTERY,false,None,Some(Box::new(|ref mut b, p| {
            //mystery, gen:lockup
        }))),
        (102,"a",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::MYSTERY,false,Some(Box::new(|ref mut b, p| {
            //mystery, Non-gen:uncover adjacent
        })),None),
        (103,"z",5,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:double adjacent
        })),None),
        (104,"w",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:thrash other, gen:double adjacent
        })),None),
        (105,"v",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:thrash other
        }))),
        (106,"u",9,GIVEABLE::NONE,GIVEABLE::VP(5),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:double adjacent
        }))),
        (107,"t",3,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen: thrash other
        }))),
        (108,"s",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:thrash other
        })),None),
        (109,"r",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:keep or discard top3
        }))),
        (110,"q",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:thrash other
        })),None),
        (111,"p",6,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:double adjacent
        }))),
        (112,"o",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:double adjacent
        }))),
        (113,"n",2,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:thrash other
        }))),
        (114,"m",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:thrash other
        })),None),
        (115,"l",8,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:double adjacent
        }))),
        (116,"k",3,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:thrash other
        }))),
        (117,"j",6,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:double adjacent,gen:thrash other
        })),None),
        (118,"i",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,None),
        (119,"h",3,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:thrash
        })),None),
        (120,"g",3,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:thrash other
        })),None),
        (121,"f",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:thrash other
        })),None),
        (122,"e",6,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:thrash other
        }))),
        (123,"d",4,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:double adjacent
        }))),
        (124,"c",3,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:thrash other
        }))),
        (125,"b",3,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:double adjacent
        })),None),
        (126,"a",4,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:thrash other
        })),None),
        (127,"b",5,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,true,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:thrash other
        }))),
        (128,"e",2,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,None,None),
        (129,"f",6,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:double adjacent
        })),Some(Box::new(|ref mut b, p| {
            //rommanc, gen:thrash other
        }))),
        (130,"h",7,GIVEABLE::NONE,GIVEABLE::VP(3),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ROMANCE,false,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:double adjacent
        }))),
        (131,"k",5,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,true,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:trash other
        }))),
        (132,"n",5,GIVEABLE::NONE,GIVEABLE::COIN(2),GIVEABLE::COIN(1),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:trash other
        })),None),
        (133,"o",8,GIVEABLE::NONE,GIVEABLE::VPCOIN(1,2),GIVEABLE::VPCOIN(1,1),GIVEABLE::NONE,Genre::ROMANCE,true,None,None),
        (134,"r",5,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,true,None,Some(Box::new(|ref mut b, p| {
            //rommanc, gen:trash
        }))),
        (135,"z",4,GIVEABLE::NONE,GIVEABLE::VP(2),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:trash other
        })),None),
        (136,"y",4,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:double adjacent
        })),Some(Box::new(|ref mut b, p| {
            //rommanc, gen:thrash other
        }))),
        (137,"x",7,GIVEABLE::NONE,GIVEABLE::VP(4),GIVEABLE::VP(2),GIVEABLE::NONE,Genre::ROMANCE,false,Some(Box::new(|ref mut b, p| {
            //rommanc, Non-gen:trash other card
        })),None),
        (138,"a",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (139,"a",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (140,"c",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (141,"d",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (142,"e",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (143,"a",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (144,"g",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (145,"h",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (146,"i",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (147,"a",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (148,"e",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (149,"l",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (150,"m",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (151,"n",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (152,"o",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (153,"p",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (154,"e",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (155,"r",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (156,"s",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (157,"t",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (158,"u",0,GIVEABLE::NONE,GIVEABLE::VP(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (159,"e",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (160,"i",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (161,"i",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (162,"i",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (163,"l",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (164,"l",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (165,"l",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (166,"n",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (167,"n",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (168,"n",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (169,"r",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (170,"r",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (171,"r",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (172,"s",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (173,"s",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (174,"s",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (175,"t",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (176,"t",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        (177,"t",0,GIVEABLE::NONE,GIVEABLE::COIN(1),GIVEABLE::NONE,GIVEABLE::NONE,Genre::NONE,false,None,None),
        }
        
    }
}
