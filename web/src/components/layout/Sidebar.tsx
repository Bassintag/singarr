import { cn } from "@/utils/cn";
import { Link } from "@tanstack/react-router";
import type { ComponentProps } from "react";

export function Sidebar({ className, ...rest }: ComponentProps<"aside">) {
  return (
    <aside
      className={cn(
        "fixed z-10 left-0 top-16 bottom-0 w-50 flex flex-col p-2 bg-gray-900 border-r border-gray-700",
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
