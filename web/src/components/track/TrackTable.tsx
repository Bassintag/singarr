/* eslint-disable react-hooks/incompatible-library */
import type { Track } from "@/domain/track";
import { tracksQueryOptions } from "@/queries/track";
import { toPageable } from "@/utils/query";
import { useQuery } from "@tanstack/react-query";
import {
  getCoreRowModel,
  useReactTable,
  type ColumnDef,
  type PaginationState,
} from "@tanstack/react-table";
import { useState } from "react";
import { Link } from "../ui/Link";
import { DataTable, DataTablePagination } from "../ui/Table";

const columns: ColumnDef<Track>[] = [
  {
    id: "trackNumber",
    accessorKey: "trackNumber",
    header: "#",
    meta: {
      className: "w-12 text-center",
    },
  },
  {
    id: "title",
    header: "Title",
    meta: {
      className: "overflow-hidden",
    },
    cell: (data) => (
      <Link
        to={`/artists/$id`}
        params={{ id: data.row.original.artist.id }}
        search={{
          albumId: data.row.original.album.id,
          trackId: data.row.original.id,
        }}
        className="block truncate"
      >
        {data.row.original.title}
      </Link>
    ),
  },
  {
    id: "artist",
    header: "Artist",
    meta: { className: "w-1/4 overflow-hidden" },
    cell: (data) => (
      <Link
        to={`/artists/$id`}
        params={{ id: data.row.original.artist.id }}
        className="block truncate"
      >
        {data.row.original.artist.name}
      </Link>
    ),
  },
  {
    id: "album",
    header: "Album",
    enableResizing: false,
    meta: {
      className: "w-1/4 overflow-hidden",
    },
    cell: (data) => (
      <Link
        to={`/artists/$id`}
        params={{ id: data.row.original.artist.id }}
        search={{ albumId: data.row.original.album.id }}
        className="block truncate"
      >
        {data.row.original.album.title}
      </Link>
    ),
  },
];

export function TrackTable() {
  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 24,
  });
  const { data: page } = useQuery(tracksQueryOptions(toPageable(pagination)));
  const table = useReactTable({
    columns,
    data: page?.items ?? [],
    rowCount: page?.total ?? 0,
    state: { pagination },
    onPaginationChange: setPagination,
    getCoreRowModel: getCoreRowModel(),
    autoResetPageIndex: false,
  });

  return (
    <>
      <DataTable className="table-fixed" table={table} />
      <DataTablePagination table={table} />
    </>
  );
}
