import type { Track } from "@/domain/track";
import { Link } from "../ui/Link";

export function TrackCell({ track }: { track: Track }) {
  return (
    <Link
      to={`/artists/$id`}
      params={{ id: track.artist.id }}
      search={{ albumId: track.album.id, trackId: track.id }}
      className="block truncate"
    >
      {track.title}
    </Link>
  );
}
