use std::sync::mpsc;
use server_lib::codec::*;
use server_lib::cards;
use websocket::message::OwnedMessage;
use game_logic::board::BoardStruct;
use game_logic::wordapi;
use rand::Rng;
use rand;
use std;
#[derive(Clone)]
pub enum GameState {
    SubmitWord,
    Buy,
    DrawCard,
    Spell(Action),
}
#[derive(Clone)]
pub enum Action {
    UseInk(usize),
}
pub trait GameCon {
    fn tx_send(&self, OwnedMessage);
}
pub struct GameEngine<con_T: GameCon> {
    players: Vec<Player>,
    connections: Vec<con_T>,
    gamestates: Vec<GameState>,
}
impl<con_T> GameEngine<con_T>
    where con_T: GameCon
{
    pub fn new(players: Vec<Player>, connections: Vec<con_T>) -> Self {
        let mut gamestates_v = vec![];
        for _ in &players {
            gamestates_v.push(GameState::DrawCard);
        }
        GameEngine {
            players: players,
            connections: connections,
            gamestates: gamestates_v,
        }
    }
    pub fn run(&mut self, rx: mpsc::Receiver<(usize, GameCommand)>) {
        let mut last_update = std::time::Instant::now();
        let turn_index = 0;
        let cardmeta: [cards::ListCard<BoardStruct>; 180] = cards::populate::<BoardStruct>();
        let mut remaining_cards = give_outstarting(&mut self.players, &cardmeta);
        let (wait_tx, wait_rx) = mpsc::channel();
        let mut wait_for_input: [Option<Vec<Box<Fn(&mut Player, &mut Vec<usize>)>>>; 4] =
            [None, None, None, None];
        'game: loop {
            let sixteen_ms = std::time::Duration::from_millis(1000);
            let now = std::time::Instant::now();
            let duration_since_last_update = now.duration_since(last_update);

            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }
            let mut temp_players = vec![];
            for mut it in self.players.iter_mut().zip(self.gamestates.iter_mut()) {
                let (ref mut _p, ref mut game_state) = it;
                //((x,y), z)
                match game_state {
                    &mut &mut GameState::DrawCard => {
                        println!("_p.hand.len(){}", _p.hand.len());
                        for _ in 0usize..(5 - _p.hand.len()) {
                            if let Some(n) = _p.draft.pop() {
                                _p.hand.push(n);
                            } else {
                                let mut rng = rand::thread_rng();
                                _p.draft = _p.discard.clone();
                                _p.discard = vec![];
                                rng.shuffle(&mut _p.draft);
                                if let Some(n) = _p.draft.pop() {
                                    _p.hand.push(n);
                                }
                            }
                        }
                        _p.arranged = vec![];
                        _p.wild = vec![];
                        _p.inked_cards = vec![];
                    }
                    _ => {}
                }
                temp_players.push(_p.clone());
            }
            while let Ok(input_request) = wait_rx.recv() {
                match input_request {
                    Some((player_id, header, option_vec)) => {
                        // request Input
                        if let Some::<&con_T>(ref con) = self.connections.get(player_id) {

                            let mut temp_vec: Vec<String> = vec![];
                            let mut temp_wait_for_input: Vec<Box<Fn(&mut Player,&mut Vec<usize>)>> = vec![];
                            for (sz, sb) in option_vec {
                                temp_vec.push(sz);
                                temp_wait_for_input.push(sb);
                            }
                            wait_for_input[player_id] = Some(temp_wait_for_input);
                            let g = json!({
                                              "request": (header, temp_vec)
                                          });
                            con.tx_send(OwnedMessage::Text(g.to_string()));

                        }
                    }
                    None => {
                        //just update boardstae
                        for it in self.connections.iter() {
                            let ref con = it;
                            let k: Result<BoardCodec, String> =
                                Ok(BoardCodec { players: temp_players.clone() });
                            let g = json!({
                                              "boardstate": k
                                          });
                            con.tx_send(OwnedMessage::Text(g.to_string()));
                        }
                    }
                }

            }


            while let Ok((player_id, game_command)) = rx.recv() {
                let mut need_update = false;
                let mut temp_board = BoardStruct::new(self.players.clone(),
                                                      self.gamestates.clone(),
                                                      remaining_cards.clone(),
                                                      wait_tx.clone());
                let mut type_is_reply = false;
                let &GameCommand { reply, .. } = &game_command;
                if let Some(_) = reply {
                    type_is_reply = true;
                }
                if let (&GameCommand { use_ink,
                                       use_remover,
                                       ref arranged,
                                       ref wild,
                                       submit_word,
                                       buyoffer,
                                       buylockup,
                                       .. },
                        Some(ref mut _p),
                        Some(ref con),
                        Some(ref mut _gamestate),
                        &None,
                        false) =
                    (&game_command,
                     self.players.get_mut(player_id),
                     self.connections.get(player_id),
                     self.gamestates.get_mut(player_id),
                     &wait_for_input[player_id],
                     type_is_reply) {

                    match _gamestate {
                        &mut &mut GameState::Spell(ref a) => {
                            if let Some(z) = use_ink {
                                _p.inked_cards.push(z);
                            } else if let Some(z) = use_remover {
                                if _p.inked_cards.contains(&z) {
                                    _p.hand.push(_p.inked_cards.remove(z));
                                } else {
                                    let k: Result<BoardCodec, String> =
                                        Err("cannot remove a card that is not inked".to_owned());
                                    let g = json!({
                                                      "boardstate": k
                                                  });
                                    con.tx_send(OwnedMessage::Text(g.to_string()));

                                }
                            } else if let &Some(ref z) = arranged {
                                _p.arranged = z.to_vec();
                            } else if let &Some((card_index, ref replacement)) = wild {
                                let mut wild_vec = vec![];
                                for _c in &_p.arranged {
                                    if *_c == card_index {
                                        wild_vec.push(Some(replacement.clone()));
                                    } else {
                                        wild_vec.push(None);
                                    }
                                }
                                _p.wild = wild_vec;
                            }
                        }
                        &mut &mut GameState::SubmitWord => {
                            //uses tempboard
                            if let Some(true) = submit_word {
                                let mut word = "".to_owned();
                                let mut valid_card = vec![];
                                for it in _p.arranged.iter().zip(_p.wild.iter()) {
                                    let (&_a, _w) = it;
                                    if let &Some(ref __w) = _w {
                                        word.push_str(&__w);
                                        valid_card.push(None);
                                    } else {
                                        let letter = cardmeta[_a].letter;
                                        word.push_str(&letter);
                                        valid_card.push(Some(_a));
                                    }
                                }
                                if wordapi::there_such_word(&word) {
                                    let mut adv_vec = vec![];
                                    let mut hor_vec = vec![];
                                    let mut mys_vec = vec![];
                                    let mut rom_vec = vec![];
                                    for t in &valid_card {
                                        if let &Some(_c) = t {
                                            track_genre(_c.clone(),
                                                        &cardmeta,
                                                        &mut adv_vec,
                                                        &mut hor_vec,
                                                        &mut mys_vec,
                                                        &mut rom_vec);
                                            resolve_giveable(_c.clone(),
                                                             &cardmeta,
                                                             player_id,
                                                             &mut temp_board,
                                                             wait_tx.clone());
                                        }
                                    }
                                    resolve_genre_giveable(player_id,
                                                           &mut temp_board,
                                                           wait_tx.clone(),
                                                           &cardmeta,
                                                           vec![&adv_vec, &hor_vec, &mys_vec,
                                                                &rom_vec]);


                                }
                            }

                        }
                        &mut &mut GameState::Buy => {
                            if let Some(z) = buyoffer {
                                //z top of remaining deck
                                buy_card_from(z,
                                              &mut remaining_cards,
                                              &cardmeta,
                                              _p,
                                              player_id,
                                              wait_tx.clone());
                            }
                            if let Some(z) = buylockup {
                                //z:index of player.lockup
                                buy_card_from_lockup(z, &cardmeta, _p, player_id, wait_tx.clone());
                            }
                        }
                        _ => {}
                    }
                }
                //save temp_board.players to self.players
                for mut it in temp_board.players.iter_mut().zip(self.players.iter_mut()) {
                    let (ref mut _tb_p, ref mut _p) = it;
                    *_p = _tb_p;
                }
                if let (&GameCommand { reply, .. }, Some(_p), _wait, true) =
                    (&game_command,
                     self.players.get_mut(player_id),
                     &mut wait_for_input[player_id],
                     type_is_reply) {
                    if let (Some(_reply), &&mut Some(ref _wait_vec)) = (reply, &_wait) {
                        if let Some(_closure) = _wait_vec.get(_reply) {
                            (*_closure)(_p, &mut remaining_cards);
                        }
                    }
                    *_wait = None;
                }

            }
            last_update = std::time::Instant::now();
        }
    }
}
pub fn give_outstarting(players: &mut Vec<Player>,
                        cardmeta: &[cards::ListCard<BoardStruct>; 180])
                        -> Vec<usize> {
    let mut remaining_deck = vec![];
    for _p in players {
        _p.starting::<BoardStruct>(cardmeta, &mut remaining_deck);
    }
    remaining_deck
}
pub fn resolve_card_during_play(card_index: usize,
                                cardmeta: &[cards::ListCard<BoardStruct>; 180],
                                player_id: usize,
                                board: &mut BoardStruct) {

}
pub fn resolve_purchase_giveables(card_index: usize,
                                  cardmeta: &[cards::ListCard<BoardStruct>; 180],
                                  _p: &mut Player) {
    if let cards::GIVEABLE::VP(_vp) = cardmeta[card_index].purchase_giveables {
        _p.vp += _vp;
    }
}
pub fn resolve_giveable(card_index: usize,
                        cardmeta: &[cards::ListCard<BoardStruct>; 180],
                        player_id: usize,
                        board: &mut BoardStruct,
                        wait_tx: mpsc::Sender<Option<(usize,
                                                      String,
                                                      Vec<(String,
                                                           Box<Fn(&mut Player,
                                                                  &mut Vec<usize>)>)>)>>) {
    if let Some(ref mut z) = board.players.get_mut(player_id) {
        giveable_match(z, player_id, &cardmeta[card_index].giveables, wait_tx);
    }
    //resolve closure
    if let Some(ref _closure) = cardmeta[card_index].giveablefn {
        (*_closure)(board, player_id, card_index);
    }
}
pub fn giveable_match(z: &mut Player,
                      player_id: usize,
                      giveables: &cards::GIVEABLE,
                      wait_tx: mpsc::Sender<Option<(usize,
                                                    String,
                                                    Vec<(String,
                                                         Box<Fn(&mut Player,
                                                                &mut Vec<usize>)>)>)>>) {
    let choose_bet = "Choose between".to_owned();
    match giveables {
        &cards::GIVEABLE::VP(_x) => {
            z.vp += _x;
        }
        &cards::GIVEABLE::COIN(_x) => {
            z.coin += _x;
        }
        &cards::GIVEABLE::VPCOIN(_x1, _x2) => {
            z.vp += _x1;
            z.coin += _x2;
        }
        &cards::GIVEABLE::COININK(_x) => {
            z.coin += _x;
            wait_tx.send(Some((player_id,
                               choose_bet,
                               vec![("Ink".to_owned(),
                                     Box::new(|ref mut p, _| { p.ink += 1; })),
                                    ("Ink Remover".to_owned(),
                                     Box::new(|ref mut p, _| { p.remover += 1; }))])))
                .unwrap();
        }
        &cards::GIVEABLE::VPINK(_x) => {
            z.vp += _x;
            wait_tx.send(Some((player_id,
                               choose_bet,
                               vec![("1 Ink".to_owned(),
                                     Box::new(|ref mut p, _| { p.ink += 1; })),
                                    ("1 Ink Remover".to_owned(),
                                     Box::new(|ref mut p, _| { p.remover += 1; }))])))
                .unwrap();
        }
        &cards::GIVEABLE::NONE => {}
        &cards::GIVEABLE::INK => {
            wait_tx.send(Some((player_id,
                               choose_bet,
                               vec![("1 Ink".to_owned(),
                                     Box::new(|ref mut p, _| { p.ink += 1; })),
                                    ("1 Ink Remover".to_owned(),
                                     Box::new(|ref mut p, _| { p.remover += 1; }))])))
                .unwrap();
        }
        &cards::GIVEABLE::VPORCOIN(_x) => {
            let j1 = format!("{} VP", _x);
            let j2 = format!("{} Coin", _x);
            let _xc = _x.clone();
            let _xcc = _xc.clone();
            wait_tx.send(Some((player_id,
                               choose_bet,
                               vec![(j1, Box::new(move |ref mut p, _| { p.vp += _x; })),
                                    (j2, Box::new(move |ref mut p, _| { p.coin += _x; }))])))
                .unwrap();
        }
        &cards::GIVEABLE::VPORCOININK(_x) => {
            let j1 = format!("{} VP and 1 ink", _x);
            let j2 = format!("{} Coin and 1 ink", _x);
            let j3 = format!("{} VP and 1 ink remover", _x);
            let j4 = format!("{} Coin and 1 ink remover", _x);
            wait_tx.send(Some((player_id,
                               choose_bet,
                               vec![(j1,
                                     Box::new(move |ref mut p, _| {
                                                  p.vp += _x;
                                                  p.ink += 1;
                                              })),
                                    (j2,
                                     Box::new(move |ref mut p, _| {
                                                  p.coin += _x;
                                                  p.ink += 1;
                                              })),
                                    (j3,
                                     Box::new(move |ref mut p, _| {
                                                  p.vp += _x;
                                                  p.remover += 1;
                                              })),
                                    (j4,
                                     Box::new(move |ref mut p, _| {
                                                  p.coin += _x;
                                                  p.remover += 1;
                                              }))])))
                .unwrap();
        }
    }
}
pub fn track_genre(card_index: usize,
                   cardmeta: &[cards::ListCard<BoardStruct>; 180],
                   adv: &mut Vec<usize>,
                   hor: &mut Vec<usize>,
                   mys: &mut Vec<usize>,
                   rom: &mut Vec<usize>) {
    match cardmeta[card_index].genre {
        cards::Genre::ADVENTURE => {
            adv.push(card_index);
        }
        cards::Genre::HORROR => {
            hor.push(card_index);
        }
        cards::Genre::MYSTERY => {
            mys.push(card_index);
        }
        cards::Genre::ROMANCE => {
            rom.push(card_index);
        }
        _ => {}
    }
}
pub fn resolve_genre_giveable(player_id: usize,
                              board: &mut BoardStruct,
                              wait_tx: mpsc::Sender<Option<(usize,
                                                            String,
                                                            Vec<(String,
                                                                 Box<Fn(&mut Player,
                                                                        &mut Vec<usize>)>)>)>>,
                              cardmeta: &[cards::ListCard<BoardStruct>; 180],
                              genre_vec: Vec<&Vec<usize>>) {
    if let Some(ref mut z) = board.players.get_mut(player_id) {
        for _o in genre_vec.clone() {
            if _o.len() >= 2 {
                for &_c in _o {
                    giveable_match(z, player_id, &cardmeta[_c].giveables, wait_tx.clone());
                }
            }

        }

    }
    for _o in genre_vec {
        if _o.len() >= 2 {
            for &_c in _o {
                if let Some(ref _closure) = cardmeta[_c].giveablefn {
                    (*_closure)(board, player_id, _c);
                }
            }

        }
    }
}
pub fn buy_card_from(position_index: usize,
                     from: &mut Vec<usize>,
                     cardmeta: &[cards::ListCard<BoardStruct>; 180],
                     _p: &mut Player,
                     player_id: usize,
                     wait_tx: mpsc::Sender<Option<(usize,
                                                   String,
                                                   Vec<(String,
                                                        Box<Fn(&mut Player,
                                                               &mut Vec<usize>)>)>)>>) {
    let res: Option<Result<(usize, String, Vec<(String, Box<Fn(&mut Player, &mut Vec<usize>)>)>),
                           String>> = match from.get(position_index) {
        Some(&_c) => {
            match cardmeta[_c].cost as f64 <= _p.coin as f64 + (_p.ink as f64 / 3.0).floor() {
                true => {
                    match cardmeta[_c].cost <= _p.coin {
                        true => {
                            _p.coin -= cardmeta[_c].cost;
                            _p.discard.push(from.remove(_c));
                            None
                        }
                        false => {
                            let j = "You do not have enough coin to buy this card, you may trade in 3 ink for one coin to buy this".to_owned();

                            Some(Ok((player_id,
                                     j,
                                     vec![("Trade in 3 ink for one coin to buy this?"
                                               .to_owned(),
                                           Box::new(move |ref mut p, ref mut rmcards| {
                                                        p.discard.push(rmcards.remove(_c));
                                                    })),
                                          ("No".to_owned(),
                                           Box::new(|ref mut p, ref mut rmcards| {}))])))

                        }
                    }
                }
                false => Some(Err("Cannot afford the card".to_owned())),
            }
        }
        None => Some(Err("Cannot find the card selected".to_owned())),
    };
    if let Some(Ok(a)) = res {
        wait_tx.send(Some(a)).unwrap();
    }
}
pub fn buy_card_from_lockup(position_index: usize,
                            cardmeta: &[cards::ListCard<BoardStruct>; 180],
                            _p: &mut Player,
                            player_id: usize,
                            wait_tx: mpsc::Sender<Option<(usize,
                                                          String,
                                                          Vec<(String,
                                                               Box<Fn(&mut Player,
                                                                      &mut Vec<usize>)>)>)>>) {
    let mut card_index = 2;
    if let Some(&_c) = _p.lockup.get(position_index) {
        card_index = _c;
    } else {
        println!("lockup does not have this card");
    }
    let res: Option<Result<(usize, String, Vec<(String, Box<Fn(&mut Player, &mut Vec<usize>)>)>),
                           String>> = match cardmeta[card_index].cost as f64 <=
          _p.coin as f64 + (_p.ink as f64 / 3.0).floor() {
        true => {
            match cardmeta[card_index].cost <= _p.coin {
                true => {
                    _p.coin -= cardmeta[card_index].cost;
                    _p.discard.push(card_index);
                    _p.lockup.remove(position_index);
                    None
                }
                false => {
                    let j = "You do not have enough coin to buy this card, you may trade in 3 ink for one coin to buy this".to_owned();
                    let cost = cardmeta[card_index].cost.clone();
                    Some(Ok((player_id,
                             j,
                             vec![("Trade in 3 ink for one coin to buy this?".to_owned(),
                                   Box::new(move |ref mut p, _| {
                        let coin_left = p.coin;
                        let remainder = cost - coin_left;
                        p.coin = 0;
                        p.ink -= remainder * 3;
                        p.discard.push(card_index);
                        p.lockup.remove(position_index);

                    })),
                                  ("No".to_owned(), Box::new(|ref mut p, _| {}))])))

                }
            }
        }
        false => Some(Err("Cannot afford the card".to_owned())),
    };
    if let Some(Ok(a)) = res {
        wait_tx.send(Some(a)).unwrap();
    }
}
