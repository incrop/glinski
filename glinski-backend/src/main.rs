mod game;
mod models;

use std::env;
use std::fs;
use std::path::Path;
use std::thread;

use rouille::Response;
use rouille::session;
use rouille::try_or_400;
use rouille::websocket;
use rouille::input;

use crate::models::ServerMessage;


fn main() {
  let assets_path = env::var("ASSETS_DIR").unwrap_or(".".to_owned());
  rouille::start_server("0.0.0.0:8080", move |request| {
    println!("Request URL: {}", request.url());
    if request.url() == "/" {
      let mut response = serve_index(format!("{}/index.html", &assets_path));
      if input::cookies(request).all(|(name, _)| name != "SID") {
        let session_id = session::generate_session_id();
        println!("Generate session_id {}", session_id);
        let header_value = format!("SID={}; Max-Age=31536000; Path=/;", session_id);
        response.headers.push(("Set-Cookie".into(), header_value.into()));
      }
      return response;
    }
    if request.url() == "/ws" {
      let (response, websocket) = try_or_400!(websocket::start(&request, Some("chess")));
      thread::spawn(move || {
        let ws = websocket.recv().unwrap();
        websocket_handling_thread(ws);
      });
      return response;
    }
    return rouille::match_assets(request, &assets_path);
  });
}

fn serve_index(index_path: impl AsRef<Path>) -> Response {
  let file = match fs::File::open(index_path) {
    Ok(f) => f,
    Err(_) => return Response::empty_404(),
  };

  return Response::from_file("text/html; charset=utf8", file)
        .with_public_cache(3600)
}

fn websocket_handling_thread(mut websocket: websocket::Websocket) {
  let Some(websocket::Message::Text(session_id)) = websocket.next() else { panic!() };
  println!("Opened websocket for session {}", &session_id);
  let game = game::get_game(&session_id);
  let game_state = ServerMessage::GameState {
    data: game,
  };
  let text = &serde_json::to_string(&game_state).unwrap();
  websocket.send_text(text).unwrap();
  while let Some(message) = websocket.next() {
    match message {
      websocket::Message::Text(txt) => {
        println!("received {:?} from a websocket", txt);
      }
      websocket::Message::Binary(_) => {
        println!("received binary from a websocket");
      }
    }
  }
  println!("Closed websocket for session {}", &session_id);
}

