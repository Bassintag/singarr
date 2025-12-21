import type { Track } from "@/domain/track";
import { tracksQueryOptions } from "@/queries/track";
import { Route } from "@/routes/artists/$id";
import { cn } from "@/utils/cn";
import { useQuery } from "@tanstack/react-query";
import { CheckIcon, SearchIcon, UserIcon, XIcon } from "lucide-react";
import type { ComponentProps } from "react";
import { Tag } from "../ui/Tag";
import { Button } from "../ui/Button";
import { AutomaticSearchTrackButton } from "./AutomaticSearchTrackButton";

export function TrackList({
  albumId,
  className,
  ...rest
}: ComponentProps<"ol"> & { albumId: number }) {
  const { data: tracks } = useQuery(tracksQueryOptions({ albumId }));

  return (
    <ol className={cn("", className)} {...rest}>
      {tracks?.items.map((track) => (
        <TrackListItem key={track.id} track={track} />
      ))}
    </ol>
  );
}

export function TrackListItem({ track }: { track: Track }) {
  const trackId = Route.useSearch({ select: (s) => s.trackId });

  return (
    <li
      data-highlight={trackId === track.id || undefined}
      className="flex flex-row items-center gap-4 px-4 py-2 text-sm transition-colors bg-gray-900 odd:bg-gray-950 hover:bg-gray-800 data-highlight:bg-primary-950"
    >
      <div className="w-4.5 text-center">{track.trackNumber}</div>
      <div>{track.title}</div>
      <div className="ml-auto [&_svg]:size-4.5">
        {track.hasLyrics ? (
          <Tag>
            <CheckIcon />
            Available
          </Tag>
        ) : (
          <Tag variant="secondary">
            <XIcon />
            Missing
          </Tag>
        )}
      </div>
      <div className="flex flex-row">
        <AutomaticSearchTrackButton
          size="icon-sm"
          variant="ghost"
          trackId={track.id}
        >
          <SearchIcon />
        </AutomaticSearchTrackButton>
        <Button
          size="icon-sm"
          variant="ghost"
          type="button"
          title="Interactive Search"
        >
          <UserIcon />
        </Button>
      </div>
    </li>
  );
}
