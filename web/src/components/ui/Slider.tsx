import { cn } from "@/utils/cn";
import { Slider as Primitive } from "@base-ui/react/slider";
import type { ComponentProps } from "react";

export function Slider({
  onChange,
  ...rest
}: Omit<ComponentProps<typeof Primitive.Root>, "onChange"> & {
  onChange?: ComponentProps<typeof Primitive.Root>["onValueChange"];
}) {
  return <Primitive.Root onValueChange={onChange} {...rest} />;
}

export function SliderValue({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Value>) {
  return (
    <Primitive.Value
      className={cn("text-sm text-gray-300 font-medium", className)}
      {...rest}
    />
  );
}

export function SliderControl({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Control>) {
  return (
    <Primitive.Control
      className={cn("h-8 flex flex-row items-center", className)}
      {...rest}
    />
  );
}

export function SliderTrack({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Track>) {
  return (
    <Primitive.Track
      className={cn("w-full h-1.5 bg-gray-800 rounded", className)}
      {...rest}
    />
  );
}

export function SliderIndicator({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Indicator>) {
  return (
    <Primitive.Indicator
      className={cn("h-full bg-primary-500 rounded", className)}
      {...rest}
    />
  );
}

export function SliderThumb({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Thumb>) {
  return (
    <Primitive.Thumb
      className={cn(
        "size-4 rounded-full transition-colors bg-gray-300 hover:bg-gray-200 data-dragging:bg-gray-400",
        className
      )}
      {...rest}
    />
  );
}
