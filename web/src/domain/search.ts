export interface SearchQuery {
  q: string;
}

export interface ArtistSearch {
  kind: "artist";
  id: number;
  imagePath: string | null;
  artist: {
    id: number;
    name: string;
  };
  album: null;
  track: null;
}

export interface AlbumSearch {
  kind: "album";
  id: number;
  imagePath: string | null;
  artist: {
    id: number;
    name: string;
  };
  album: {
    id: number;
    title: string;
  };
  track: null;
}

export interface TrackSearch {
  kind: "track";
  id: number;
  imagePath: string | null;
  artist: {
    id: number;
    name: string;
  };
  album: {
    id: number;
    title: string;
  };
  track: {
    id: number;
    title: string;
  };
}

export type Search = ArtistSearch | AlbumSearch | TrackSearch;
