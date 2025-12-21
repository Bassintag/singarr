import { cn } from "@/utils/cn";
import { cva, type VariantProps } from "class-variance-authority";
import { Input as Primitive } from "@base-ui/react/input";
import type { ComponentProps } from "react";

// eslint-disable-next-line react-refresh/only-export-components
export const inputVariants = cva(
  "w-full rounded bg-gray-800 border border-gray-700 text-sm text-regular text-gray-300 transition-colors placeholder:text-gray-600 focus-visible:outline-none focus-visible:border-primary-600 group-[&]/input:border-0 group-[&]/input:bg-transparent",
  {
    variants: {
      size: {
        md: "h-8 px-2",
      },
    },
    defaultVariants: {
      size: "md",
    },
  }
);

export function Input({
  size,
  className,
  ...rest
}: ComponentProps<typeof Primitive> & VariantProps<typeof inputVariants>) {
  return (
    <Primitive className={cn(inputVariants({ size }), className)} {...rest} />
  );
}
