export interface SearchQuery {
  q: string;
}

export interface Search {
  kind: "artist" | "album" | "track";
  id: number;
  artist: {
    id: number;
    name: string;
  };
  album?: {
    id: number;
    title: string;
  };
  track?: {
    id: number;
    title: string;
  };
}
