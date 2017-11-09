use codec_lib::codec::*;
use game_logic::game_engine::GameCon;
pub fn broadcast<T: GameCon>(gamestates: &mut Vec<GameState>,
                             cons: &Vec<T>,
                             players: &Vec<Player>,
                             log: &mut Vec<ClientReceivedMsg>) {
    for _con in cons.iter() {
        println!("broadcast_show_draft");
        let k: Result<BoardCodec, String> = Ok(BoardCodec {
                                                   players: players.clone(),
                                                   gamestates: gamestates.clone(),
                                                   offer_row: vec![],
                                                   turn_index: 0,
                                                   ticks: None,
                                               });

        let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
        h.set_boardstate(k);
        _con.tx_send(h, log);
    }

}
pub fn give_player_index<T: GameCon>(cons: &Vec<T>, log: &mut Vec<ClientReceivedMsg>) {
    let mut c = 0;
    for _con in cons.iter() {
        let mut h = ClientReceivedMsg::deserialize_receive("{}").unwrap();
        h.set_player_index(c);
        _con.tx_send(h, log);
        c += 1;
    }

}
