export type Piece = 'pawn' | 'knight' | 'bishop' | 'rook' | 'queen' | 'king' 
export type PieceColor = 'black' | 'white'
export type CellColor = 'dark' | 'light' | 'mid'

export type Cell = {
  color: CellColor,
  fileIdx: number,
  rankIdx: number,
  piece: Piece | undefined,
}

export type Field = Cell[][]

