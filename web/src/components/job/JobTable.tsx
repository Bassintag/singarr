/* eslint-disable react-hooks/incompatible-library */
import type { Job } from "@/domain/job";
import { jobsQueryOptions } from "@/queries/job";
import { toPageable } from "@/utils/query";
import { useQuery } from "@tanstack/react-query";
import {
  getCoreRowModel,
  useReactTable,
  type ColumnDef,
  type PaginationState,
} from "@tanstack/react-table";
import { formatRelative } from "date-fns";
import { useState } from "react";
import {
  DataTable,
  DataTableContent,
  DataTablePagination,
  DataTablePlaceholder,
  TableContainer,
} from "../ui/Table";
import { sentenceCase } from "change-case";

const columns: ColumnDef<Job>[] = [
  {
    id: "id",
    header: "ID",
    accessorKey: "id",
    meta: {
      className: "w-20 text-center",
    },
  },
  {
    id: "createdAt",
    header: "Created at",
    meta: {
      className: "w-56",
    },
    accessorFn: (job) => formatRelative(job.createdAt, new Date()),
  },
  {
    id: "type",
    header: "Type",
    meta: {
      className: "max-md:w-48",
    },
    accessorFn: (job) => sentenceCase(job.payload.type),
  },
  {
    id: "status",
    header: "Status",
    meta: {
      className: "max-md:w-32",
    },
    accessorFn: (job) => sentenceCase(job.status),
  },
];

export function JobTable() {
  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 24,
  });
  const { data: page } = useQuery(jobsQueryOptions(toPageable(pagination)));
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
