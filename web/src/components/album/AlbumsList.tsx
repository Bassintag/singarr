import type { AlbumWithStats } from "@/domain/album";
import { albumsQueryOptions } from "@/queries/album";
import { Route } from "@/routes/artists/$id";
import { useQuery } from "@tanstack/react-query";
import { ChevronDownIcon } from "lucide-react";
import { useState, type ComponentProps } from "react";
import { TrackStatsProgress } from "../stats/TrackStatsProgress";
import { cn } from "@/utils/cn";
import { TrackList } from "../track/TrackList";

export function AlbumsList({ className, ...rest }: ComponentProps<"ol">) {
  const { id } = Route.useParams();
  const { data: page } = useQuery(albumsQueryOptions({ artistId: id }));

  return (
    <ol className={cn("m-4 flex flex-col gap-4", className)} {...rest}>
      {page?.items.map((album) => (
        <AlbumsListRow key={album.id} album={album} />
      ))}
    </ol>
  );
}

export function AlbumsListRow({ album }: { album: AlbumWithStats }) {
  const albumId = Route.useSearch({ select: (s) => s.albumId });
  const defaultOpen = albumId === album.id;
  const [open, setOpen] = useState(defaultOpen);

  return (
    <li
      data-state={open ? "open" : "close"}
      className="group border rounded overflow-hidden border-gray-700"
    >
      <div
        className={
          "bg-gray-900 flex flex-row items-center gap-4 h-12 px-4 transition-colors hover:bg-gray-800 active:bg-gray-950 group-data-[state=open]:border-b-0 group-data-[state=open]:rounded-b-none"
        }
        onClick={() => setOpen(!open)}
      >
        <ChevronDownIcon className="size-4.5 transition-transform group-data-[state=open]:rotate-180" />
        <div className="text-base">{album.title}</div>
        <TrackStatsProgress className="w-64 ml-auto" stats={album.stats} />
      </div>
      {open && <TrackList className="rounded-t-none" albumId={album.id} />}
    </li>
  );
}
