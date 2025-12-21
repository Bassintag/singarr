import type { Page } from "@/domain/generic";
import type { Track, TrackSearch } from "@/domain/track";
import { fetchApi } from "@/utils/api";
import { queryOptions } from "@tanstack/react-query";

export function tracksQueryOptions(query?: TrackSearch) {
  return queryOptions({
    queryKey: ["tracks", query],
    queryFn: () => {
      return fetchApi<Page<Track>>("tracks", { query });
    },
  });
}

export function trackQueryOptions(id: number) {
  return queryOptions({
    queryKey: ["tracks", id],
    queryFn: () => {
      return fetchApi<Track>(`tracks/${id}`);
    },
  });
}
