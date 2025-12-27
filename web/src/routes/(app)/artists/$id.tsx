import { AlbumsList } from "@/components/album/AlbumsList";
import { ArtistBanner } from "@/components/artist/ArtistBanner";
import { LyricsTable } from "@/components/lyrics/LyricsTable";
import { idSchema } from "@/domain/generic";
import { createFileRoute } from "@tanstack/react-router";
import z from "zod";

export const Route = createFileRoute("/(app)/artists/$id")({
  component: RouteComponent,
  params: idSchema,
  validateSearch: z.object({
    albumId: z.int().min(1).optional().catch(undefined),
    trackId: z.int().min(1).optional().catch(undefined),
  }),
});

function RouteComponent() {
  return (
    <>
      <ArtistBanner />
      <div className="flex flex-col gap-8 p-4 md:px-6 md:py-8">
        <AlbumsList />
        <div className="flex flex-col gap-4">
          <div className="text-xl">Lyrics files</div>
          <div className="border border-gray-700 rounded">
            <LyricsTable />
          </div>
        </div>
      </div>
    </>
  );
}
