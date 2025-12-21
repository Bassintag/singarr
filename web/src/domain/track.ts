import z from "zod";
import type { Album } from "./album";
import type { Artist } from "./artist";
import { pageableSchema } from "./generic";

export interface Track {
  id: number;
  trackNumber: number;
  title: string;
  hasLyrics: boolean;
  artist: Artist;
  album: Album;
}

export const trackSearchSchema = pageableSchema.extend({
  artistId: z.int().min(1).optional(),
  albumId: z.int().min(1).optional(),
});

export type TrackSearch = z.infer<typeof trackSearchSchema>;
