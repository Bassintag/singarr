import type { Search } from "@/domain/search";
import { searchQueryOptions } from "@/queries/search";
import { router } from "@/router";
import { useQuery } from "@tanstack/react-query";
import { DiscIcon, MusicIcon, UserIcon } from "lucide-react";
import { useMemo, useState } from "react";
import {
  Autocomplete,
  AutocompleteContent,
  AutocompleteInput,
  AutocompleteItem,
  AutocompleteList,
} from "../ui/Autocomplete";

export function LibrarySearchbar() {
  const [q, setQ] = useState("");
  const { data } = useQuery({
    ...searchQueryOptions({ q }),
    enabled: q.length >= 3,
  });

  return (
    <Autocomplete value={q} onValueChange={setQ} filteredItems={data}>
      <AutocompleteInput placeholder="Search" />
      <AutocompleteContent>
        <AutocompleteList>
          {(search: Search) => (
            <LibrarySearchbarItem key={search.id} search={search} />
          )}
        </AutocompleteList>
      </AutocompleteContent>
    </Autocomplete>
  );
}

const icons = {
  artist: <UserIcon />,
  album: <DiscIcon />,
  track: <MusicIcon />,
};

function LibrarySearchbarItem({ search }: { search: Search }) {
  const { title, description, icon } = useMemo(() => {
    return {
      title: search.track?.title ?? search.album?.title ?? search.artist.name,
      description: search.kind === "artist" ? undefined : search.artist.name,
      icon: icons[search.kind],
    };
  }, [search]);
  return (
    <AutocompleteItem
      nativeButton={false}
      className="flex flex-row gap-2 items-center"
      onClick={() => {
        router.navigate({
          to: "/artists/$id",
          params: { id: search.artist.id },
          search: {
            albumId: search.album?.id,
            trackId: search.track?.id,
          },
        });
      }}
    >
      <div className="shrink-0 flex items-center justify-center bg-gray-800 rounded-sm size-8 [&>svg]:size-3.5">
        {icon}
      </div>
      <div className="overflow-hidden">
        <div className="text-sm truncate">{title}</div>
        {description && (
          <div className="text-gray-400 text-xs">{description}</div>
        )}
      </div>
    </AutocompleteItem>
  );
}
