/* eslint-disable react-hooks/incompatible-library */
import type { Lyrics } from "@/domain/lyrics";
import { lyricsQueryOptions } from "@/queries/lyrics";
import { toPageable } from "@/utils/query";
import { useQuery } from "@tanstack/react-query";
import {
  getCoreRowModel,
  useReactTable,
  type ColumnDef,
  type PaginationState,
} from "@tanstack/react-table";
import { useState } from "react";
import { DataTable, DataTablePagination } from "../ui/Table";
import { CheckIcon, EyeIcon, XIcon } from "lucide-react";
import { Button } from "../ui/Button";
import { LyricsDialog } from "./LyricsDialog";
import { Route } from "@/routes/artists/$id";

const columns: ColumnDef<Lyrics>[] = [
  {
    accessorKey: "filePath",
    header: "File Path",
    meta: {
      className: "w-full",
    },
    cell: (data) => (
      <div className="font-mono">{data.row.original.filePath}</div>
    ),
  },
  {
    accessorKey: "synced",
    header: "Synced",
    meta: {
      className: "w-0 text-center",
    },
    cell: (data) => (
      <div className="inline-block mx-auto">
        {data.getValue() ? (
          <CheckIcon className="size-3.5 text-success" />
        ) : (
          <XIcon className="size-3.5 text-failure" />
        )}
      </div>
    ),
  },
  {
    header: "Actions",
    meta: {
      className: "w-0 text-center",
    },
    cell: (data) => (
      <div className="flex flex-row justify-center gap-2 items-center">
        <LyricsDialog key={data.row.original.id} lyricId={data.row.original.id}>
          <Button variant="ghost" size="icon-sm">
            <EyeIcon />
          </Button>
        </LyricsDialog>
      </div>
    ),
  },
];

export function LyricsTable() {
  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 24,
  });
  const { data: page } = useQuery(
    lyricsQueryOptions({
      ...toPageable(pagination),
      artistId: Route.useParams().id,
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
    <>
      <DataTable table={table} />
      <DataTablePagination table={table} />
    </>
  );
}
