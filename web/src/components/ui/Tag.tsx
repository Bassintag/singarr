import { cn } from "@/utils/cn";
import { cva, type VariantProps } from "class-variance-authority";
import type { ComponentProps } from "react";

const tagVariants = cva(
  "inline-flex flex-row items-center gap-1 px-1.5 py-0.5 rounded-sm text-xs [&>svg]:size-3.5",
  {
    variants: {
      variant: {
        primary: "bg-primary-900 text-white",
        secondary: "bg-gray-800 text-gray-300",
      },
    },
    defaultVariants: {
      variant: "primary",
    },
  }
);

export function Tag({
  variant,
  className,
  ...rest
}: ComponentProps<"div"> & VariantProps<typeof tagVariants>) {
  return <div className={cn(tagVariants({ variant }), className)} {...rest} />;
}
