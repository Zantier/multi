use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "view-room")]
    ViewRoom { id: String },

    #[serde(rename = "join-room")]
    JoinRoom { name: String },

    #[serde(rename = "leave-room")]
    LeaveRoom {},

    #[serde(rename = "pick-cards")]
    PickCards { cards: Vec<usize> },

    #[serde(rename = "start-game")]
    StartGame {},

    #[serde(rename = "heartbeat")]
    Heartbeat {},

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "update-players")]
    UpdatePlayers { players: Vec<PlayerUpdate>, started: bool },

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerUpdate {
    pub name: String,
    pub score: i32,
    pub minus_score: i32,
    pub timeout: i32,
    pub connected: bool,
}
