use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Packets {
  Chat { user: String, content: String },
  Move { direction: f32 },
  Ping,
  Spawn { username: String }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientPackets {
  Ping,
  AddPlayer
}

/*

packets will ALWAYS have a type attribute.

e.g. json structure for incoming data:
{
  type: "Chat",
  user: "Username",
  content: "hello world"
}

type should be 1:1 with packet types here

*/