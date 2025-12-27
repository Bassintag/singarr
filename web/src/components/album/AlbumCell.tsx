import type { Album } from "@/domain/album";
import { Disc3Icon } from "lucide-react";
import { LinkCell, LinkCellImage, LinkCellPlaceholder } from "../ui/Table";

export function AlbumCell({ album }: { album: Album }) {
  return (
    <LinkCell
      to={`/artists/$id`}
      params={{ id: album.artist.id }}
      search={{ albumId: album.id }}
    >
      {album.coverPath ? (
        <LinkCellImage src={album.coverPath} />
      ) : (
        <LinkCellPlaceholder>
          <Disc3Icon />
        </LinkCellPlaceholder>
      )}

      {album.title}
    </LinkCell>
  );
}
