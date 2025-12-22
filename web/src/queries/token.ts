import { type TokenPair, type CreateTokens } from "@/domain/token";
import { fetchApi } from "@/utils/api";
import { mutationOptions } from "@tanstack/react-query";

export function createTokensMutationOptions() {
  return mutationOptions({
    mutationFn: (body: CreateTokens) => {
      return fetchApi<TokenPair>("tokens", { method: "POST", json: body });
    },
  });
}
