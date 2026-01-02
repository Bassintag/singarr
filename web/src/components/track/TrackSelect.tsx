import type { Album } from "@/domain/album";
import type { Track, TrackSearch } from "@/domain/track";
import { tracksQueryOptions } from "@/queries/track";
import { useQuery } from "@tanstack/react-query";
import { useMemo, type ComponentProps } from "react";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectGroupLabel,
  SelectItem,
  SelectItems,
  SelectTrigger,
} from "../ui/Select";
import { AppImage } from "../layout/AppImage";

export function TrackSelect<Multiple extends boolean | undefined = false>({
  query,
  ...rest
}: ComponentProps<typeof Select<number, Multiple>> & { query?: TrackSearch }) {
  const { data: page } = useQuery(tracksQueryOptions(query));

  const groups = useMemo(() => {
    if (page == null) return [];
    const groups = new Map<number, { album: Album; tracks: Track[] }>();
    for (const track of page.items) {
      const existing = groups.get(track.album.id);
      if (existing) {
        existing.tracks.push(track);
      } else {
        groups.set(track.album.id, { album: track.album, tracks: [track] });
      }
    }
    return Array.from(groups.values());
  }, [page]);

  return (
    <Select
      {...rest}
      itemToStringLabel={(id) => {
        return page?.items.find((i) => i.id === id)?.title ?? "Inconnu";
      }}
    >
      <SelectTrigger />
      <SelectContent>
        {groups.map((g) => (
          <SelectGroup key={g.album.id}>
            <SelectGroupLabel>
              {g.album.coverPath && (
                <AppImage
                  src={g.album.coverPath}
                  className="size-3.5 rounded-xs"
                />
              )}
              {g.album.title}
            </SelectGroupLabel>
            <SelectItems>
              {g.tracks.map((track) => (
                <SelectItem key={track.id} value={track.id}>
                  {track.title}
                </SelectItem>
              ))}
            </SelectItems>
          </SelectGroup>
        ))}
      </SelectContent>
    </Select>
  );
}
