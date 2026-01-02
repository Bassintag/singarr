import { artistQueryOptions } from "@/queries/artist";
import { Route } from "@/routes/(app)/artists/$id";
import { resolveImageUrl } from "@/utils/image";
import { useQuery } from "@tanstack/react-query";
import {
  CloudUploadIcon,
  HardDriveIcon,
  Mic2Icon,
  MusicIcon,
  RadarIcon,
  RefreshCwIcon,
  SearchIcon,
} from "lucide-react";
import { AppImage } from "../layout/AppImage";
import {
  Banner,
  BannerAction,
  BannerActions,
  BannerBackground,
  BannerContent,
  BannerDescription,
  BannerHeader,
  BannerTitle,
} from "../ui/Banner";
import { Tag } from "../ui/Tag";
import { AutomaticSearchArtistButton } from "./AutomaticSearchArtistButton";
import { ScanArtistButton } from "./ScanArtistButton";
import { SyncArtistButton } from "./SyncArtistButton";
import { ImportLyricsDialog } from "../lyrics/ImportLyricsDialog";

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
        <ImportLyricsDialog>
          <BannerAction className="md:ml-auto">
            <CloudUploadIcon />
            Upload
          </BannerAction>
        </ImportLyricsDialog>
      </BannerActions>
      <BannerBackground src={resolveImageUrl(artist?.imagePath)}>
        <BannerContent>
          {artist?.imagePath && (
            <AppImage
              className="size-64 rounded bg-cover border border-gray-700 max-md:hidden"
              src={artist.imagePath}
            />
          )}
          <BannerHeader>
            <BannerTitle>{artist?.name}</BannerTitle>
            <div className="flex flex-row flex-wrap gap-2">
              <Tag variant="secondary">
                <MusicIcon />
                {artist?.stats.tracksCount} tracks
              </Tag>
              <Tag variant="secondary">
                <Mic2Icon />
                {artist?.stats.withLyricsCount} lyrics
              </Tag>
              <Tag variant="secondary">
                <RadarIcon />
                Lidarr ID: {artist?.lidarrId}
              </Tag>
            </div>
            {artist?.description && (
              <BannerDescription>{artist?.description}</BannerDescription>
            )}
          </BannerHeader>
        </BannerContent>
      </BannerBackground>
    </Banner>
  );
}
