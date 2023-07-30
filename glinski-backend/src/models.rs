use std::sync::{Arc, Mutex};

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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Coords {
  pub file_idx: usize,
  pub rank_idx: usize,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Piece {
  pub color: PieceColor,
  pub ptype: PieceType,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Cell {
  pub coords: Coords,
  pub color: CellColor,
  pub piece: Option<Piece>,
}

pub type Board = Vec<Vec<Cell>>;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Move {
  pub from: Coords,
  pub to: Coords,
}

#[derive(Serialize, Deserialize)]
pub struct Moves {
  pub from: Coords,
  pub to: Vec<Coords>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerGame {
  pub player: Option<PieceColor>,
  pub board: Board,
  pub available_moves: Vec<Moves>,
  pub last_move: Option<Move>,
}

pub struct GlobalGame {
  pub sessions: [Option<String>; 2],
  pub board: Board,
  pub history: Vec<Move>,
}

pub struct GameSession {
  pub player: Option<PieceColor>,
  pub game: Arc<Mutex<GlobalGame>>,
}