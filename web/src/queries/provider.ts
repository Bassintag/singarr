import type { ProviderResult, ProviderResultSearch } from "@/domain/provider";
import { fetchApi } from "@/utils/api";
import { queryOptions } from "@tanstack/react-query";

export function getProvidersResultsQueryOption(query: ProviderResultSearch) {
  return queryOptions({
    queryKey: ["providers", "results", query],
    queryFn: () => {
      return fetchApi<ProviderResult[]>("providers/results", { query });
    },
    refetchOnMount: false,
    refetchOnReconnect: false,
    refetchOnWindowFocus: false,
  });
}
