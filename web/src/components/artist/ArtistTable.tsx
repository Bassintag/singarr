/* eslint-disable react-hooks/incompatible-library */
import type { ArtistWithStats } from "@/domain/artist";
import { artistsQueryOptions } from "@/queries/artist";
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
import {
  DataTable,
  DataTableContent,
  DataTablePagination,
  DataTablePlaceholder,
  TableContainer,
} from "../ui/Table";
import { ArtistCell } from "./ArtistCell";

const columns: ColumnDef<ArtistWithStats>[] = [
  {
    id: "title",
    header: "Title",
    meta: {
      className: "max-md:w-58",
    },
    cell: (data) => <ArtistCell artist={data.row.original} />,
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

export function ArtistTable() {
  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 24,
  });
  const { data: page } = useQuery(artistsQueryOptions(toPageable(pagination)));
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
