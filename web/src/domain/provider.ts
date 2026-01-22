export interface ProviderFile {
  identifier: string;
  name: string;
  trackName: string;
  artistName: string;
  albumTitle: string;
  synced: boolean;
  durationMs: number;
  content: string;
}

export interface ProviderMetadata {
  name: string;
}

export interface ProviderResult {
  provider: ProviderMetadata;
  file: ProviderFile;
  score: number;
}

export interface ProviderResultSearch {
  trackId: number;
}
