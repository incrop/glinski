import { CellColor, Field, PieceColor, PieceType } from "./models";

export const generateFiled = (): Field => {
  const field: Field = Array.from(
    Array(11), 
    (_, fileIdx) => Array.from(
      Array(11 - Math.abs(5 - fileIdx)),
      (_, rankIdx) => ({
        fileIdx,
        rankIdx,
        color: ['dark', 'mid', 'light'][(fileIdx + rankIdx + Math.max(0, fileIdx - 5)) % 3] as CellColor,
        piece: undefined,
      })
    ),
  )
  for (let fileIdx = 1; fileIdx <= 9; fileIdx++) {
    field[fileIdx][4 - Math.abs(fileIdx - 5)].piece = {color: 'white', type: 'pawn'}
    field[fileIdx][6].piece = {color: 'black', type: 'pawn'}
  }
  field[5][0].piece = field[5][1].piece = field[5][2].piece = {color: 'white', type: 'bishop'}
  field[5][8].piece = field[5][9].piece = field[5][10].piece = {color: 'black', type: 'bishop'}
  field[2][0].piece = field[8][0].piece = {color: 'white', type: 'rook'}
  field[2][7].piece = field[8][7].piece = {color: 'black', type: 'rook'}
  field[3][0].piece = field[7][0].piece = {color: 'white', type: 'knight'}
  field[3][8].piece = field[7][8].piece = {color: 'black', type: 'knight'}
  field[4][0].piece = {color: 'white', type: 'queen'}
  field[4][9].piece = {color: 'black', type: 'queen'}
  field[6][0].piece = {color: 'white', type: 'king'}
  field[6][9].piece = {color: 'black', type: 'king'}
  return field
}
