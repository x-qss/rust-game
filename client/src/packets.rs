use serde::{Serialize, Deserialize};

use crate::structs::player::Player;

// packets our client can send to the server
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientPackets {
    Ping,
    Aim { direction: f32 },
    Move { direction: f32 },
    Spawn { username: String },
}

// packets we receive from the server
pub enum IncomingPackets {
    AddPlayer { data: Player },
    UpdatePlayers { data: Vec<Player> },
}
