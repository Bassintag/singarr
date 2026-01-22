import { cn } from "@/utils/cn";
import type { ComponentProps } from "react";
import { Button } from "./Button";

export function Banner({ className, ...rest }: ComponentProps<"div">) {
  return <div className={cn("flex flex-col", className)} {...rest} />;
}

export function BannerActions({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "p-4 flex flex-row flex-wrap max-md:justify-between gap-4 bg-gray-700 md:px-6",
        className
      )}
      {...rest}
    />
  );
}

export function BannerAction({ ...props }: ComponentProps<typeof Button>) {
  return <Button size="sm" type="button" variant="ghost" {...props} />;
}

export function BannerBackground({
  src,
  className,
  children,
  ...rest
}: ComponentProps<"div"> & { src?: string }) {
  if (src == null) return children;
  return (
    <div
      style={{ backgroundImage: `url(${src})` }}
      className={cn(
        "bg-cover bg-center *:bg-gray-900/90 *:backdrop-blur-sm",
        className
      )}
      {...rest}
    >
      {children}
    </div>
  );
}

export function BannerContent({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn("p-4 flex flex-row gap-6 bg-gray-900 md:px-6", className)}
      {...rest}
    />
  );
}

export function BannerHeader({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div className={cn("flex flex-col gap-2 w-full", className)} {...rest} />
  );
}

export function BannerTitle({ className, ...rest }: ComponentProps<"h1">) {
  return (
    <h1 className={cn("text-4xl max-md:text-center", className)} {...rest} />
  );
}

export function BannerDescription({
  className,
  ...rest
}: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "text-sm whitespace-pre-wrap mt-2 max-md:text-justify",
        className
      )}
      {...rest}
    />
  );
}
