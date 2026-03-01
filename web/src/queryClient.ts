import {
  keepPreviousData,
  QueryCache,
  QueryClient,
} from "@tanstack/react-query";
import { useTokenState } from "./hooks/token/useTokenState";
import { router } from "./router";
import { ApiAuthError, ApiError } from "./utils/api";

const queryCache = new QueryCache({
  onError: (e) => {
    console.error(e);
    if (e instanceof ApiError) {
      if (e.response.status === 401 || e.response.status === 403) {
        useTokenState.getState().set(null);
        router.navigate({ to: "/login" });
      }
    } else if (e instanceof ApiAuthError) {
      useTokenState.getState().set(null);
      router.navigate({ to: "/login" });
    }
  },
});

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 60_000,
      placeholderData: keepPreviousData,
      retry: (retryCount, e) => {
        if (retryCount > 3) return false;
        if (e instanceof ApiAuthError) return false;
        return (
          !(e instanceof ApiError) ||
          (e.response.status !== 401 && e.response.status !== 403)
        );
      },
    },
  },
  queryCache,
});
