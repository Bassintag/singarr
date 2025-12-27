import { usePagination } from "@/hooks/usePagination";
import { cn } from "@/utils/cn";
import { flexRender, type RowData, type Table } from "@tanstack/react-table";
import { ChevronLeftIcon, ChevronRightIcon, EllipsisIcon } from "lucide-react";
import { createContext, use, type ComponentProps } from "react";
import { Button } from "./Button";
import { Link } from "./Link";
import { AppImage } from "../layout/AppImage";

export function TableContainer({ className, ...rest }: ComponentProps<"div">) {
  return <div className={cn("overflow-x-auto", className)} {...rest} />;
}

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
      className={cn(
        "h-10 px-4 text-start text-sm font-normal first:pl-6 last:pr-6",
        className
      )}
      {...rest}
    />
  );
}

export function Td({ className, ...rest }: ComponentProps<"td">) {
  return (
    <td
      className={cn("py-2 px-4 text-sm first:pl-6 last:pr-6", className)}
      {...rest}
    />
  );
}

declare module "@tanstack/react-table" {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  interface ColumnMeta<TData extends RowData, TValue> {
    className?: string;
    cellClassName?: string;
    headerClassName?: string;
  }
}

const DataTableContext = createContext<Table<unknown>>(null as never);

export function DataTable<T>({
  table,
  className,
  ...rest
}: ComponentProps<"div"> & { table: Table<T> }) {
  return (
    <DataTableContext value={table as Table<unknown>}>
      <div className={cn("flex flex-col", className)} {...rest} />
    </DataTableContext>
  );
}

export function DataTableContent(props: ComponentProps<typeof Table>) {
  const table = use(DataTableContext);

  return (
    <Table {...props}>
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

export function DataTablePlaceholder({
  className,
  children = "No items found",
  ...rest
}: ComponentProps<"div">) {
  const table = use(DataTableContext);

  return (
    table.getRowCount() === 0 && (
      <div
        className={cn(
          "py-4 text-center text-sm italic text-gray-400",
          className
        )}
        {...rest}
      >
        {children}
      </div>
    )
  );
}

export function DataTablePagination({
  className,
  ...rest
}: ComponentProps<"div">) {
  const table = use(DataTableContext);
  const pagination = usePagination(table);

  return (
    <div
      className={cn(
        "px-6 py-4 bg-gray-950 border-t border-gray-700 flex flex-col justify-between items-center gap-4 md:flex-row",
        className
      )}
      {...rest}
    >
      <div className="text-xs">
        Show {Math.min(pagination.endIndex, pagination.startIndex + 1)} to{" "}
        {pagination.endIndex} of {pagination.count} entries
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

export function LinkCell({ className, ...rest }: ComponentProps<typeof Link>) {
  return (
    <Link
      className={cn("flex flex-row gap-2 items-center truncate", className)}
      {...rest}
    />
  );
}

export function LinkCellImage({
  className,
  ...rest
}: ComponentProps<typeof AppImage>) {
  return (
    <AppImage
      className={cn(
        "size-6 border border-gray-700 rounded shrink-0",
        className
      )}
      {...rest}
    />
  );
}

export function LinkCellPlaceholder({
  className,
  ...rest
}: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "size-6 flex items-center justify-center *:size-4 shrink-0 text-gray-400 bg-gray-950 border border-gray-700 rounded",
        className
      )}
      {...rest}
    />
  );
}
