import type { Status } from "@/domain/status";
import { fetchApi } from "@/utils/api";
import { queryOptions } from "@tanstack/react-query";

export function statusQueryOptions() {
  return queryOptions({
    queryKey: ["status"],
    queryFn: () => fetchApi<Status>("status"),
  });
}
