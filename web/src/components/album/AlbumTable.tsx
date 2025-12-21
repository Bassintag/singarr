/* eslint-disable react-hooks/incompatible-library */
import type { AlbumWithStats } from "@/domain/album";
import { albumsQueryOptions } from "@/queries/album";
import { toPageable } from "@/utils/query";
import { useQuery } from "@tanstack/react-query";
import {
  getCoreRowModel,
  useReactTable,
  type ColumnDef,
  type PaginationState,
} from "@tanstack/react-table";
import { useState } from "react";
import { TrackStatsProgress } from "../stats/TrackStatsProgress";
import { Link } from "../ui/Link";
import { DataTable, DataTablePagination } from "../ui/Table";

const columns: ColumnDef<AlbumWithStats>[] = [
  {
    id: "title",
    header: "Title",
    meta: {
      className: "w-1/2",
    },
    cell: (data) => (
      <Link
        to={`/artists/$id`}
        params={{ id: data.row.original.artist.id }}
        search={{ albumId: data.row.original.id }}
        className="block truncate"
      >
        {data.row.original.title}
      </Link>
    ),
  },
  {
    id: "artist",
    header: "Artist",
    meta: {
      className: "w-1/2",
    },
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
    id: "progress",
    header: "Lyrics",
    meta: {
      className: "w-64",
    },
    cell: (data) => <TrackStatsProgress stats={data.row.original.stats} />,
  },
];

export function AlbumTable() {
  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 24,
  });
  const { data: page } = useQuery(albumsQueryOptions(toPageable(pagination)));
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
