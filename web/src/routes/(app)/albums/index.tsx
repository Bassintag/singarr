import { AlbumTable } from "@/components/album/AlbumTable";
import { SyncLibraryButton } from "@/components/library/SyncLibraryButton";
import { Banner, BannerActions } from "@/components/ui/Banner";
import { createFileRoute } from "@tanstack/react-router";
import { RefreshCwIcon } from "lucide-react";

export const Route = createFileRoute("/(app)/albums/")({
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
      <AlbumTable />
    </div>
  );
}
