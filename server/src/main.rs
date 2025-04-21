use futures_util::StreamExt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8088";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");
    let (_write, mut read) = ws_stream.split();

    println!("New WebSocket connection");

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received message: {}", text);
            }
            Ok(Message::Close(_)) => {
                println!("Connection closed");
                break;
            }
            _ => {}
        }
    }
}
