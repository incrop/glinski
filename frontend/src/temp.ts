import { CellColor, Field } from "./models";

export const generateFiled = (): Field =>
  Array.from(
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
