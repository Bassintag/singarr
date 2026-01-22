import { cn } from "@/utils/cn";
import { Select as Primitive } from "@base-ui/react/select";
import { CheckIcon, ChevronDownIcon } from "lucide-react";
import type { ComponentProps } from "react";

export function Select<Value, Multiple extends boolean | undefined = false>({
  onChange,
  ...rest
}: Omit<ComponentProps<typeof Primitive.Root<Value, Multiple>>, "onChange"> & {
  onChange?: ComponentProps<
    typeof Primitive.Root<Value, Multiple>
  >["onValueChange"];
}) {
  return <Primitive.Root onValueChange={onChange} {...rest} />;
}

export function SelectTrigger({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Trigger>) {
  return (
    <Primitive.Trigger
      className={cn(
        "h-8 text-sm flex flex-row items-center rounded bg-gray-800 border border-gray-700 transition-colors hover:bg-gray-700 active:bg-gray-900 data-popup-open:border-primary-500",
        className
      )}
      {...rest}
    >
      <Primitive.Value className="grow pl-2 text-start" />
      <Primitive.Icon className="shrink-0 mr-2 transition-transform data-popup-open:rotate-180">
        <ChevronDownIcon className="size-4.5" />
      </Primitive.Icon>
    </Primitive.Trigger>
  );
}

export function SelectContent({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Popup>) {
  return (
    <Primitive.Portal>
      <Primitive.Positioner
        side="bottom"
        alignItemWithTrigger={false}
        sideOffset={8}
        className="z-50"
      >
        <Primitive.Popup
          className={cn(
            "bg-gray-800 rounded border border-gray-700 overflow-auto min-w-(--anchor-width) max-w-(--available-width) max-h-(--available-height) *:first:pt-1 *:last:pb-1",
            className
          )}
          {...rest}
        ></Primitive.Popup>
      </Primitive.Positioner>
    </Primitive.Portal>
  );
}

export function SelectGroup({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Group>) {
  return (
    <Primitive.Group className={cn("flex flex-col", className)} {...rest} />
  );
}

export function SelectGroupLabel({
  className,
  ...rest
}: ComponentProps<typeof Primitive.GroupLabel>) {
  return (
    <Primitive.GroupLabel
      className={cn(
        "text text-sm text-gray-400 px-2 py-1 flex flex-row gap-2 items-center",
        className
      )}
      {...rest}
    />
  );
}

export function SelectItems({
  className,
  ...rest
}: ComponentProps<typeof Primitive.List>) {
  return (
    <Primitive.List className={cn("flex flex-col", className)} {...rest} />
  );
}

export function SelectItem({
  className,
  children,
  ...rest
}: ComponentProps<typeof Primitive.Item>) {
  return (
    <Primitive.Item
      className={cn(
        "px-2 h-6 flex flex-row items-center gap-2 text-gray-300 transition-colors hover:bg-gray-700 active:bg-gray-900",
        className
      )}
      {...rest}
    >
      <Primitive.ItemIndicator>
        <CheckIcon className="size-3.5" />
      </Primitive.ItemIndicator>
      <Primitive.ItemText className="only:ml-5.5 text-sm cursor-default">
        {children}
      </Primitive.ItemText>
    </Primitive.Item>
  );
}
