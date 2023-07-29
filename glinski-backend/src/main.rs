mod game;
mod models;

use std::env;
use std::fs;
use std::path::Path;
use std::thread;

use models::Message;
use rouille::Response;
use rouille::try_or_400;
use rouille::websocket;


fn main() {
  let assets_path = env::var("ASSETS_DIR").unwrap_or(".".to_owned());
  rouille::start_server("0.0.0.0:8080", move |request| {
    if request.url() == "/ws" {
      let (response, websocket) = try_or_400!(websocket::start(&request, Some("chess")));
      thread::spawn(move || {
          let ws = websocket.recv().unwrap();
          websocket_handling_thread(ws);
      });
      return response;
    }
    if request.url() == "/" {
      return serve_index(format!("{}/index.html", &assets_path));
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
  let game = game::get_game();
  let game_state = Message::GameState {
    data: game,
  };
  let text = &serde_json::to_string(&game_state).expect("json error");
  websocket.send_text(text).expect("websocket error");
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
}

