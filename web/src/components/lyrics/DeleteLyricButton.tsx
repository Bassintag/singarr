import type { ComponentProps } from "react";
import { Button } from "../ui/Button";
import { useMutation } from "@tanstack/react-query";
import { deleteLyricMutationOptions } from "@/queries/lyrics";

export function DeleteLyricsButton({
  lyricId,
  ...rest
}: ComponentProps<typeof Button> & {
  lyricId: number;
}) {
  const deleteLyric = useMutation(deleteLyricMutationOptions());

  return (
    <Button
      variant="outline"
      className="text-failure"
      disabled={deleteLyric.isPending}
      onClick={() => {
        deleteLyric.mutate(lyricId);
      }}
      {...rest}
    />
  );
}
