import type { Task } from "@/domain/tasks";
import { fetchApi } from "@/utils/api";
import { queryOptions } from "@tanstack/react-query";

export function tasksQueryOptions() {
  return queryOptions({
    queryKey: ["tasks"],
    queryFn: () => {
      return fetchApi<Task[]>("tasks");
    },
  });
}
