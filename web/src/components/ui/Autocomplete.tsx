import { cn } from "@/utils/cn";
import { Autocomplete as Primitive } from "@base-ui/react";
import type { ComponentProps } from "react";
import { inputVariants } from "./Input";

export const Autocomplete = Primitive.Root;

export function AutocompleteContent({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Popup>) {
  return (
    <Primitive.Portal>
      <Primitive.Positioner sideOffset={8} className="z-50">
        <Primitive.Popup
          className={cn(
            "bg-gray-900 border border-gray-700 rounded py-1 w-(--anchor-width)",
            className
          )}
          {...rest}
        />
      </Primitive.Positioner>
    </Primitive.Portal>
  );
}

export function AutocompleteInput({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Input>) {
  return (
    <Primitive.Input className={cn(inputVariants(), className)} {...rest} />
  );
}

export function AutocompleteList({
  className,
  ...rest
}: ComponentProps<typeof Primitive.List>) {
  return (
    <Primitive.List
      className={cn("flex flex-col divide-gray-700", className)}
      {...rest}
    />
  );
}

export function AutocompleteItem({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Item>) {
  return (
    <Primitive.Item
      className={cn(
        "px-2 py-1 transition cursor-pointer hover:bg-gray-800",
        className
      )}
      {...rest}
    />
  );
}
