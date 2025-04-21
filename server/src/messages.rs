use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")] // Uses "type" field to determine the variant
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
