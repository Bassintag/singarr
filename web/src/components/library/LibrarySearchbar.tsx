import type { Search } from "@/domain/search";
import { searchQueryOptions } from "@/queries/search";
import { router } from "@/router";
import { useQuery } from "@tanstack/react-query";
import { DiscIcon, MusicIcon, UserIcon } from "lucide-react";
import { useMemo, useState } from "react";
import {
  Autocomplete,
  AutocompleteCollection,
  AutocompleteContent,
  AutocompleteEmpty,
  AutocompleteGroup,
  AutocompleteGroupLabel,
  AutocompleteInput,
  AutocompleteItem,
  AutocompleteList,
} from "../ui/Autocomplete";
import { AppImage } from "../layout/AppImage";

export function LibrarySearchbar() {
  const [q, setQ] = useState("");
  const { data, isEnabled } = useQuery({
    ...searchQueryOptions({ q }),
    enabled: q.length >= 1,
  });
  const groups = useMemo(() => {
    if (data == null) return [];
    const groups: Record<Search["kind"], Search[]> = {
      artist: [],
      album: [],
      track: [],
    };
    for (const item of data) {
      groups[item.kind].push(item);
    }
    return Object.entries(groups)
      .filter(([, value]) => value.length > 0)
      .map(([key, value]) => ({
        value: key,
        items: value,
      }));
  }, [data]);

  return (
    <Autocomplete value={q} onValueChange={setQ} filteredItems={groups}>
      <AutocompleteInput placeholder="Search" />
      {isEnabled && (
        <AutocompleteContent>
          <AutocompleteEmpty>No result</AutocompleteEmpty>
          <AutocompleteList>
            {(group: (typeof groups)[number]) => (
              <AutocompleteGroup key={group.value} items={group.items}>
                <AutocompleteGroupLabel>{group.value}s</AutocompleteGroupLabel>
                <AutocompleteCollection>
                  {(item: Search) => (
                    <LibrarySearchbarItem
                      key={`${item.kind}:${item.id}`}
                      search={item}
                    />
                  )}
                </AutocompleteCollection>
              </AutocompleteGroup>
            )}
          </AutocompleteList>
        </AutocompleteContent>
      )}
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
    let description: string | undefined = undefined;
    if (search.kind !== "artist") {
      description = search.artist.name;
      if (search.kind === "track") {
        description = `${search.album.title} - ${search.artist.name}`;
      } else {
        description = search.artist.name;
      }
    }
    return {
      title: search.track?.title ?? search.album?.title ?? search.artist.name,
      description,
      icon: icons[search.kind],
    };
  }, [search]);

  return (
    <AutocompleteItem
      nativeButton={false}
      className="flex flex-row gap-2 items-center"
      onClick={(e) => {
        e.stopPropagation();
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
      {search.imagePath ? (
        <AppImage
          className="shrink-0 flex items-center justify-center bg-gray-800 rounded-sm size-8 [&>svg]:size-3.5"
          src={search.imagePath}
        />
      ) : (
        <div className="shrink-0 flex items-center justify-center bg-gray-800 rounded-sm size-8 [&>svg]:size-3.5">
          {icon}
        </div>
      )}
      <div className="overflow-hidden">
        <div className="text-sm truncate">{title}</div>
        {description && (
          <div className="text-gray-400 text-xs truncate">{description}</div>
        )}
      </div>
    </AutocompleteItem>
  );
}
