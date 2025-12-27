import { cn } from "@/utils/cn";
import type { ComponentProps } from "react";
import { Button } from "../ui/Button";
import { useSidebarState } from "@/hooks/layout/useSidebarState";
import { MenuIcon, XIcon } from "lucide-react";

export function Navbar({ className, ...rest }: ComponentProps<"header">) {
  return (
    <header
      className={cn(
        "fixed z-10 left-0 right-0 top-0 h-16 px-4 flex flex-row items-center gap-4 bg-gray-900 border-b border-gray-700 md:px-6",
        className
      )}
      {...rest}
    />
  );
}

export function NavbarSidebarToggle({
  className,
  ...rest
}: ComponentProps<typeof Button>) {
  const { open, setOpen } = useSidebarState();

  return (
    <Button
      variant="ghost"
      size="icon"
      className={cn("md:hidden", className)}
      onClick={() => setOpen(!open)}
      {...rest}
    >
      {open ? <XIcon /> : <MenuIcon />}
    </Button>
  );
}

export function NavbarApp({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "flex flex-row items-center gap-4 max-md:hidden",
        className
      )}
      {...rest}
    />
  );
}

export function NavbarTitle({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "bg-primary-900 text-white px-3 py-1 rounded text-sm uppercase",
        className
      )}
      {...rest}
    />
  );
}

export function NavbarActions({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "flex flex-row items-center gap-4 max-md:grow md:ml-auto",
        className
      )}
      {...rest}
    />
  );
}
