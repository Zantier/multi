use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug)]
pub struct Client {
  id: i32,
  room_id: i32,
  ws: SplitSink<WebSocketStream<TcpStream>, Message>,
  name: String,
}
