use crate::models::{Board, PieceColor, Moves, PieceType, Coords, Cell, Piece, Move};

pub fn get_available_moves(board: &Board, player: PieceColor, last_move: Option<Move>) -> Vec<Moves> {
  // TODO check
  let mut moves = Vec::new();
  board.iter().for_each(|file| {
    file.iter().for_each(|cell| {
      if let Some(piece) = cell.piece {
        if piece.color == player {
          moves.push(Moves {
            from: cell.coords,
            to: get_available_moves_for_piece(board, piece, cell.coords),
          })
        }
      }
    });
  });
  return moves;
}

fn get_available_moves_for_piece(board: &Board, piece: Piece, coords: Coords) -> Vec<Coords> {
  let mut dest = Vec::new();
  match piece.ptype {
    PieceType::Pawn => {
      if let Some(cell) = coords.offset(0, 1).get_cell(&board) {
        if cell.is_empty() {
          dest.push(cell.coords);
          // Did not move yet
          if coords.rank_idx as i32 == 4 - (coords.file_idx as i32 - 5).abs() {
            if let Some(cell) = coords.offset(0, 2).get_cell(&board) {
              if cell.is_empty() {
                dest.push(cell.coords);
              }
            }
          }
        }
      }
      [coords.offset(1, 1), coords.offset(-1, 1)].iter()
        .filter_map(|coords| coords.get_cell(board))
        .filter(|cell| cell.is_attackable_by(piece.color))
        .for_each(|cell| dest.push(cell.coords))
    }
    _ => {}
  }
  return dest;
}


impl Coords {
  fn offset(&self, file_offset: i32, rank_offset: i32) -> Coords {
    Coords { 
      file_idx: (self.file_idx as i32 + file_offset) as usize, 
      rank_idx: (self.rank_idx as i32 + rank_offset) as usize,
    }
  }
  fn get_cell<'a>(&self, board: &'a Board) -> Option<&'a Cell> {
    if let Some(file) = board.get(self.file_idx) {
      if let Some(cell) = file.get(self.rank_idx) {
        return Some(cell);
      }
    }
    return None;
  }
}

impl Cell {
  fn is_empty(&self) -> bool {
    self.piece.is_none()
  }
  fn is_attackable_by(&self, player: PieceColor) -> bool {
    match self.piece {
      Some(Piece { color, .. }) => color != player,
      None => false,
    }
  }
}
