import { cn } from "@/utils/cn";
import type { ComponentProps } from "react";

export function InputGroup({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "group/input h-8 flex flex-row rounded bg-gray-800 border border-gray-700 has-focus-visible:border-primary-600",
        className
      )}
      {...rest}
    />
  );
}
