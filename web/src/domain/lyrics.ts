import z from "zod";
import { pageableSchema } from "./generic";

export interface Lyrics {
  id: number;
  synced: boolean;
  filePath: string;
  checksum: string;
}

export interface LyricsContent {
  text: string;
}

export const lyricsSearchSchema = pageableSchema.extend({
  artistId: z.int().min(1).optional().catch(undefined),
  albumId: z.int().min(1).optional().catch(undefined),
  trackId: z.int().min(1).optional().catch(undefined),
});

export type LyricsSearch = z.infer<typeof lyricsSearchSchema>;
