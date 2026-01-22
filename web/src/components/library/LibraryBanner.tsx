import { HardDriveIcon, RefreshCwIcon, SearchIcon } from "lucide-react";
import { Banner, BannerActions } from "../ui/Banner";
import { SyncLibraryButton } from "./SyncLibraryButton";
import { SearchLibraryButton } from "./SearchLibraryButton";
import { ScanLibraryButton } from "./ScanLibraryButton";

export function LibraryBanner() {
  return (
    <Banner>
      <BannerActions>
        <SyncLibraryButton variant="ghost" size="sm">
          <RefreshCwIcon />
          Sync library
        </SyncLibraryButton>
        <ScanLibraryButton variant="ghost" size="sm">
          <HardDriveIcon />
          Scan disk
        </ScanLibraryButton>
        <SearchLibraryButton variant="ghost" size="sm">
          <SearchIcon />
          Search missing
        </SearchLibraryButton>
      </BannerActions>
    </Banner>
  );
}
