import type { TokenPair, TokenPayload } from "@/domain/token";
import { useTokenState } from "@/hooks/token/useTokenState";
import * as qs from "qs";

export class ApiError extends Error {
  readonly response: ResponseInit;

  constructor(
    response: ResponseInit,
    message?: string,
    options?: ErrorOptions
  ) {
    super(message, options);
    this.response = response;
  }
}

export interface ApiRequestInit extends RequestInit {
  auth?: boolean;
  json?: unknown;
  query?: unknown;
}

let tokenPromise: Promise<string> | null = null;

function isExpired(payload: TokenPayload) {
  return (payload.exp - 5) * 1000 < Date.now();
}

async function getToken() {
  const tokens = useTokenState.getState().tokens;
  if (tokens == null) {
    return null;
  }
  if (!isExpired(tokens.accessPayload)) {
    return tokens.access;
  }
  if (tokenPromise == null) {
    if (isExpired(tokens.refreshPayload)) {
      return null;
    }
    tokenPromise = new Promise<string>((resolve, reject) => {
      fetchApi<TokenPair>("tokens", {
        auth: false,
        method: "POST",
        json: { refreshToken: tokens.refresh },
      })
        .then((tokens) => {
          useTokenState.getState().set(tokens);
          resolve(tokens.access);
        })
        .catch(reject)
        .finally(() => {
          tokenPromise = null;
        });
    });
  }
  return tokenPromise;
}

export function resolveApiUrl(path: string) {
  const basePath = import.meta.env.PUBLIC_API_PATH;
  if (basePath) path = `${basePath}${path}`;
  return new URL(path, import.meta.env.PUBLIC_API_URL ?? window.location.href);
}

export async function fetchApi<T>(
  path: string,
  { auth = true, json, query, ...init }: ApiRequestInit = {}
) {
  const url = resolveApiUrl(path);
  url.search = qs.stringify(query);
  init.headers = new Headers(init.headers);
  if (auth) {
    const token = await getToken();
    if (token) {
      init.headers.set("Authorization", `Bearer ${token}`);
    }
  }
  if (json) {
    init.headers.set("Content-Type", "application/json");
    init.body = JSON.stringify(json);
  }
  const response = await fetch(url, init);
  if (!response.ok) {
    throw new ApiError(response);
  }
  return (await response.json()) as T;
}
