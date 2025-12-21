// Import
export interface ImportLyricsJob {
  type: "importLyrics";
  trackId: number;
  provider?: string;
}

// Search
export interface ScanArtistJob {
  type: "scanArtist";
  artistId: number;
}

export interface ScanAlbumJob {
  type: "scanAlbum";
  albumId: number;
}

export interface ScanTrackJob {
  type: "scanTrack";
  trackId: number;
}

// Search
export interface SearchArtistJob {
  type: "searchArtist";
  artistId: number;
}

export interface SearchAlbumJob {
  type: "searchAlbum";
  albumId: number;
}

export interface SearchTrackJob {
  type: "searchTrack";
  trackId: number;
}

// Sync
export interface SyncLibraryJob {
  type: "syncLibrary";
}

export interface SyncArtistJob {
  type: "syncArtist";
  artistId: number;
}

export type JobPayload =
  | ImportLyricsJob
  | ScanArtistJob
  | ScanAlbumJob
  | ScanTrackJob
  | SearchArtistJob
  | SearchAlbumJob
  | SearchTrackJob
  | SyncLibraryJob
  | SyncArtistJob;

export type JobStatus = "pending" | "running" | "done" | "failed";

export interface Job {
  id: number;
  payload: JobPayload;
  status: JobStatus;
  error?: string;
}
