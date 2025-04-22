use futures_util::stream::SplitSink;
use rand;
use std::cmp::{max,min};
use std::collections::HashSet;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

use crate::card::{get_third_card, id_to_card, Card};
use crate::messages::PlayerUpdate;
use crate::util;

#[derive(Debug)]
pub struct Client {
    pub id: u32,
    pub room_id: Option<String>,
    pub name: Option<String>,
    pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
}

#[derive(Debug)]
pub struct Player {
    pub client_id: Option<u32>,
    pub name: String,
    pub score: i32,
    pub minus_score: i32,
    pub timeout: i32,
}

#[derive(Debug)]
pub struct PickCards {
    pub player: i32,
    pub cards: Vec<i32>,
    pub expire: i32,
}

#[derive(Debug)]
pub struct Room {
    pub id: String,
    pub viewers: Vec<u32>,
    pub players: Vec<Player>,
    pub started: bool,
    pub start_time: i32,
    pub cards_left: Vec<Card>,
    pub cards: Vec<Option<Card>>,
    /// Picked cards that are correct
    pub correct: Vec<PickCards>,
    /// Picked cards that are wrong
    pub wrong: Vec<PickCards>,
    pub game_over: bool,
    /// When the room should be deleted. Undefined means there
    /// are still players in the room.
    pub delete_time: Option<i32>,
}

impl Room {
    pub fn get_player_updates(&self) -> Vec<PlayerUpdate> {
        let mut players = Vec::new();
        let now = util::get_now();
        for player in self.players.iter() {
            players.push(PlayerUpdate {
                name: player.name.clone(),
                score: player.score,
                minus_score: player.minus_score,
                connected: player.client_id.is_some(),
                timeout: player.timeout - now,
            });
        }

        players
    }

    pub fn add_cards(&mut self) {
        let mut missing_indexes = Vec::<usize>::new();
        for (i, card) in self.cards.iter().enumerate() {
            if card.is_none() {
                missing_indexes.push(i);
            }
        }

        if missing_indexes.len() == 0 {
            return;
        }

        if self.cards_left.len() <= missing_indexes.len() {
            for (i, card) in self.cards.iter_mut().enumerate() {
                if card.is_none() {
                    if let Some(card_left) = self.cards_left.pop() {
                        *card = Some(card_left);
                        missing_indexes.push(i);
                    }
                }
            }

            self.game_over = !self.get_has_solution();
            return;
        }

        // Replace this card if no solution
        let replace_index = missing_indexes[rand::random_range(0..missing_indexes.len())];

        for card in self.cards.iter_mut() {
            if card.is_none() {
                if let Some(card_left) = self.cards_left.pop() {
                    *card = Some(card_left);
                }
            }
        }

        if self.get_has_solution() {
            return;
        }

        // There is no solution yet, so replace a card to force one

        let mut index1 = rand::random_range(0..11);
        if index1 >= replace_index {
            index1 += 1;
        }
        let mut index2 = rand::random_range(0..10);
        if index2 >= min(replace_index, index1) {
            index2 += 1;
        }
        if index2 >= max(replace_index, index1) {
            index2 += 1;
        }

        let third_card_id = get_third_card(self.cards[index1].as_ref().unwrap(), self.cards[index2].as_ref().unwrap());
        self.cards[replace_index] = Some(id_to_card(third_card_id));
        println!("Set card {replace_index} = {index1} + {index2}");

        for i in 0..self.cards_left.len() {
            if self.cards_left[i].id == third_card_id {
                self.cards_left.remove(i);
                break;
            }
        }
        return;
    }

    /// Get if the current cards have a solution
    fn get_has_solution(&self) -> bool {
        let mut ids = HashSet::new();
        for card in self.cards.iter() {
            if let Some(card) = card {
                ids.insert(card.id);
            }
        }

        let mut result = false;
        for (i, card_i) in self.cards.iter().enumerate() {
            if let Some(card_i) = card_i {
                for card_j in self.cards.iter().skip(i + 1) {
                    if let Some(card_j) = card_j {
                        let id = get_third_card(card_i, card_j);
                        if ids.contains(&id) {
                            result = true;
                        }
                    }
                }
            }
        }

        return result;
    }
}

impl Room {
    pub fn new(id: String) -> Room {
        Room {
            id,
            viewers: Vec::new(),
            players: Vec::new(),
            started: false,
            start_time: 0,
            cards_left: Vec::new(),
            cards: Vec::new(),
            correct: Vec::new(),
            wrong: Vec::new(),
            game_over: false,
            delete_time: None,
        }
    }
}
