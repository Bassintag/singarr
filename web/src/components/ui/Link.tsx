import { cn } from "@/utils/cn";
import { Link as RouterLink, type LinkProps } from "@tanstack/react-router";
import type { ComponentProps } from "react";

export function Link({ className, ...props }: LinkProps & ComponentProps<"a">) {
  return (
    <RouterLink
      className={cn(
        "text-primary-600 font-medium hover:text-primary-500 active:text-primary-700",
        className
      )}
      {...props}
    />
  );
}
