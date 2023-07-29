import { CellColor, PieceType, PieceColor, Board, Game, Coords } from "./models";
import { shapeScale, shapes } from "./shapes";

const padding = 10
const outlineWidth = 4
const outlineColorRGB = '#000000'
const selectedCellOverlayRGBA = 'rgba(0, 255, 0, 0.2)'

const cellColorRGB = (cellColor: CellColor): string => 
  ({
    dark: '#d18b47',
    light: '#ffce9e',
    mid: '#e8ab6f',
  })[cellColor]
  
// Base hexagonal cell
//
//     ***********
//     ***********
//    **         **
//    **         **
//   **           **
//   **           **
//  **             **
//  **             **
// **               **
// **               **-
//  **             ** |
//  **             ** |
//   **           **  |
//   **           **  |
//    **         **   |
//    **         **   |
//     ***********    -
//     ***********    c
//     a|-------||--|b
const a = 9
const b = 4
const c = 8

export const drawBoard = (ctx: CanvasRenderingContext2D, game: Game) => {
  const heightBase = 11 * c * 2
  const widthBase = 11 * (a + b) + b
  const maxSize = ctx.canvas.clientHeight - padding * 2
  const m = maxSize / heightBase
  const offsetX = padding + Math.floor((maxSize - m * widthBase) / 2)
  const offsetY = padding + maxSize - Math.ceil((maxSize - m * heightBase) / 2)

  ctx.fillStyle = outlineColorRGB
  ctx.resetTransform()
  game.board.forEach((file, fileIdx) => {
    const cellOffsetX = offsetX + m * fileIdx * (a + b)
    const fileOffsetY = offsetY - m * c * Math.abs(5 - fileIdx)
    file.forEach((_, rankIdx) => {
      if (fileIdx > 0 && fileIdx < 10 && rankIdx > 0 && rankIdx < file.length - 1) {
        return;
      }
      const cellOffsetY = fileOffsetY - m * c * 2 * rankIdx
      ctx.beginPath()
      ctx.moveTo(cellOffsetX - outlineWidth, cellOffsetY - m * c)
      ctx.lineTo(cellOffsetX + m * b - outlineWidth / 2, cellOffsetY + outlineWidth)
      ctx.lineTo(cellOffsetX + m * (b + a) + outlineWidth / 2, cellOffsetY + outlineWidth)
      ctx.lineTo(cellOffsetX + m * (b * 2 + a) + outlineWidth, cellOffsetY - m * c)
      ctx.lineTo(cellOffsetX + m * (b + a) + outlineWidth / 2, cellOffsetY - m * c * 2 - outlineWidth)
      ctx.lineTo(cellOffsetX + m * b - outlineWidth / 2, cellOffsetY - m * c * 2 - outlineWidth)
      ctx.fill()
    })
  })

  game.board.forEach((file, fileIdx) => {
    const cellOffsetX = offsetX + m * fileIdx * (a + b)
    const fileOffsetY = offsetY - m * c * Math.abs(5 - fileIdx)
    file.forEach((cell, rankIdx) => {
      const cellOffsetY = fileOffsetY - m * c * 2 * rankIdx
      ctx.beginPath()
      ctx.moveTo(cellOffsetX, cellOffsetY - m * c)
      ctx.lineTo(cellOffsetX + m * b, cellOffsetY)
      ctx.lineTo(cellOffsetX + m * (b + a), cellOffsetY)
      ctx.lineTo(cellOffsetX + m * (b * 2 + a), cellOffsetY - m * c)
      ctx.lineTo(cellOffsetX + m * (b + a), cellOffsetY - m * c * 2)
      ctx.lineTo(cellOffsetX + m * b, cellOffsetY - m * c * 2)
      ctx.lineTo(cellOffsetX, cellOffsetY - m * c)
      ctx.fillStyle = cellColorRGB(cell.color)
      ctx.fill()
      if (game.selected?.from?.file_idx == fileIdx && game.selected?.from?.rank_idx == rankIdx) {
        ctx.fillStyle = selectedCellOverlayRGBA
        ctx.fill()
      }
    })
  })

  game.board.forEach((file, fileIdx) => {
    const cellOffsetX = offsetX + m * fileIdx * (a + b)
    const fileOffsetY = offsetY - m * c * Math.abs(5 - fileIdx)
    file.forEach((cell, rankIdx) => {
      const cellOffsetY = fileOffsetY - m * c * 2 * rankIdx
      if (cell.piece) {
        ctx.translate(cellOffsetX + m * (b + a / 2), cellOffsetY - m * c)
        ctx.scale(shapeScale * m, -shapeScale * m)
        drawPiece(ctx, cell.piece.color, cell.piece.ptype)
        ctx.resetTransform()
      }
      if (game.selected?.to?.some(({file_idx, rank_idx}) => file_idx == fileIdx && rank_idx == rankIdx)) {
        ctx.beginPath()
        ctx.arc(cellOffsetX + m * (b + a / 2), cellOffsetY - m * c, 3 * m, 0, 2 * Math.PI, false)
        ctx.fillStyle = selectedCellOverlayRGBA
        ctx.fill()
      }
    })
  })
}

