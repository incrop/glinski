use crate::models::{Game, PieceColor, Cell, Coords, CellColor, PieceType, Piece};

pub fn get_game() -> Game {
  Game {
    player: Some(PieceColor::White),
    board: new_board(),
    last_move: None,
  }
}

fn new_board() -> Vec<Vec<Cell>> {
  let mut board: Vec<Vec<Cell>> = (0..11).map(|file_idx| {
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