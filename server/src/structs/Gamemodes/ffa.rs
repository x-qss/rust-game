mod Player;
use crate::Player;
use crate::structs::gamemode::Gamemode;

pub struct FFA {
    players: Vec<Player>,
}

impl Gamemode for FFA {
    fn update(&mut self) {

    }

    fn on_player_join(&mut self, uuid: i32) {

    }
}