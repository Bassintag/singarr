import { AlbumsList } from "@/components/album/AlbumsList";
import { ArtistBanner } from "@/components/artist/ArtistBanner";
import { LyricsTable } from "@/components/lyrics/LyricsTable";
import { idSchema } from "@/domain/generic";
import { createFileRoute } from "@tanstack/react-router";
import z from "zod";

export const Route = createFileRoute("/artists/$id")({
  component: RouteComponent,
  params: idSchema,
  validateSearch: z.object({
    albumId: z.int().min(1).optional().catch(undefined),
    trackId: z.int().min(1).optional().catch(undefined),
  }),
});

function RouteComponent() {
  return (
    <div>
      <ArtistBanner />
      <AlbumsList />
      <div className="m-4 border border-gray-700 rounded overflow-hidden">
        <LyricsTable />
      </div>
    </div>
  );
}