const drawPiece = (ctx: CanvasRenderingContext2D, color: PieceColor, piece: PieceType) => {
  const fillStyles = ['#000', '#fff']
  let fillStyleIdx = 0
  shapes[color][piece].forEach((path: Path2D) => {
      ctx.fillStyle = fillStyles[fillStyleIdx]
      ctx.fill(path)
      fillStyleIdx = (fillStyleIdx + 1) % fillStyles.length
  })
}

export const findCellByPixelCoords = (ctx: CanvasRenderingContext2D, x: number, y: number): Coords | undefined => {
  const heightBase = 11 * c * 2
  const widthBase = 11 * (a + b) + b
  const maxSize = ctx.canvas.clientHeight - padding * 2
  const m = maxSize / heightBase
  const offsetX = padding + Math.floor((maxSize - m * widthBase) / 2)
  const offsetY = padding + maxSize - Math.ceil((maxSize - m * heightBase) / 2)
  const approxFileIdx = (x - offsetX) / (m * (b + a))
  const flooredFileIdx = Math.floor(approxFileIdx) 
  const approxRankIdx = (offsetY - y - m * c * Math.abs(5 - flooredFileIdx)) / (m * 2 * c)
  const flooredRankIdx = Math.floor(approxRankIdx)
  let file_idx = flooredFileIdx
  let rank_idx = flooredRankIdx
  if (approxFileIdx - flooredFileIdx < b / (a + b)) {
    const dFile = (approxFileIdx - flooredFileIdx) * (a + b) / b
    const dRank = Math.abs(flooredRankIdx + 0.5 - approxRankIdx) * 2
    if (dFile < dRank) {
      file_idx--
      if (approxRankIdx - flooredRankIdx < 0.5) {
        if (file_idx < 5) {
          rank_idx--
        }
      } else {
        if (file_idx >= 5) {
          rank_idx++
        }
      }
    }
  }
  if (file_idx < 0 || file_idx > 10 || rank_idx < 0 || rank_idx > 10 - Math.abs(5 - file_idx)) {
    return;
  }
  return {
    file_idx,
    rank_idx,
  }
}

export const processCanvasClick = (ctx: CanvasRenderingContext2D, game: Game, x: number, y: number) => {
  const coords = findCellByPixelCoords(ctx, x, y)
  const selected = game.selected
  delete game.selected
  if (coords) {
    if (!selected || selected.from.file_idx !== coords.file_idx || selected.from.rank_idx !== coords.rank_idx) {
      const available_move = game.available_moves
        .find(({from}) => from.file_idx === coords.file_idx && from.rank_idx === coords.rank_idx)
      if (available_move) {
        game.selected = available_move
      }
    }
  }
  drawBoard(ctx, game)
}