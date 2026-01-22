import { LibraryBanner } from "@/components/library/LibraryBanner";
import { TrackTable } from "@/components/track/TrackTable";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/(app)/tracks/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <>
      <LibraryBanner />
      <TrackTable />
    </>
  );
}
