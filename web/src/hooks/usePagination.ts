import type { Table } from "@tanstack/react-table";
import { useMemo } from "react";

export type PageItem =
  | {
      type: "ellipsis";
    }
  | {
      type: "page";
      index: number;
    };

const pageItems = 7;

export function usePagination<T>(table: Table<T>) {
  const state = table.getState();
  const pageCount = table.getPageCount(); // 20
  const { pageIndex, pageSize } = state.pagination; // 10
  const items = useMemo(() => {
    const items: PageItem[] = [];
    const count = pageItems; // 5
    const halfItems = Math.floor(count / 2); // 3
    let start = Math.max(0, pageIndex - halfItems); // 7
    let end = Math.min(pageCount, start + count); // 13
    start = Math.max(0, end - count);
    if (start > 0) {
      start += 2;
      items.push({ type: "page", index: 0 });
      items.push({ type: "ellipsis" });
    }
    if (end < pageCount - 1) {
      end -= 2;
    }
    for (let i = start; i < end; i += 1) {
      items.push({ type: "page", index: i });
    }
    if (end < pageCount - 1) {
      items.push({ type: "ellipsis" });
      items.push({ type: "page", index: pageCount - 1 });
    }
    return items;
  }, [pageCount, pageIndex]);
  const count = table.getRowCount();
  const startIndex = pageIndex * pageSize;
  const endIndex = Math.min(count, startIndex + pageSize);
  return { items, pageIndex, count, startIndex, endIndex };
}
