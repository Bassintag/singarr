import { cn } from "@/utils/cn";
import { NumberField as Primitive } from "@base-ui/react";
import { MinusIcon, PlusIcon } from "lucide-react";
import type { ComponentProps } from "react";
import { buttonVariants } from "./Button";
import { InputGroup } from "./InputGroup";
import { Input } from "./Input";

export function NumberField({
  onChange,
  className,
  ...rest
}: ComponentProps<typeof Primitive.Root> & {
  onChange: ComponentProps<typeof Primitive.Root>["onValueChange"];
}) {
  return (
    <Primitive.Root
      className={className}
      onValueChange={onChange}
      render={<InputGroup />}
      {...rest}
    />
  );
}

export function NumberFieldInput({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Input>) {
  return (
    <Primitive.Input
      className={cn("grow text-center", className)}
      {...rest}
      render={<Input />}
    />
  );
}

export function NumberFieldIncrement({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Increment>) {
  return (
    <Primitive.Increment
      className={cn(
        buttonVariants({ size: "icon", variant: "ghost" }),
        "shrink-0",
        className
      )}
      {...rest}
    >
      <PlusIcon />
    </Primitive.Increment>
  );
}

export function NumberFieldDecrement({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Decrement>) {
  return (
    <Primitive.Decrement
      className={cn(
        buttonVariants({ size: "icon", variant: "ghost" }),
        "shrink-0",
        className
      )}
      {...rest}
    >
      <MinusIcon />
    </Primitive.Decrement>
  );
}
