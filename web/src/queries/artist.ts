import type { ArtistWithStats } from "@/domain/artist";
import type { Page, Pageable } from "@/domain/generic";
import { fetchApi } from "@/utils/api";
import { queryOptions } from "@tanstack/react-query";

export function artistsQueryOptions(query?: Pageable) {
  return queryOptions({
    queryKey: ["artists", query],
    queryFn: () => {
      return fetchApi<Page<ArtistWithStats>>("artists", { query });
    },
  });
}

export function artistQueryOptions(id: number) {
  return queryOptions({
    queryKey: ["artists", id],
    queryFn: () => {
      return fetchApi<ArtistWithStats>(`artists/${id}`);
    },
  });
}
