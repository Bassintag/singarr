import { cn } from "@/utils/cn";
import type { ComponentProps } from "react";

export function Progress({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn("relative h-5 bg-gray-800 rounded-sm", className)}
      {...rest}
    />
  );
}

export function ProgressBar({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "absolute top-0 left-0 bottom-0 bg-primary-800 rounded-sm",
        className
      )}
      {...rest}
    />
  );
}

export function ProgressLabel({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "absolute left-1/2 top-1/2 -translate-1/2 text-xs font-medium text-gray-300",
        className
      )}
      {...rest}
    />
  );
}
