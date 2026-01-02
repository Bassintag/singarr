import type { CountsStats } from "@/domain/stats";
import { fetchApi } from "@/utils/api";
import { queryOptions } from "@tanstack/react-query";

export function countsStatsQueryOptions() {
  return queryOptions({
    queryKey: ["stats", "counts"],
    queryFn: () => fetchApi<CountsStats>("stats/counts"),
  });
}
