import type { TokenPair } from "@/domain/token";
import { create } from "zustand";

export interface TokenState {
  tokens?: TokenPair;

  set(tokens: TokenPair | undefined): void;
}

export const useTokenState = create<TokenState>((set) => ({
  tokens: undefined,
  set: (tokens) => set({ tokens }),
}));
