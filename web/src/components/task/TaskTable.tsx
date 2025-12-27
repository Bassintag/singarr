/* eslint-disable react-hooks/incompatible-library */
import type { Task } from "@/domain/tasks";
import { createJobMutationOptions } from "@/queries/job";
import { tasksQueryOptions } from "@/queries/task";
import { useMutation, useQuery } from "@tanstack/react-query";
import {
  getCoreRowModel,
  useReactTable,
  type ColumnDef,
} from "@tanstack/react-table";
import { sentenceCase } from "change-case";
import { CronExpressionParser } from "cron-parser";
import * as cronstrue from "cronstrue";
import { formatDistanceToNow } from "date-fns";
import { PlayIcon } from "lucide-react";
import { Button } from "../ui/Button";
import {
  DataTable,
  DataTableContent,
  DataTablePlaceholder,
  TableContainer,
} from "../ui/Table";

const columns: ColumnDef<Task>[] = [
  {
    id: "name",
    header: "Name",
    accessorFn: (task) => sentenceCase(task.payload.type),
  },
  {
    id: "cron",
    header: "Interval",
    accessorFn: (task) => cronstrue.toString(task.cron),
  },
  {
    id: "next",
    header: "Next execution",
    accessorFn: (task) => {
      return formatDistanceToNow(
        CronExpressionParser.parse(task.cron).next().toDate(),
        { addSuffix: true, includeSeconds: false }
      );
    },
  },
  {
    id: "run",
    header: "Run",
    cell: ({ row }) => {
      // eslint-disable-next-line react-hooks/rules-of-hooks
      const createJob = useMutation(createJobMutationOptions());

      return (
        <Button
          variant="ghost"
          size="icon-sm"
          disabled={createJob.isPending}
          onClick={() => {
            createJob.mutate(row.original.payload);
          }}
        >
          <PlayIcon className="fill-current" />
        </Button>
      );
    },
  },
];

export function TaskTable() {
  const { data: tasks } = useQuery(tasksQueryOptions());
  const table = useReactTable({
    columns,
    data: tasks ?? [],
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <DataTable table={table}>
      <TableContainer>
        <DataTableContent />
      </TableContainer>
      <DataTablePlaceholder />
    </DataTable>
  );
}
