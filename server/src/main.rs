use futures_util::{SinkExt, StreamExt};
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, sleep};
use tokio_tungstenite::accept_async;

mod card;
mod messages;
use messages::ClientMessage;
mod room;
use room::{Client, Room};
mod server_state;
use server_state::ServerState;
mod util;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8088";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on {addr}");

    let state = Arc::new(Mutex::new(ServerState {
        clients: HashMap::new(),
        next_client_id: 0,
        rooms: HashMap::new(),
    }));

    {
        let state = state.clone();
        tokio::spawn(tick(state));
    }

    while let Ok((stream, _)) = listener.accept().await {
        let state = state.clone();
        tokio::spawn(handle_connection(state, stream));
    }
}

fn check_delete_room<'a>(state: &'a mut ServerState, room_id: &str) -> Option<&'a Room> {
    let now = util::get_now();
    let room = &state.rooms[room_id];

    if let Some(delete_time) = room.delete_time {
        if now < delete_time {
            return None;
        }

        let room_id = room.id.clone();
        if room.viewers.len() == 0 {
            state.rooms.remove(&room_id);
            return None;
        } else {
            let mut new_room = Room::new(room_id.clone());
            for &viewer in room.viewers.iter() {
                new_room.viewers.push(viewer);
            }
            state.rooms.insert(room_id.clone(), new_room);
            return state.rooms.get(&room_id);
        }
    }

    None
}

async fn tick(state: Arc<Mutex<ServerState>>) {
    loop {
        {
            let mut state = state.lock().unwrap();
            let now = util::get_now();
            let room_ids = state.rooms.keys().cloned().collect::<Vec<_>>();
            for room_id in room_ids {
                // TODO: See if the room needs deleting
                //let new_room = check_delete_room(&mut state, room_id);
                //if let Some(new_room) = new_room {
                //    send_update_players(&mut state.clients, &mut new_room, true, true);
                //    continue;
                //}

                let room = state.rooms.get_mut(&room_id).unwrap();
                let mut updated = false;
                let mut i = 0;
                while i < room.wrong.len() {
                    if now > room.wrong[i].expire {
                        room.wrong.remove(i);
                        updated = true;
                    } else {
                        i += 1;
                    }
                }

                let mut i = 0;
                while i < room.correct.len() {
                    if now > room.correct[i].expire {
                        for &card_index in &room.correct[i].cards {
                            room.cards[card_index as usize] = None;
                        }
                        room.correct.remove(i);
                        updated = true;
                    } else {
                        i += 1;
                    }
                }

                room.add_cards();
                if updated {
                    // TODO: send update
                    //send_update_game_all(room);
                }
            }
        }

        sleep(Duration::from_millis(100)).await;
    }
}

async fn handle_connection(state: Arc<Mutex<ServerState>>, stream: TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");
    let (write, mut read) = ws_stream.split();

    println!("New WebSocket connection");

    let client_id = {
        let mut state = state.lock().unwrap();
        let client = Client {
            id: state.next_client_id,
            room_id: None,
            name: None,
            sender: write,
        };
        state.clients.insert(client.id, client);
        state.next_client_id += 1;
        state.next_client_id - 1
    };

    while let Some(raw_result) = read.next().await {
        match raw_result {
            Ok(raw_message) => {
                if let Ok(raw_message) = raw_message.to_text() {
                    if raw_message == "" {
                        println!("Empty message");
                        break;
                    }

                    match serde_json::from_str::<ClientMessage>(raw_message) {
                        Ok(message) => {
                            handle_message(&state, message, client_id).await;
                        }
                        Err(e) => {
                            eprintln!("Error {e}");
                            eprintln!("Received invalid JSON: {raw_message}");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {e}");
                break;
            }
        }
    }

    println!("Client disconnected");
    let mut client = {
        let mut state = state.lock().unwrap();
        state.clients.remove(&client_id).unwrap()
    };
    _ = client.sender.close().await;
}

async fn handle_message(state: &Arc<Mutex<ServerState>>, message: ClientMessage, client_id: u32) {
    let mut state = state.lock().unwrap();

    match message {
        ClientMessage::ViewRoom { id } => {
            println!("[{client_id}] ViewRoom {id}");
            state.view_room(client_id, &id).await;
        }
        ClientMessage::JoinRoom { name } => {
            println!("[{client_id}] JoinRoom {name}");
            state.join_room(client_id, name).await;
        }
        ClientMessage::LeaveRoom {} => {
            println!("[{client_id}] Client left the room");
            // TODO: add logic
        }
        ClientMessage::PickCards { cards } => {
            println!("[{client_id}] Client picked cards: {cards:?}");
            // TODO: add logic
        }
        ClientMessage::StartGame {} => {
            println!("[{client_id}] Game start requested");
            // TODO: add logic
        }
        ClientMessage::Heartbeat {} => {
            // Used to keep the connection alive
        }
        ClientMessage::Unknown => {
            println!("[{client_id}] Unknown message received");
        }
    }
}
