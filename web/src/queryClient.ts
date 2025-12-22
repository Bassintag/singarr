import {
  keepPreviousData,
  QueryCache,
  QueryClient,
} from "@tanstack/react-query";
import { ApiError } from "./utils/api";
import { router } from "./router";
import { useTokenState } from "./hooks/token/useTokenState";

const queryCache = new QueryCache({
  onError: (e) => {
    if (e instanceof ApiError) {
      if (e.response.status === 401) {
        useTokenState.getState().set(undefined);
        router.navigate({ to: "/login" });
      }
    }
  },
});

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      placeholderData: keepPreviousData,
      retry: (retryCount, e) => {
        if (retryCount > 3) return false;
        return !(e instanceof ApiError) || e.response.status != 401;
      },
    },
  },
  queryCache,
});
