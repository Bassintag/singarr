import { cn } from "@/utils/cn";
import { cva, type VariantProps } from "class-variance-authority";
import type { ComponentProps } from "react";

// eslint-disable-next-line react-refresh/only-export-components
export const buttonVariants = cva(
  "shrink-0 flex flex-row justify-center items-center gap-2 font-medium transition-colors rounded disabled:opacity-50",
  {
    variants: {
      variant: {
        solid:
          "text-gray-200 bg-primary-500 hover:bg-primary-400 active:bg-primary-600",
        ghost: "text-gray-300 hover:bg-white/10 active:bg-white/5",
        outline:
          "border border-gray-700 bg-gray-900 hover:bg-gray-800 active:bg-gray-950",
        danger: "bg-failure text-black",
      },
      size: {
        md: "h-8 px-4 [&_svg]:size-4.5 text-sm",
        sm: "h-6 px-2 [&_svg]:size-3.5 text-xs",
        icon: "size-8 [&_svg]:size-4.5 text-sm",
        "icon-sm": "size-6 [&_svg]:size-3.5 text-xs",
      },
    },
    defaultVariants: {
      variant: "solid",
      size: "md",
    },
  }
);

export function Button({
  variant,
  size,
  className,
  ...rest
}: ComponentProps<"button"> & VariantProps<typeof buttonVariants>) {
  return (
    <button
      type="button"
      className={cn(buttonVariants({ variant, size }), className)}
      {...rest}
    />
  );
}
