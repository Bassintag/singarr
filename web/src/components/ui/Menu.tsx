import { cn } from "@/utils/cn";
import { Menu as Primitive } from "@base-ui/react/menu";
import type { ComponentProps } from "react";

export const Menu = Primitive.Root;

export const MenuTrigger = Primitive.Trigger;

export function MenuContent({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Popup>) {
  return (
    <Primitive.Portal>
      <Primitive.Positioner sideOffset={8} className="z-50">
        <Primitive.Popup
          className={cn(
            "bg-gray-800 rounded border border-gray-700 overflow-auto min-w-(--anchor-width) max-w-(--available-width) max-h-(--available-height) py-1",
            className
          )}
          {...rest}
        />
      </Primitive.Positioner>
    </Primitive.Portal>
  );
}

export function MenuItem({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Item>) {
  return (
    <Primitive.Item
      className={cn(
        "flex flex-row items-center gap-2 px-4 h-8 text-sm text-gray-300 transition-colors [&>svg]:size-4.5 hover:bg-gray-700 active:bg-gray-900",
        className
      )}
      {...rest}
    />
  );
}
