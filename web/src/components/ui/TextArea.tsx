import { cn } from "@/utils/cn";
import type { ComponentProps } from "react";

export function TextArea({ className, ...rest }: ComponentProps<"textarea">) {
  return (
    <textarea
      className={cn(
        className,
        "w-full rounded bg-gray-800 border border-gray-700 text-sm text-regular text-gray-300 transition-colors placeholder:text-gray-600 focus-visible:outline-none focus-visible:border-primary-600"
      )}
      {...rest}
    />
  );
}
