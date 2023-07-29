import { Game, Message } from "./models";

export const handleMessage = (game: Game | null, message: Message): Game => {
  switch (message.type) {
    case "GameState": return message.data;
  }
}