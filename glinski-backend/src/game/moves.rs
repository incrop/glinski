use crate::models::{Board, PieceColor, Moves, PieceType, Coords, Cell, Piece, Move, GlobalGame};

pub fn get_available_moves(board: &Board, player: PieceColor, last_move: Option<Move>) -> Vec<Moves> {
  // TODO checkmate?
  let previous_player = last_move.map(|Move {to, ..}| {
    board[to.file_idx][to.rank_idx].piece.unwrap().color
  }).unwrap_or(PieceColor::Black);
  if player == previous_player {
    return vec![];
  }
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
      if let Some(cell) = coords.offset(Direction::PohjoinenEtelä, 1).get_cell(&board) {
        if cell.is_empty() {
          dest.push(cell.coords);
          // Did not move yet
          if coords.rank_idx as i32 == 4 - (coords.file_idx as i32 - 5).abs() {
            if let Some(cell) = coords.offset(Direction::PohjoinenEtelä, 2).get_cell(&board) {
              if cell.is_empty() {
                dest.push(cell.coords);
              }
            }
          }
        }
      }
      [coords.offset(Direction::LuodeKaakko, 1), coords.offset(Direction::KoillinenLounas, 1)].iter()
        .filter_map(|coords| coords.get_cell(board))
        .filter(|cell| cell.is_attackable_by(piece.color))
        .for_each(|cell| dest.push(cell.coords))
    }
    _ => {}
  }
  return dest;
}

impl GlobalGame {
  pub fn try_apply_move(&mut self, game_move: Move) -> bool {
    // validate moves?
    
    let Move {to, from} = game_move;
    // TODO: en passant
    self.board[to.file_idx][to.rank_idx].piece = self.board[from.file_idx][from.rank_idx].piece;
    self.board[from.file_idx][from.rank_idx].piece = None;
    self.history.push(game_move);
    return true;
  }
}

enum Direction {
  LuodeKaakko,
  PohjoinenEtelä,
  KoillinenLounas,
}

impl Coords {
  fn offset(&self, direction: Direction, distance: i32) -> Coords {
    if distance == 0 {
      return *self;
    }
    match direction {
      Direction::LuodeKaakko => Coords { 
        rank_idx: {
          let towards_axis = 0.max((self.file_idx as i32 - 5) * distance.signum()).min(distance.abs());
          let right_of_axis = if distance > 0 {
            towards_axis
          } else {
            distance.abs() - towards_axis
          };
          (self.rank_idx as i32 + right_of_axis * distance.signum()) as usize
        },
        file_idx: (self.file_idx as i32 - distance) as usize, 
      },
      Direction::PohjoinenEtelä => Coords {
        file_idx: self.file_idx,
        rank_idx: (self.rank_idx as i32 + distance) as usize,
      },
      Direction::KoillinenLounas => Coords {
        file_idx: (self.file_idx as i32 + distance) as usize, 
        rank_idx: {
          let towards_axis = 0.max((5 - self.file_idx as i32) * distance.signum()).min(distance.abs());
          let left_of_axis = if distance > 0 {
            towards_axis
          } else {
            distance.abs() - towards_axis
          };
          (self.rank_idx as i32 + left_of_axis * distance.signum()) as usize
        }
      }
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
