use futures_util::SinkExt;
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::Message;

use crate::messages::ServerMessage;
use crate::room::{Client, Player, Room};

#[derive(Debug)]
pub struct ServerState {
    pub clients: HashMap<u32, Client>,
    pub next_client_id: u32,
    pub rooms: HashMap<String, Room>,
}

impl ServerState {
    pub async fn send_packet(&mut self, client_id: u32, data: ServerMessage) {
        let text = serde_json::to_string(&data).unwrap();
        let client = self.clients.get_mut(&client_id).unwrap();
        _ = client.sender.send(Message::Text(text.into())).await;
    }

    pub async fn view_room(&mut self, client_id: u32, room_id: &str) {
        if !self.rooms.contains_key(room_id) {
            self.rooms.insert(room_id.to_string(), Room::new(room_id.to_string()));
        }

        let room = self.rooms.get_mut(room_id).unwrap();
        room.viewers.push(client_id);
        self.clients.get_mut(&client_id).unwrap().room_id = Some(room.id.clone());

        let packet = ServerMessage::UpdatePlayers { players: room.get_player_updates(), started: room.started };
        self.send_packet(client_id, packet).await;
    }

    pub async fn join_room(&mut self, client_id: u32, client_name: String) {
        let client = self.clients.get_mut(&client_id).unwrap();
        client.name = Some(client_name);
        let room = self.rooms.get_mut(client.room_id.as_ref().unwrap()).unwrap();
        let mut success = false;
        if room.started {
            for player in room.players.iter_mut() {
                if player.client_id.is_none() && Some(&player.name) == client.name.as_ref() {
                    player.client_id = Some(client.id);
                    success = true;
                }
            }
        } else {
            success = true;
            for player in room.players.iter() {
                if Some(&player.name) == client.name.as_ref() {
                    success = false;
                }
            }

            if success {
                room.viewers = room.viewers
                    .iter()
                    .map(|&x| x)
                    .filter(|&id| id != client.id)
                    .collect();
                room.add_player(Player { client_id: Some(client.id), name: client.name.clone().unwrap(), score: 0, minus_score: 0, timeout: 0 });
            }
        }

        if success {
            client.room_id = Some(room.id.to_string());
            if room.started {
                let player_index = room.get_player_index(client.id);
                let packet = room.get_update_game_packet(player_index);
                self.send_packet(client_id, packet).await;
            }

            let client = self.clients.get_mut(&client_id).unwrap();
            let room_id = client.room_id.clone().unwrap();
            self.send_update_players(&room_id, true, true).await;
        } else {
            self.send_packet(client_id, ServerMessage::RejectJoinGame {}).await;
        }
    }

    pub async fn leave_view_room(&mut self, client_id: u32) {
        let client = self.clients.get(&client_id).unwrap();

        if let Some(room_id) = client.room_id.clone() {
            self.leave_room(client_id);
            self.view_room(client_id, &room_id).await;

            let room = self.rooms.get_mut(&room_id).unwrap();
            room.check_empty();
            self.send_update_players(&room_id, true, true).await;
        }
    }

    pub async fn disconnect(&mut self, client_id: u32) {
        let client = self.clients.get(&client_id).unwrap();

        if let Some(room_id) = client.room_id.clone() {
            self.leave_room(client_id);

            let room = self.rooms.get_mut(&room_id).unwrap();
            room.check_empty();
            self.send_update_players(&room_id, true, true).await;
        }
    }

    pub fn leave_room(&mut self, client_id: u32) {
        let client = self.clients.get_mut(&client_id).unwrap();
        if let Some(room) = self.rooms.get_mut(client.room_id.as_ref().unwrap()) {
            client.room_id = None;
            room.remove_client(client.id);
        }
    }

    pub async fn send_update_players(&mut self, room_id: &str, send_to_players: bool, send_to_viewers: bool) {
        let room = &self.rooms[room_id];
        let packet = ServerMessage::UpdatePlayers {
            players: room.get_player_updates(),
            started: room.started,
        };
        if send_to_players {
            for player in room.players.to_vec() {
                if let Some(client_id) = player.client_id {
                    self.send_packet(client_id, packet.clone()).await;
                }
            }
        }
        if send_to_viewers {
            let room = &self.rooms[room_id];
            for client_id in room.viewers.to_vec() {
                self.send_packet(client_id, packet.clone()).await;
            }
        }
    }

    pub async fn start_game(&mut self, client_id: u32) {
        let client = self.clients.get(&client_id).unwrap();
        let room_id = client.room_id.clone().unwrap();
        let room = self.rooms.get_mut(&room_id).unwrap();
        room.start();

        self.send_update_game_all(&room_id).await;
        self.send_update_players(&room_id, false, true).await;
    }

    pub async fn pick_cards(&mut self, client_id: u32, card_indexes: &[usize]) {
        let client = self.clients.get(&client_id).unwrap();
        let room_id = client.room_id.clone().unwrap();
        let room = self.rooms.get_mut(&room_id).unwrap();
        room.pick_cards(client_id, card_indexes);

        self.send_update_game_all(&room_id).await;
    }

    async fn send_update_game_all(&mut self, room_id: &str) {
        let room = &self.rooms[room_id];
        let data = room.get_update_game_packet(None);

        let client_ids: Vec<_> = room.players.iter()
            .filter_map(|player| player.client_id)
            .collect();

        for client_id in client_ids {
            self.send_packet(client_id, data.clone()).await;
        }
    }
}
