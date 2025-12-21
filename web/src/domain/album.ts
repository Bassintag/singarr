import z from "zod";
import { pageableSchema, type TrackStats } from "./generic";
import type { Artist } from "./artist";

export interface Album {
  id: number;
  title: string;
  coverPath?: string;
  lidarrId: number;
  musicbrainzId: string;
  artist: Artist;
}

export interface AlbumWithStats extends Album {
  stats: TrackStats;
}

export const albumSearchSchema = pageableSchema.extend({
  artistId: z.int().min(1).optional(),
});

export type AlbumSearch = z.infer<typeof albumSearchSchema>;
