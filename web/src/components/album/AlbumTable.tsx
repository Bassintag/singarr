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
import { ArtistCell } from "../artist/ArtistCell";
import { TrackStatsProgress } from "../stats/TrackStatsProgress";
import {
  DataTable,
  DataTableContent,
  DataTablePagination,
  DataTablePlaceholder,
  TableContainer,
} from "../ui/Table";
import { AlbumCell } from "./AlbumCell";

const columns: ColumnDef<AlbumWithStats>[] = [
  {
    id: "title",
    header: "Title",
    meta: { className: "w-1/2 max-md:w-64" },
    cell: (data) => <AlbumCell album={data.row.original} />,
  },
  {
    id: "artist",
    header: "Artist",
    meta: { className: "w-1/2 max-md:w-48" },
    cell: (data) => <ArtistCell artist={data.row.original.artist} />,
  },
  {
    id: "progress",
    header: "Lyrics",
    meta: { className: "w-64" },
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
    <DataTable table={table}>
      <TableContainer>
        <DataTableContent className="table-fixed" />
      </TableContainer>
      <DataTablePlaceholder />
      <DataTablePagination />
    </DataTable>
  );
}
