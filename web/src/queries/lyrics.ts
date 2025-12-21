import type { Page } from "@/domain/generic";
import type { Lyrics, LyricsContent, LyricsSearch } from "@/domain/lyrics";
import { fetchApi } from "@/utils/api";
import { invalidateAll } from "@/utils/query";
import { mutationOptions, queryOptions } from "@tanstack/react-query";

export function lyricsQueryOptions(query?: LyricsSearch) {
  return queryOptions({
    queryKey: ["lyrics", "list", query],
    queryFn: () => {
      return fetchApi<Page<Lyrics>>("lyrics", { query });
    },
  });
}

export function lyricQueryOptions(id: number) {
  return queryOptions({
    queryKey: ["lyrics", "details", id],
    queryFn: () => {
      return fetchApi<Lyrics>(`lyrics/${id}`);
    },
  });
}

export function deleteLyricMutationOptions() {
  return mutationOptions({
    mutationFn: (id: number) => {
      return fetchApi<Lyrics>(`lyrics/${id}`, { method: "DELETE" });
    },
    onSettled: async (_data, _error, id, _onResult, { client }) => {
      client.removeQueries({
        queryKey: ["lyrics", "details", id],
      });
      client.removeQueries({
        queryKey: ["lyrics", "content", id],
      });
      await invalidateAll(client);
    },
  });
}

export function lyricContentQueryOptions(id: number) {
  return queryOptions({
    queryKey: ["lyrics", "content", id],
    queryFn: () => {
      return fetchApi<LyricsContent>(`lyrics/${id}/content`);
    },
  });
}
