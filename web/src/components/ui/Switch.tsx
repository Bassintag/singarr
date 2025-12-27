import { cn } from "@/utils/cn";
import { Switch as Primitive } from "@base-ui/react/switch";
import type { ComponentProps } from "react";

export function Switch({
  value,
  onChange,
  className,
  ...rest
}: Omit<ComponentProps<typeof Primitive.Root>, "value" | "onChange"> & {
  value?: ComponentProps<typeof Primitive.Root>["checked"];
  onChange?: ComponentProps<typeof Primitive.Root>["onCheckedChange"];
}) {
  return (
    <Primitive.Root
      checked={value}
      onCheckedChange={onChange}
      className={cn(
        "relative inline-flex flex-row items-center w-12 h-6.5 bg-gray-800 border border-gray-700 rounded-full data-checked:bg-primary-500 data-checked:border-primary-700 transition-colors",
        className
      )}
      {...rest}
    >
      <Primitive.Thumb className="shrink-0 absolute size-5 bg-gray-600 rounded-full top-0.5 left-0.5 data-checked:left-6 data-checked:bg-gray-300 transition-[left]" />
    </Primitive.Root>
  );
}
