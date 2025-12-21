import { cn } from "@/utils/cn";
import type { ComponentProps } from "react";

export function Navbar({ className, ...rest }: ComponentProps<"header">) {
  return (
    <header
      className={cn(
        "fixed z-10 left-0 right-0 top-0 h-16 px-4 flex flex-row items-center gap-4 bg-gray-900 border-b border-gray-700",
        className
      )}
      {...rest}
    />
  );
}

export function NavbarApp({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn("flex flex-row items-center gap-4", className)}
      {...rest}
    />
  );
}

export function NavbarTitle({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "bg-primary-900 text-white px-3 py-1 rounded text-sm uppercase",
        className
      )}
      {...rest}
    />
  );
}

export function NavbarActions({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn("ml-auto flex flex-row items-center gap-4", className)}
      {...rest}
    />
  );
}
