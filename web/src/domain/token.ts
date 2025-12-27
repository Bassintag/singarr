import z from "zod";

export interface TokenPayload {
  iat: number;
  exp: number;
}

export interface TokenPair {
  access: string;
  refresh: string;
}

export interface TokenPayload {
  iat: number;
  exp: number;
}

export const createTokensSchema = z.object({
  username: z.string().min(1),
  password: z.string().min(1),
});

export type CreateTokens = z.infer<typeof createTokensSchema>;
