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

/// This packet is only sent if the game has a start time
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "reject-join-game")]
    RejectJoinGame {},

    #[serde(rename = "update-game")]
    UpdateGame {
        players: Vec<PlayerUpdate>,
        cards: Vec<Option<i32>>,
        wrong: Vec<PickCards>,
        correct: Vec<PickCards>,
        game_over: bool,
        start_time: i32,
    },

    #[serde(rename = "update-players")]
    UpdatePlayers { players: Vec<PlayerUpdate>, started: bool },

    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerUpdate {
    pub name: String,
    pub score: i32,
    pub minus_score: i32,
    pub timeout: i32,
    pub connected: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PickCards {
    pub player: i32,
    pub cards: Vec<i32>,
    pub expire: i32,
}
