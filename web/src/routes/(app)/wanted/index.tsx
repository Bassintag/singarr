import { TrackTable } from "@/components/track/TrackTable";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/(app)/wanted/")({
  component: RouteComponent,
});

function RouteComponent() {
  return <TrackTable search={{ hasLyrics: false }} />;
}
