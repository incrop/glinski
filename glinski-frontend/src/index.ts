import { drawBoard, findCellByPixelCoords } from "./draw";
import { Coords, Game } from "./models";

const canvas = document.getElementsByTagName('canvas')[0]
const ctx = canvas.getContext('2d')!!

canvas.addEventListener('click', (event) => {
  if (game) {
    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;
    processCanvasClick(ctx, game, x, y);
  }
}, false);

let game: Game | null = null

const onResize = () => {
  const width = Math.min(window.innerHeight, window.innerWidth)
  canvas.height = width
  canvas.width = width
  game && drawBoard(ctx, game)
}

window.addEventListener('resize', onResize)
onResize()

const UID = localStorage.getItem("UID") || (() => {
  const S4 = () => (((1+Math.random())*0x10000)|0).toString(16).substring(1)
  const res = `${S4()}${S4()}-${S4()}-${S4()}-${S4()}-${S4()}${S4()}${S4()}`
  localStorage.setItem("UID", res)
  return res;
})()

const socket = new WebSocket("ws://" + location.host + "/ws", "chess");
socket.onopen = () => {
  socket.send(UID)
}
socket.onmessage = (event) => {
  game = JSON.parse(event.data)
  game && drawBoard(ctx, game)
}

const processCanvasClick = (ctx: CanvasRenderingContext2D, game: Game, x: number, y: number) => {
  const coords = findCellByPixelCoords(ctx, x, y)
  makeChanges(game, coords)
  drawBoard(ctx, game)
}

const makeChanges = (game: Game, coords: Coords | undefined) => {
  if (!coords) {
    delete game.selected
    return
  }
  if (game.selected) {
    const {from, to} = game.selected
    delete game.selected
    if (from.file_idx === coords.file_idx && from.rank_idx === coords.rank_idx) {
      return
    }
    const move_coords = to.find(({file_idx, rank_idx}) => file_idx === coords.file_idx && rank_idx === coords.rank_idx)
    if (move_coords) {
      const from_cell = game.board[from.file_idx][from.rank_idx]
      const to_cell = game.board[move_coords.file_idx][move_coords.rank_idx]
      to_cell.piece = from_cell.piece
      from_cell.piece = null
      game.available_moves = []
      game.last_move = {
        from: from,
        to: move_coords,
      }
      socket.send(JSON.stringify(game.last_move))
      return
    }
  }
  const available_move = game.available_moves
    .find(({from}) => from.file_idx === coords.file_idx && from.rank_idx === coords.rank_idx)
  if (available_move) {
    game.selected = available_move
  }
}
