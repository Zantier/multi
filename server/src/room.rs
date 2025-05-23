use futures_util::stream::SplitSink;
use rand;
use rand::seq::SliceRandom;
use std::cmp::{max,min};
use std::collections::HashSet;
use std::iter;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

use crate::card::{check_match, get_third_card, id_to_card, Card};
use crate::messages::{PickCards, PlayerUpdate, ServerMessage};
use crate::util::{self, get_now};

const COLOR_EXPIRE_MS: i32 = 5000;
const TIMEOUT_MS: i32 = 10000;

#[derive(Debug)]
pub struct Client {
    pub id: u32,
    pub room_id: Option<String>,
    pub name: Option<String>,
    pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub client_id: Option<u32>,
    pub name: String,
    pub score: i32,
    pub minus_score: i32,
    pub timeout: i32,
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

    pub fn get_player_index(&self, client_id: u32) -> Option<u32> {
        for i in 0..self.players.len() {
            if self.players[i].client_id == Some(client_id) {
                return Some(i as u32);
            }
        }

        return None;
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
        self.delete_time = None;
    }

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

    pub fn get_update_game_packet(&self, player_index: Option<u32>) -> ServerMessage {
        let packet = ServerMessage::UpdateGame {
            players: self.get_player_updates(),
            cards: self.cards.iter().map(|card| card.as_ref().map(|card| card.id)).collect(),
            wrong: self.wrong.to_vec(),
            correct: self.correct.to_vec(),
            game_over: self.game_over,
            start_time: self.start_time - util::get_now(),
        };

        if let Some(_player_index) = player_index {
            // Add playerState
        }

        packet
    }

    pub fn start(&mut self) {
        self.started = true;
        self.start_time = util::get_now() + 3000;
        // Attributes: color (3), amount (3), shape (3), fill (3)
        let mut card_ids: Vec<_> = (0..81).collect();
        card_ids.shuffle(&mut rand::rng());
        self.cards_left = card_ids
            .iter()
            .map(|&id| id_to_card(id))
            .collect();
        self.cards = iter::repeat(None).take(12).collect();
        self.add_cards();
    }

    pub fn pick_cards(&mut self, client_id: u32, card_indexes: &[usize]) {
        let player_index = self.get_player_index(client_id).unwrap();

        let is_match = check_match(&self.cards[card_indexes[0]], &self.cards[card_indexes[1]], &self.cards[card_indexes[2]]);
        if is_match {
            // Check that somebody else didn't already use any of the cards
            for pick in self.correct.iter() {
                for i in 0..3 {
                    if pick.cards.contains(&card_indexes[i]) {
                        return;
                    }
                }
            }
        }

        if is_match {
            self.correct.push(PickCards {
                player: player_index,
                cards: card_indexes.to_vec(),
                expire:  get_now() + COLOR_EXPIRE_MS,
            });
            self.players[player_index as usize].score += 1;
        } else {
            self.wrong.push(PickCards {
                player: player_index,
                cards: card_indexes.to_vec(),
                expire:  get_now() + COLOR_EXPIRE_MS,
            });
            self.players[player_index as usize].minus_score += 1;
            self.players[player_index as usize].timeout = get_now() + TIMEOUT_MS;
        }
    }
}
