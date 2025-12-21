import type { ComponentProps } from "react";
import { Button } from "../ui/Button";
import { useMutation } from "@tanstack/react-query";
import { createJobMutationOptions } from "@/queries/job";

export function AutomaticSearchAlbumButton({
  albumId,
  ...rest
}: ComponentProps<typeof Button> & { albumId: number }) {
  const createJob = useMutation(createJobMutationOptions());
  return (
    <Button
      type="button"
      title="Automatic Search"
      disabled={createJob.isPending}
      onClick={() => {
        createJob.mutate({ type: "searchAlbum", albumId });
      }}
      {...rest}
    />
  );
}
