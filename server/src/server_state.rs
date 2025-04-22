use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::Message;

use crate::messages::ServerMessage;
use crate::room::{Client, Room};

#[derive(Debug)]
pub struct ServerState {
    pub clients: HashMap<u32, Client>,
    pub next_client_id: u32,
    pub rooms: HashMap<String, Room>,
}

impl ServerState {
    pub fn send_packet(&mut self, client_id: u32, data: ServerMessage) {
        let text = serde_json::to_string(&data).unwrap();
        let client = self.clients.get_mut(&client_id).unwrap();
        client.sender.send(Message::Text(text.into()));
    }

    pub fn view_room(&mut self, client_id: u32, room_id: &str) {
        if !self.rooms.contains_key(room_id) {
            self.rooms.insert(room_id.to_string(), Room::new(room_id.to_string()));
        }

        let room = self.rooms.get_mut(room_id).unwrap();
        room.viewers.push(client_id);
        self.clients.get_mut(&client_id).unwrap().room_id = Some(room.id.clone());

        let packet = ServerMessage::UpdatePlayers { players: room.get_player_updates(), started: room.started };
        self.send_packet(client_id, packet);
    }

    fn join_room(&self, client_id: u32, client_name: String) {
    }
}
