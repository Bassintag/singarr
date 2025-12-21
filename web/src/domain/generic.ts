import z from "zod";

export interface Page<T> {
  total: number;
  items: T[];
}

export const pageableSchema = z.object({
  page: z.int().min(0).optional(),
  size: z.int().min(1).max(64).optional(),
});

export const idSchema = z.object({
  id: z.coerce.number().min(1),
});

export interface Pageable {
  page: number;
  size: number;
}

export interface TrackStats {
  tracksCount: number;
  withLyricsCount: number;
}
