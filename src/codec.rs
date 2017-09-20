use serde_json;
use serde::{Deserialize, Deserializer};
use cards;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use rand;
fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
    where D: Deserializer<'de>,
          T: Deserialize<'de>
{
    Ok(Some(Option::deserialize(deserializer)?))
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableInfo {
    pub numberOfPlayers: usize,
    pub players: Vec<String>,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PrivateInformation {}
#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct Player {
    pub name: String,
    pub vp: usize,
    pub coin: usize,
    pub ink: usize,
    pub remover: usize,
    pub arranged: Vec<usize>,
    pub wild: Vec<Option<String>>,
    pub inked_cards: Vec<usize>,
    pub hand: Vec<usize>,
    pub draft: Vec<usize>,
    pub discard: Vec<usize>,
    pub lockup:Vec<usize>,
    pub rotated_cards:Vec<usize>
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            vp: 0,
            coin: 0,
            ink: 0,
            remover: 0,
            arranged: vec![],
            wild: vec![],
            inked_cards: vec![],
            hand: vec![],
            draft: vec![],
            discard: vec![],
            lockup:vec![],
            rotated_cards:vec![]
        }
    }
    pub fn starting<T: cards::Board>(&mut self,
                                     cardmeta: &[cards::ListCard<T>; 180],
                                     remaining_deck: &mut Vec<usize>) {
        let mut collected_letter = vec![];
        let mut collected_id = vec![];
        let mut rand_id = vec![];
        let mut two_cards_id = vec![];
        for &cards::ListCard { letter, ref genre, ref giveables, id, .. } in cardmeta.iter() {
            if let &cards::Genre::NONE = genre {
                if let &cards::GIVEABLE::COIN(_) = giveables {
                    if !remaining_deck.contains(&id) {
                        break;
                    } else {
                        if !collected_letter.contains(&letter) {
                            collected_letter.push(letter);
                            let t = remaining_deck.remove(id);
                            collected_id.push(t);

                        }
                    }
                } else if let &cards::GIVEABLE::VP(_) = giveables {
                    if !remaining_deck.contains(&id) {
                        break;
                    } else {
                        rand_id.push(id);
                    }
                }
            }
        }
        let between = Range::new(0, rand_id.len() - 1);
        let mut rng = rand::thread_rng();
        for _ in 0..2 {
            let c = between.ind_sample(&mut rng) as usize;
            if let Some(&idz) = rand_id.get(c) {
                two_cards_id.push(idz);
                rand_id.remove(idz);
            }
        }
        collected_id.extend(two_cards_id);
        rng.shuffle(&mut collected_id);
        let vecdraft = collected_id.split_off(5);
        self.hand = collected_id;
        self.draft = vecdraft;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameCommand {
    pub use_ink: Option<usize>,
    pub use_remover: Option<usize>,
    pub arranged: Option<Vec<usize>>,
    pub wild: Option<(usize, String)>,
    pub submit_word: Option<bool>,
    pub reply: Option<usize>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoardCodec {
    pub players: Vec<Player>,
}
CGM_codec!{
    structname:ServerReceivedMsg,
    rename:{
    },optional:{
    (gamecommand,set_gamecommand,GameCommand),
   (newTable,set_new_table,bool),
    (ready,set_ready,bool),
    (joinTable,set_join_table,i32),
    (changePlayers,set_change_player,usize),
    (leaveTable,set_leave_table,bool),
    (joinLobby,set_join_lobby,bool),
    (namechange,set_name_change,String),
    (chat,set_chat,String),
    (location,set_location,String),
    },rename_optional:{},else:{}
}
CGM_codec!{
    structname:ClientReceivedMsg,
   rename:{
    },optional:{
    (tables,set_tables,Vec<TableInfo>),
    (tablenumber,set_tablenumber,i32),
    (players,set_players,Vec<Player>),
    (privateInformation,set_private_information,PrivateInformation),
    (boardstate,set_boardstate,Result<BoardCodec,String>),
    (request,set_request,(String,Vec<String>)),
    (reason,set_reason,String),
    (optional,set_optional,bool),
    (location,set_location,String),
    (sender,set_sender,String),
    (message,set_message,String),
    (log,set_log,String),
    },rename_optional:{ (type_name,set_type_name,String,"type"),},else:{}
}
