import { cn } from "@/utils/cn";
import { Dialog as Primitive } from "@base-ui/react/dialog";
import { type ComponentProps } from "react";

export const Dialog = Primitive.Root;

export const DialogTrigger = Primitive.Trigger;

export function DialogContent({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Popup>) {
  return (
    <Primitive.Portal>
      <Primitive.Backdrop className="z-40 bg-black/20 backdrop-blur-md fixed inset-0" />
      <Primitive.Popup
        className={cn(
          "z-40 fixed left-1/2 top-1/2 -translate-1/2 w-2xl max-w-[calc(100dvw-2em)] max-h-[70dvh] bg-gray-900 rounded border border-gray-700 overflow-auto",
          className
        )}
        {...rest}
      />
    </Primitive.Portal>
  );
}

export function DialogTitle({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Title>) {
  return (
    <Primitive.Title
      className={cn("text-lg text-medium", className)}
      {...rest}
    />
  );
}

export function DialogDescription({
  className,
  ...rest
}: ComponentProps<typeof Primitive.Description>) {
  return (
    <Primitive.Description
      className={cn("text-sm truncate text-gray-400", className)}
      {...rest}
    />
  );
}

export function DialogHeader({ className, ...rest }: ComponentProps<"div">) {
  return (
    <header
      className={cn("p-4 sticky top-0 bg-gray-900", className)}
      {...rest}
    />
  );
}

export function DialogBody({ className, ...rest }: ComponentProps<"div">) {
  return <div className={cn("px-4 bg-gray-950", className)} {...rest} />;
}

export function DialogFooter({ className, ...rest }: ComponentProps<"div">) {
  return (
    <header
      className={cn(
        "p-4 sticky bottom-0 mt-auto flex flex-row gap-2 justify-end bg-gray-900",
        className
      )}
      {...rest}
    />
  );
}

export const DialogClose = Primitive.Close;
