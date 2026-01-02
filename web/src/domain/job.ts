// Import
export interface ImportLyricsJob {
  type: "importLyrics";
  trackId: number;
  provider?: string;
  synced: boolean;
  content: string;
}

// Scan
export interface ScanLibraryJob {
  type: "scanLibrary";
}

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
export interface SearchLibraryJob {
  type: "searchLibrary";
}

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
  | ScanLibraryJob
  | ScanArtistJob
  | ScanAlbumJob
  | ScanTrackJob
  | SearchLibraryJob
  | SearchArtistJob
  | SearchAlbumJob
  | SearchTrackJob
  | SyncLibraryJob
  | SyncArtistJob;

export type JobStatus = "pending" | "running" | "done" | "failed";

export interface Job {
  id: number;
  createdAt: string;
  payload: JobPayload;
  status: JobStatus;
  error?: string;
}
