import type { AlbumSearch, AlbumWithStats } from "@/domain/album";
import type { Page } from "@/domain/generic";
import { fetchApi } from "@/utils/api";
import { queryOptions } from "@tanstack/react-query";

export function albumsQueryOptions(query?: AlbumSearch) {
  return queryOptions({
    queryKey: ["albums", query],
    queryFn: () => {
      return fetchApi<Page<AlbumWithStats>>("albums", { query });
    },
  });
}

export function albumQueryOptions(id: number) {
  return queryOptions({
    queryKey: ["albums", id],
    queryFn: () => {
      return fetchApi<AlbumWithStats>(`albums/${id}`);
    },
  });
}
