import { cn } from "@/utils/cn";
import { Separator as Primitive } from "@base-ui/react/separator";
import type { ComponentProps } from "react";

export function Separator({
  className,
  ...rest
}: ComponentProps<typeof Primitive>) {
  return <Primitive className={cn("h-px bg-gray-700", className)} {...rest} />;
}
