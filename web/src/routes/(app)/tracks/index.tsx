import { SyncLibraryButton } from "@/components/library/SyncLibraryButton";
import { TrackTable } from "@/components/track/TrackTable";
import { Banner, BannerActions } from "@/components/ui/Banner";
import { createFileRoute } from "@tanstack/react-router";
import { RefreshCwIcon } from "lucide-react";

export const Route = createFileRoute("/(app)/tracks/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div>
      <Banner>
        <BannerActions>
          <SyncLibraryButton variant="ghost" size="sm">
            <RefreshCwIcon />
            Sync
          </SyncLibraryButton>
        </BannerActions>
      </Banner>
      <TrackTable />
    </div>
  );
}
