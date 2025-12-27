import { pageableSchema, type TrackStats } from "./generic";

export interface Artist {
  id: number;
  name: string;
  description?: string;
  imagePath?: string;
  lidarrId: number;
  musicbrainzId: string;
}

export interface ArtistWithStats extends Artist {
  stats: TrackStats;
}

export const artistSearchSchema = pageableSchema;
