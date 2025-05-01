mod Player;

use crate::Player;
use crate::structs::gamemode::Gamemode;

pub struct TDM {
    players: Vec<Player>
}

impl Gamemode for TDM {
    fn update(&mut self) {

    }

    fn on_player_join(player_uuid: i32) {

    }
}