import { resolveImageUrl } from "@/utils/image";
import type { ComponentProps } from "react";

export function AppImage({ src, ...rest }: ComponentProps<"img">) {
  return <img src={resolveImageUrl(src)} {...rest} />;
}
