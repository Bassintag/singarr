import { useQuery } from "@tanstack/react-query";
import {
  CloudUploadIcon,
  HardDriveIcon,
  RefreshCwIcon,
  SearchIcon,
} from "lucide-react";
import {
  Banner,
  BannerAction,
  BannerActions,
  BannerContent,
  BannerHeader,
  BannerTitle,
} from "../ui/Banner";
import { AutomaticSearchArtistButton } from "./AutomaticSearchArtistButton";
import { artistQueryOptions } from "@/queries/artist";
import { Route } from "@/routes/artists/$id";
import { SyncArtistButton } from "./SyncArtistButton";
import { ScanArtistButton } from "./ScanArtistButton";

export function ArtistBanner() {
  const { id } = Route.useParams();
  const { data: artist } = useQuery(artistQueryOptions(id));

  return (
    <Banner>
      <BannerActions>
        <SyncArtistButton size="sm" variant="ghost" artistId={id}>
          <RefreshCwIcon />
          Sync
        </SyncArtistButton>
        <ScanArtistButton size="sm" variant="ghost" artistId={id}>
          <HardDriveIcon />
          Scan disk
        </ScanArtistButton>
        <AutomaticSearchArtistButton size="sm" variant="ghost" artistId={id}>
          <SearchIcon />
          Search
        </AutomaticSearchArtistButton>
        <BannerAction className="ml-auto">
          <CloudUploadIcon />
          Upload
        </BannerAction>
      </BannerActions>
      <BannerContent>
        <BannerHeader>
          <BannerTitle>{artist?.name}</BannerTitle>
        </BannerHeader>
      </BannerContent>
    </Banner>
  );
}
