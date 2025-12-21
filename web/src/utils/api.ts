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
  json?: unknown;
  query?: unknown;
}

export function resolveApiUrl(path: string) {
  const basePath = import.meta.env.PUBLIC_API_PATH;
  if (basePath) {
    path = `${basePath}${path}`;
  }
  return new URL(path, import.meta.env.PUBLIC_API_URL ?? window.location.href);
}

export async function fetchApi<T>(
  path: string,
  { json, query, ...init }: ApiRequestInit = {}
) {
  const url = resolveApiUrl(path);
  url.search = qs.stringify(query);
  if (json) {
    init.headers = new Headers(init.headers);
    init.headers.set("Content-Type", "application/json");
    init.body = JSON.stringify(json);
  }
  const response = await fetch(url, init);
  if (!response.ok) {
    throw new ApiError(response);
  }
  return (await response.json()) as T;
}
