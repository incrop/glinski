import { drawField } from "./draw";
import { generateFiled } from "./temp";

const canvas = document.getElementsByTagName('canvas')[0]
const ctx = canvas.getContext('2d')!!
ctx.translate(0.5, 0.5)

const field = generateFiled()

const onResize = () => {
  const width = Math.min(window.innerHeight, window.innerWidth)
  canvas.height = width
  canvas.width = width
  drawField(ctx, field)
}

window.addEventListener('resize', onResize)
onResize()
