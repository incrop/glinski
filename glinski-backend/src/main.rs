mod game;
mod models;

use std::env;
use std::sync::atomic::AtomicUsize;
use std::sync::OnceLock;
use std::collections::HashMap;
use std::sync::atomic::Ordering;

use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use futures_util::StreamExt;
use models::PlayerGame;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;
use warp::ws::Message;
use warp::ws::WebSocket;

use crate::models::Move;

static NEXT_CONN_NUM: AtomicUsize = AtomicUsize::new(1);
static CONNECTIONS: OnceLock<RwLock<HashMap<usize, mpsc::UnboundedSender<()>>>> = OnceLock::new();

#[tokio::main]
async fn main() {
  let assets_path = env::var("ASSETS_DIR").unwrap_or(".".to_owned());
  let serve_static = warp::fs::dir(assets_path);
  let serve_websocket = warp::path("ws")
    .and(warp::ws())
    .map(|ws: warp::ws::Ws| {
      ws.on_upgrade(move |socket| handle_connection(socket))
    });
  warp::serve(serve_websocket.or(serve_static))
        .run(([127, 0, 0, 1], 8080))
        .await;
}

async fn handle_connection(ws: warp::ws::WebSocket) {
  let conn_num = NEXT_CONN_NUM.fetch_add(1, Ordering::Relaxed);
  let (mut ws_tx, mut ws_rx) = ws.split();

  let uid = ws_rx.next().await.unwrap().unwrap();
  let uid = uid.to_str().unwrap().to_owned();
  println!("{}({}): Opened websocket", uid, conn_num);

  send_game_state(&mut ws_tx, &game::get_game(&uid)).await;

  let (tx, rx) = mpsc::unbounded_channel();
  let mut rx = UnboundedReceiverStream::new(rx);
  let uid_clone = uid.clone();
  tokio::task::spawn(async move {
    while let Some(_) = rx.next().await {
      println!("{}({}): Sending game state", uid_clone, conn_num);
      send_game_state(&mut ws_tx, &game::get_game(&uid_clone)).await;
      println!("{}({}): Sent game state", uid_clone, conn_num);
    }
  });

  CONNECTIONS.get_or_init(|| RwLock::default()).write().await.insert(conn_num, tx);
  
  while let Some(message) = ws_rx.next().await {
    let message = message.unwrap();
    if let Ok(txt) = message.to_str() {
      let game_move: Move = serde_json::from_str(txt).unwrap();
      println!("{}({}): Received move", uid, conn_num);
      if game::handle_move(&uid, game_move) {
        for (_, tx) in CONNECTIONS.get().unwrap().read().await.iter() {
          tx.send(()).unwrap();
        }
      }
    }
  }

  CONNECTIONS.get().unwrap().write().await.remove(&conn_num);
  println!("{}({}): Closed websocket", uid, conn_num);
}

async fn send_game_state(ws_tx: &mut SplitSink<WebSocket, Message>, game: &PlayerGame) {
  let text = &serde_json::to_string(&game).unwrap();
  let message = warp::ws::Message::text(text);
  ws_tx.send(message).await.unwrap()
}
