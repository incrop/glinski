import { handleMessage } from "./client";
import { drawBoard } from "./draw";
import { Game } from "./models";

const canvas = document.getElementsByTagName('canvas')[0]
const ctx = canvas.getContext('2d')!!
ctx.translate(0.5, 0.5)

let game: Game | null = null

const onResize = () => {
  const width = Math.min(window.innerHeight, window.innerWidth)
  canvas.height = width
  canvas.width = width
  game && drawBoard(ctx, game.board)
}

window.addEventListener('resize', onResize)
onResize()

const socket = new WebSocket("ws://" + location.host + "/ws", "chess");
socket.onmessage = (event) => {
  game = handleMessage(game, JSON.parse(event.data))
  drawBoard(ctx, game.board)
}
