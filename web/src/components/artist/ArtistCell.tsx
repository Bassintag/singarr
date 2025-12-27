import type { Artist } from "@/domain/artist";
import { UserIcon } from "lucide-react";
import { LinkCell, LinkCellImage, LinkCellPlaceholder } from "../ui/Table";

export function ArtistCell({ artist }: { artist: Artist }) {
  return (
    <LinkCell to={`/artists/$id`} params={{ id: artist.id }}>
      {artist.imagePath ? (
        <LinkCellImage src={artist.imagePath} />
      ) : (
        <LinkCellPlaceholder>
          <UserIcon />
        </LinkCellPlaceholder>
      )}
      {artist.name}
    </LinkCell>
  );
}
