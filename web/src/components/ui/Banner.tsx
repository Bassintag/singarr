import { cn } from "@/utils/cn";
import type { ComponentProps } from "react";
import { Button } from "./Button";

export function Banner({ className, ...rest }: ComponentProps<"div">) {
  return <div className={cn("flex flex-col", className)} {...rest} />;
}

export function BannerActions({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn("px-6 py-4 flex flex-row gap-4 bg-gray-700", className)}
      {...rest}
    />
  );
}

export function BannerAction({ ...props }: ComponentProps<typeof Button>) {
  return <Button size="sm" type="button" variant="ghost" {...props} />;
}

export function BannerContent({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn("px-6 py-4 flex flex-row gap-4 bg-gray-900", className)}
      {...rest}
    />
  );
}

export function BannerHeader({ className, ...rest }: ComponentProps<"div">) {
  return <div className={cn("flex flex-col gap-2", className)} {...rest} />;
}

export function BannerTitle({ className, ...rest }: ComponentProps<"h1">) {
  return <h1 className={cn("text-2xl", className)} {...rest} />;
}

export function BannerDescription({
  className,
  ...rest
}: ComponentProps<"div">) {
  return <div className={cn("", className)} {...rest} />;
}
