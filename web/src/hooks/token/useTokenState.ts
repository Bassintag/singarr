import type { TokenPair, TokenPayload } from "@/domain/token";
import { create } from "zustand";
import { persist } from "zustand/middleware";
import { jwtDecode } from "jwt-decode";

export interface TokenState {
  tokens:
    | (TokenPair & {
        accessPayload: TokenPayload;
        refreshPayload: TokenPayload;
      })
    | null;

  set(tokens: TokenPair | null): void;
}

export const useTokenState = create(
  persist<TokenState>(
    (set) => ({
      tokens: null,
      set: (tokens) => {
        if (tokens) {
          const accessPayload = jwtDecode<TokenPayload>(tokens.access);
          const refreshPayload = jwtDecode<TokenPayload>(tokens.refresh);
          set({ tokens: { ...tokens, accessPayload, refreshPayload } });
        } else {
          set({ tokens: null });
        }
      },
    }),
    { name: "singarr:token" }
  )
);
