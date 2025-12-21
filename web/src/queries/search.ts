import type { Search, SearchQuery } from "@/domain/search";
import { fetchApi } from "@/utils/api";
import { queryOptions } from "@tanstack/react-query";

export function searchQueryOptions(query: SearchQuery) {
  return queryOptions({
    queryKey: ["search", query],
    queryFn: () => fetchApi<Search[]>(`search`, { query }),
  });
}
