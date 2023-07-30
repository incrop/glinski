mod moves;

use std::{sync::{OnceLock, RwLock, Arc, Mutex}, collections::HashMap, time::{UNIX_EPOCH, SystemTime}};

use crate::models::{GlobalGame, PieceColor, Cell, Coords, CellColor, PieceType, Piece, Board, PlayerGame, GameSession, Move};

use self::moves::get_available_moves;

static ONE_GAME: OnceLock<Arc<Mutex<GlobalGame>>> = OnceLock::new();

static SESSIONS: OnceLock<RwLock<HashMap<String, GameSession>>> = OnceLock::new();

pub fn get_game(uid: &str) -> PlayerGame {
  println!("{}: Getting game", uid);
  let lock = SESSIONS.get_or_init(|| RwLock::new(HashMap::new()));
  let mut sessions_map = lock.write().unwrap();
  let session = sessions_map.entry(uid.to_owned()).or_insert_with(|| {
    let game = ONE_GAME.get_or_init(|| {
      println!("{}: One game initialized", uid);
      Arc::new(Mutex::new(
        GlobalGame {
          sessions: [None, None],
          board: new_board(),
          history: Vec::new(),
        }
      ))
    });
    println!("{}: Session initialized", uid);
    GameSession {
      player: {
        let mut game = game.lock().unwrap();
        pick_player(&mut game.sessions, uid)
      },
      game: game.clone(),
    }
  });
  return session.get_player_game()
}

pub fn handle_move(uid: &str, game_move: Move) -> bool {
  let lock = SESSIONS.get_or_init(|| RwLock::new(HashMap::new()));
  let sessions_map = lock.write().unwrap();
  let session = sessions_map.get(uid).unwrap();
  let mut game = session.game.lock().unwrap();
  let player = session.player.unwrap_or(PieceColor::White);
  return game.try_apply_move(adjust_last_move(game_move, player))
}

impl GameSession {
  fn get_player_game(&self) -> PlayerGame {
    println!("Getting player game for {:?}", self.player);
    let game: std::sync::MutexGuard<'_, GlobalGame> = self.game.lock().unwrap();
    let player = self.player.unwrap_or(PieceColor::White);
    let board = get_player_board(&game.board, player);
    let last_move = game.history.last().copied().map(|m| adjust_last_move(m, player));
    let available_moves = match self.player {
      None => vec![],
      Some(player) => get_available_moves(&board, player, last_move),
    };
    println!("Player moves for {:?}: {}", self.player, available_moves.len());
    return PlayerGame { 
      player: self.player, 
      board, 
      available_moves,
      last_move, 
    }
  }
}

fn adjust_last_move(last_move: Move, player: PieceColor) -> Move {
  match player {
    PieceColor::White => last_move,
    PieceColor::Black => Move { 
      from: last_move.from.rotate_board(),
      to: last_move.to.rotate_board(),
    },
  }
}

impl Coords {
  fn rotate_board(&self) -> Coords {
    let file_idx = 10 - self.file_idx;
    let file_len = (11 - (5 - file_idx as i32).abs()) as usize;
    let rank_idx = file_len - self.rank_idx - 1;
    Coords {
      file_idx,
      rank_idx,
    }
  }
}

fn get_player_board(board: &Board, player: PieceColor) -> Board {
  (0..11).map(|file_idx| {
    let file_len = 11 - (5i32 - file_idx).abs();
    let orig_file_idx = if player == PieceColor::White { file_idx } else { 10 - file_idx };
    (0..file_len).map(|rank_idx| {
      let orig_rank_idx = if player == PieceColor::White { rank_idx } else { file_len - rank_idx - 1 };
      let mut cell = board[orig_file_idx as usize][orig_rank_idx as usize];
      cell.coords = Coords { 
        file_idx: file_idx as usize, 
        rank_idx: rank_idx as usize,
      };
      cell
    }).collect()
  }).collect()
}

fn pick_player(sessions: &mut [Option<String>; 2], session_id: &str) -> Option<PieceColor> {
  let init_sessions = sessions.iter().filter(|s| s.is_some()).count();
  let idx = match init_sessions {
    0 => Some((SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() % 2) as usize),
    1 => Some(sessions.iter().position(|s| s.is_none()).unwrap()),
    _ => None,
  };
  idx.map(|idx| {
    sessions[idx] = Some(session_id.to_owned());
    if idx == 0 {
      PieceColor::White
    } else {
      PieceColor::Black
    }
  })
}

fn new_board() -> Board {
  let mut board: Board = (0..11).map(|file_idx| {
    let file_len = 11 - (5i32 - file_idx).abs();
    (0..file_len).map(|rank_idx| {
      let cell_color_idx = (file_idx + rank_idx + 0.max(file_idx - 5)) % 3;
      Cell {
        coords: Coords {
          file_idx: file_idx as usize,
          rank_idx: rank_idx as usize,
        },
        color: [CellColor::Dark, CellColor::Mid, CellColor::Light][cell_color_idx as usize],
        piece: None,
      }
    }).collect()
  }).collect();
  (1..10).for_each(|file_idx| {
    board[file_idx][(4 - (file_idx as i32 - 5).abs()) as usize].piece = Some(Piece {
      color: PieceColor::White,
      ptype: PieceType::Pawn,
    });
    board[file_idx][6].piece = Some(Piece {
      color: PieceColor::Black,
      ptype: PieceType::Pawn,
    });
  });
  let white_bishop = Some(Piece {
    color: PieceColor::White,
    ptype: PieceType::Bishop,
  });
  board[5][0].piece = white_bishop;
  board[5][1].piece = white_bishop;
  board[5][2].piece = white_bishop;
  let black_bishop = Some(Piece {
    color: PieceColor::Black,
    ptype: PieceType::Bishop,
  });
  board[5][8].piece = black_bishop;
  board[5][9].piece = black_bishop;
  board[5][10].piece = black_bishop;
  let white_rook = Some(Piece {
    color: PieceColor::White,
    ptype: PieceType::Rook,
  });
  board[2][0].piece = white_rook;
  board[8][0].piece = white_rook;
  let black_rook = Some(Piece {
    color: PieceColor::Black,
    ptype: PieceType::Rook,
  });
  board[2][7].piece = black_rook;
  board[8][7].piece = black_rook;
  let white_knight = Some(Piece {
    color: PieceColor::White,
    ptype: PieceType::Knight,
  });
  board[3][0].piece = white_knight;
  board[7][0].piece = white_knight;
  let black_knight = Some(Piece {
    color: PieceColor::Black,
    ptype: PieceType::Knight,
  });
  board[3][8].piece = black_knight;
  board[7][8].piece = black_knight;
  board[4][0].piece = Some(Piece {
    color: PieceColor::White,
    ptype: PieceType::Queen,
  });
  board[4][9].piece = Some(Piece {
    color: PieceColor::Black,
    ptype: PieceType::Queen,
  });
  board[6][0].piece = Some(Piece {
    color: PieceColor::White,
    ptype: PieceType::King,
  });
  board[6][9].piece = Some(Piece {
    color: PieceColor::Black,
    ptype: PieceType::King,
  });
  return board;
}