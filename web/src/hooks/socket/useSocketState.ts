import { create } from "zustand";
import { useTokenState } from "../token/useTokenState";
import { resolveApiUrl } from "@/utils/api";
import type { AppEvent } from "@/domain/event";

export interface SocketState {
  socket: WebSocket | null;
  listen: (listener: SocketListener) => () => void;
}

export type SocketListener = (event: AppEvent) => void;

export const useSocketState = create<SocketState>((set, get) => {
  const listeners: SocketListener[] = [];

  function createSocket() {
    const tokens = useTokenState.getState().tokens;
    if (tokens == null) return null;
    const socketUrl = resolveApiUrl("socket");
    socketUrl.protocol = socketUrl.protocol === "https" ? "wss" : "ws";
    if (tokens) {
      socketUrl.searchParams.set("accessToken", tokens.access);
    }
    const socket = new WebSocket(socketUrl.href);
    socket.addEventListener("message", (e) => {
      const event = JSON.parse(e.data) as AppEvent;
      for (const listener of listeners) {
        listener(event);
      }
    });
    return socket;
  }

  useTokenState.subscribe(() => {
    const { socket } = get();
    if (socket) {
      socket.close();
    }
    set({ socket: createSocket() });
  });

  return {
    socket: createSocket(),
    listen(listener) {
      listeners.push(listener);
      return () => {
        const index = listeners.indexOf(listener);
        if (index >= 0) {
          listeners.splice(index, 1);
        }
      };
    },
  };
});

export const useSocket = () => {
  return useSocketState((s) => s.socket);
};
