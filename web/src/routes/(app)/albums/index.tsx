import { AlbumTable } from "@/components/album/AlbumTable";
import { LibraryBanner } from "@/components/library/LibraryBanner";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/(app)/albums/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <>
      <LibraryBanner />
      <AlbumTable />
    </>
  );
}
