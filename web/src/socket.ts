import type { AppEvent } from "./domain/event";
import { resolveApiUrl } from "./utils/api";

const socketUrl = resolveApiUrl("socket");
socketUrl.protocol = "ws";
export const socket = new WebSocket(socketUrl.href);

export function listenToEvents(callback: (event: AppEvent) => void) {
  const listener = (message: MessageEvent<string>) => {
    callback(JSON.parse(message.data));
  };
  socket.addEventListener("message", listener);
  return () => {
    socket.removeEventListener("message", listener);
  };
}
