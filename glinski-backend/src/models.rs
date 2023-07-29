use serde_derive::{Serialize, Deserialize};


#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub enum PieceType {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub enum PieceColor {
  Black,
  White,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub enum CellColor {
  Dark,
  Light,
  Mid,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Coords {
  pub file_idx: usize,
  pub rank_idx: usize,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Piece {
  pub color: PieceColor,
  pub ptype: PieceType,
}

#[derive(Serialize, Deserialize)]
pub struct Cell {
  pub coords: Coords,
  pub color: CellColor,
  pub piece: Option<Piece>,
}

#[derive(Serialize, Deserialize)]
pub struct Move {
  pub from: Coords,
  pub to: Coords,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
  pub player: Option<PieceColor>,
  pub board: Vec<Vec<Cell>>,
  pub last_move: Option<Move>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
  GameState { data: Game },
}