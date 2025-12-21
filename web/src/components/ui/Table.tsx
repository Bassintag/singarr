import { cn } from "@/utils/cn";
import { flexRender, type RowData, type Table } from "@tanstack/react-table";
import type { ComponentProps } from "react";
import { Button } from "./Button";
import { ChevronLeftIcon, ChevronRightIcon, EllipsisIcon } from "lucide-react";
import { usePagination } from "@/hooks/usePagination";

export function Table({ className, ...rest }: ComponentProps<"table">) {
  return <table className={cn("w-full", className)} {...rest} />;
}

export function THead({ className, ...rest }: ComponentProps<"thead">) {
  return (
    <thead
      className={cn("bg-gray-800 border-b border-gray-700", className)}
      {...rest}
    />
  );
}

export function TBody({ className, ...rest }: ComponentProps<"tbody">) {
  return (
    <tbody
      className={cn("*:even:bg-gray-900 *:odd:bg-gray-950", className)}
      {...rest}
    />
  );
}

export function TFoot({ className, ...rest }: ComponentProps<"tfoot">) {
  return (
    <tfoot
      className={cn(
        "py-2 px-4 bg-gray-950 border-t border-gray-700",
        className
      )}
      {...rest}
    />
  );
}

export function Tr({ className, ...rest }: ComponentProps<"tr">) {
  return <tr className={cn("", className)} {...rest} />;
}

export function Th({ className, ...rest }: ComponentProps<"th">) {
  return (
    <th
      className={cn("py-2 px-4 text-start font-normal", className)}
      {...rest}
    />
  );
}

export function Td({ className, ...rest }: ComponentProps<"td">) {
  return <td className={cn("py-2 px-4 text-sm", className)} {...rest} />;
}

declare module "@tanstack/react-table" {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  interface ColumnMeta<TData extends RowData, TValue> {
    className?: string;
    cellClassName?: string;
    headerClassName?: string;
  }
}

export function DataTable<T>({
  table,
  ...rest
}: ComponentProps<"table"> & { table: Table<T> }) {
  return (
    <Table {...rest}>
      <THead>
        {table.getHeaderGroups().map((headerGroup) => (
          <Tr key={headerGroup.id}>
            {headerGroup.headers.map((header) => {
              const meta = header.column.columnDef.meta;
              return (
                <Th
                  key={header.id}
                  className={cn(meta?.className, meta?.headerClassName)}
                  colSpan={header.colSpan}
                >
                  {flexRender(
                    header.column.columnDef.header,
                    header.getContext()
                  )}
                </Th>
              );
            })}
          </Tr>
        ))}
      </THead>
      <TBody>
        {table.getRowModel().rows.map((row) => (
          <Tr key={row.id}>
            {row.getVisibleCells().map((cell) => {
              const meta = cell.column.columnDef.meta;
              return (
                <Td
                  key={cell.id}
                  className={cn(meta?.className, meta?.cellClassName)}
                >
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </Td>
              );
            })}
          </Tr>
        ))}
      </TBody>
    </Table>
  );
}

export function DataTablePagination<T>({
  table,
  className,
  ...rest
}: ComponentProps<"div"> & { table: Table<T> }) {
  const pagination = usePagination(table);
  return (
    <div
      className={cn(
        "px-4 py-2 bg-gray-950 border-t border-gray-700 flex flex-row justify-between items-center gap-4",
        className
      )}
      {...rest}
    >
      <div className="text-xs">
        Show {pagination.startIndex + 1} to {pagination.endIndex} of{" "}
        {pagination.count} entries
      </div>
      <div className="flex flex-row items-center gap-2">
        <Button
          type="button"
          variant="outline"
          size="icon-sm"
          disabled={!table.getCanPreviousPage()}
          onClick={() => table.setPageIndex(pagination.pageIndex - 1)}
        >
          <ChevronLeftIcon />
        </Button>
        {pagination.items.map((item, i) =>
          item.type === "ellipsis" ? (
            <div
              key={`${item.type}-${i}`}
              className="size-6 flex items-center justify-center"
            >
              <EllipsisIcon className="size-3.5" />
            </div>
          ) : (
            <Button
              key={`${item.type}_${item.index}-${i}`}
              type="button"
              variant="outline"
              size="icon-sm"
              data-active={item.index === pagination.pageIndex || undefined}
              className="data-active:border-primary-700 data-active:bg-primary-900"
              onClick={() => table.setPageIndex(item.index)}
            >
              {item.index + 1}
            </Button>
          )
        )}
        <Button
          type="button"
          variant="outline"
          size="icon-sm"
          disabled={!table.getCanNextPage()}
          onClick={() => table.setPageIndex(pagination.pageIndex + 1)}
        >
          <ChevronRightIcon />
        </Button>
      </div>
    </div>
  );
}
