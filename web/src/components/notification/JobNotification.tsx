import type { Job, JobPayload } from "@/domain/job";
import { albumQueryOptions } from "@/queries/album";
import { artistQueryOptions } from "@/queries/artist";
import { trackQueryOptions } from "@/queries/track";
import { useQuery } from "@tanstack/react-query";
import { createContext, use } from "react";

type JobContextValue = Job;

const JobContext = createContext<JobContextValue>(null as never);

function useJob<T extends JobPayload["type"]>() {
  return use(JobContext) as Job & { payload: { type: T } };
}

function ImportLyricsNotification() {
  const job = useJob<"importLyrics">();
  const { data: track } = useQuery(trackQueryOptions(job.payload.trackId));
  return (
    <>
      Importing {track?.title}
      {job.payload.provider && <> from {job.payload.provider}</>}
    </>
  );
}

function ArtistNotification({ prefix }: { prefix: string }) {
  const { data: artist } = useQuery(
    artistQueryOptions(
      useJob<"scanArtist" | "searchArtist" | "syncArtist">().payload.artistId
    )
  );
  return `${prefix} ${artist?.name}`;
}

function AlbumNotification({ prefix }: { prefix: string }) {
  const { data: album } = useQuery(
    albumQueryOptions(useJob<"scanAlbum" | "searchAlbum">().payload.albumId)
  );
  return `${prefix} ${album?.title}`;
}

function TrackNotification({ prefix }: { prefix: string }) {
  const { data: track } = useQuery(
    trackQueryOptions(useJob<"scanTrack" | "searchTrack">().payload.trackId)
  );
  return `${prefix} ${track?.title}`;
}

const elements: Record<JobPayload["type"], React.ReactNode> = {
  importLyrics: <ImportLyricsNotification />,
  scanLibrary: "Scanning library",
  scanArtist: <ArtistNotification prefix="Scanning" />,
  scanAlbum: <AlbumNotification prefix="Scanning" />,
  scanTrack: <TrackNotification prefix="Scanning" />,
  searchLibrary: "Searching library",
  searchArtist: <ArtistNotification prefix="Searching" />,
  searchAlbum: <AlbumNotification prefix="Searching" />,
  searchTrack: <TrackNotification prefix="Searching" />,
  syncLibrary: "Syncing library",
  syncArtist: <ArtistNotification prefix="Syncing" />,
};

export function JobNotificationTitle({ job }: { job: Job }) {
  const element = elements[job.payload.type];
  return element && <JobContext value={job}>{element}</JobContext>;
}
