export type PieceType = 'pawn' | 'knight' | 'bishop' | 'rook' | 'queen' | 'king' 
export type PieceColor = 'black' | 'white'
export type CellColor = 'dark' | 'light' | 'mid'

export type Coords = {
  file_idx: number,
  rank_idx: number,
}
export type Piece = {
  color: PieceColor,
  ptype: PieceType,
}

export type Cell = {
  coords: Coords,
  color: CellColor,
  piece: Piece | null,
}

export type Board = Cell[][]

export type Move = {
  from: Coords,
  to: Coords,
}

export type Moves = {
  from: Coords,
  to: Coords[],
}

export type Game = {
  player: PieceColor | null,
  board: Board,
  available_moves: Moves[],
  last_move: Move | null,
  selected?: Moves,
}

export type Message = {
  type: 'GameState',
  data: any,
}