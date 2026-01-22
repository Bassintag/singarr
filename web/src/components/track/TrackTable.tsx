/* eslint-disable react-hooks/incompatible-library */
import type { Track, TrackSearch } from "@/domain/track";
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
import { AlbumCell } from "../album/AlbumCell";
import { ArtistCell } from "../artist/ArtistCell";
import {
  DataTable,
  DataTableContent,
  DataTablePagination,
  DataTablePlaceholder,
  TableContainer,
} from "../ui/Table";
import { TrackCell } from "./TrackCell";

const columns: ColumnDef<Track>[] = [
  {
    id: "trackNumber",
    accessorKey: "trackNumber",
    header: "#",
    meta: { className: "w-12 text-center" },
  },
  {
    id: "title",
    header: "Title",
    meta: { className: "overflow-hidden max-md:w-64" },
    cell: (data) => <TrackCell track={data.row.original} />,
  },
  {
    id: "album",
    header: "Album",
    enableResizing: false,
    meta: { className: "w-1/4 overflow-hidden max-md:w-64" },
    cell: (data) => <AlbumCell album={data.row.original.album} />,
  },
  {
    id: "artist",
    header: "Artist",
    meta: { className: "w-1/4 overflow-hidden max-md:w-48" },
    cell: (data) => <ArtistCell artist={data.row.original.artist} />,
  },
];

export function TrackTable({ search }: { search?: TrackSearch }) {
  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 24,
  });
  const { data: page } = useQuery(
    tracksQueryOptions({
      ...search,
      ...toPageable(pagination),
    })
  );
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
