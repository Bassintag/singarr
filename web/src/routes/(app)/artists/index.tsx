import { ArtistTable } from "@/components/artist/ArtistTable";
import { LibraryBanner } from "@/components/library/LibraryBanner";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/(app)/artists/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <>
      <LibraryBanner />
      <ArtistTable />
    </>
  );
}
