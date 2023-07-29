import { handleMessage } from "./client";
import { drawBoard, processCanvasClick } from "./draw";
import { Game } from "./models";

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

const cookies = Object.fromEntries(document.cookie.split(";").map((pair) => pair.trim().split("=")))

const socket = new WebSocket("ws://" + location.host + "/ws", "chess");
socket.onopen = () => {
  socket.send(cookies.SID)
}
socket.onmessage = (event) => {
  game = handleMessage(game, JSON.parse(event.data))
  drawBoard(ctx, game)
}
