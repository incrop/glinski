import { CellColor, Field } from "./models";

const padding = 10
const outlineWidth = 4
const outlineColorRGB = '#000000'

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

export const drawField = (ctx: CanvasRenderingContext2D, field: Field) => {
  const heightBase = 11 * c * 2
  const widthBase = 11 * (a + b) + b
  const maxSize = ctx.canvas.clientHeight - padding * 2
  const m = Math.floor(maxSize / heightBase) || (maxSize / heightBase)
  const offsetX = padding + Math.floor((maxSize - m * widthBase) / 2)
  const offsetY = padding + maxSize - Math.ceil((maxSize - m * heightBase) / 2);
  const renderRuns = [true, false]


  ctx.fillStyle = outlineColorRGB
  field.forEach((file, fileIdx) => {
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

  field.forEach((file, fileIdx) => {
    const cellOffsetX = offsetX + m * fileIdx * (a + b)
    const fileOffsetY = offsetY - m * c * Math.abs(5 - fileIdx)
    file.forEach((cell, rankIdx) => {
      const cellOffsetY = fileOffsetY - m * c * 2 * rankIdx
      ctx.fillStyle = cellColorRGB(cell.color)
      ctx.beginPath()
      ctx.moveTo(cellOffsetX, cellOffsetY - m * c)
      ctx.lineTo(cellOffsetX + m * b, cellOffsetY)
      ctx.lineTo(cellOffsetX + m * (b + a), cellOffsetY)
      ctx.lineTo(cellOffsetX + m * (b * 2 + a), cellOffsetY - m * c)
      ctx.lineTo(cellOffsetX + m * (b + a), cellOffsetY - m * c * 2)
      ctx.lineTo(cellOffsetX + m * b, cellOffsetY - m * c * 2)
      ctx.lineTo(cellOffsetX, cellOffsetY - m * c)
      ctx.fill()
    })
  })
}