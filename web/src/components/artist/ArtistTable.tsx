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
import { Link } from "../ui/Link";
import { DataTable, DataTablePagination } from "../ui/Table";

const columns: ColumnDef<ArtistWithStats>[] = [
  {
    id: "title",
    header: "Title",
    cell: (data) => (
      <Link to={`/artists/$id`} params={{ id: data.row.original.id }}>
        {data.row.original.name}
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
    <>
      <DataTable className="table-fixed" table={table} />
      <DataTablePagination table={table} />
    </>
  );
}
