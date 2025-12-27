import { useSidebarState } from "@/hooks/layout/useSidebarState";
import { cn } from "@/utils/cn";
import { Link } from "@tanstack/react-router";
import type { ComponentProps } from "react";

export function Sidebar({ className, ...rest }: ComponentProps<"aside">) {
  const open = useSidebarState((s) => s.open);

  return (
    <aside
      data-close={!open || undefined}
      data-open={open || undefined}
      className={cn(
        "fixed z-10 left-0 top-16 bottom-0 flex flex-col px-3 py-4 bg-gray-900 border-r border-gray-700 transition-transform max-md: right-0 max-md:-translate-x-full max-md:data-open:translate-x-0 md:w-50",
        className
      )}
      {...rest}
    />
  );
}

export function SidebarLink({
  className,
  ...rest
}: ComponentProps<typeof Link>) {
  return (
    <Link
      className={cn(
        "flex flex-row items-center gap-4 h-10 px-3 text-sm border-l-2 border-transparent rounded transition-colors hover:bg-gray-800 [&_svg]:size-4.5 data-[status=active]:border-primary-500 data-[status=active]:rounded-l-none",
        className
      )}
      {...rest}
    />
  );
}
