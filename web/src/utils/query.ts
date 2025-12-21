import type { Pageable } from "@/domain/generic";
import type { QueryClient } from "@tanstack/react-query";
import type { PaginationState } from "@tanstack/react-table";

export function toPageable(pagination: PaginationState): Pageable {
  return {
    page: pagination.pageIndex,
    size: pagination.pageSize,
  };
}

export async function invalidateAll(client: QueryClient) {
  await Promise.all([
    client.invalidateQueries({
      queryKey: ["artists"],
    }),
    client.invalidateQueries({
      queryKey: ["albums"],
    }),
    client.invalidateQueries({
      queryKey: ["tracks"],
    }),
    client.invalidateQueries({
      queryKey: ["lyrics"],
    }),
  ]);
}
