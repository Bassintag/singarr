import type { AppEvent } from "@/domain/event";
import { getToken, resolveApiUrl } from "@/utils/api";
import { create } from "zustand";
import { useTokenState } from "../token/useTokenState";

export interface SocketState {
  listen: (listener: SocketListener) => () => void;
}

export type SocketListener = (event: AppEvent) => void;

export const useSocketState = create<SocketState>(() => {
  const listeners: SocketListener[] = [];
  let socket: WebSocket | null = null;

  async function createSocket() {
    const token = await getToken();
    const socketUrl = resolveApiUrl("socket");
    socketUrl.protocol = socketUrl.protocol.startsWith("https") ? "wss" : "ws";
    if (token) {
      socketUrl.searchParams.set("accessToken", token);
    }
    socket = new WebSocket(socketUrl.href);
    socket.addEventListener("message", (e) => {
      const event = JSON.parse(e.data) as AppEvent;
      for (const listener of listeners) {
        listener(event);
      }
    });
    socket.addEventListener("close", () => {
      socket = null;
    });
  }

  useTokenState.subscribe(() => {
    if (socket == null) {
      void createSocket();
    }
  });

  void createSocket();

  return {
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
