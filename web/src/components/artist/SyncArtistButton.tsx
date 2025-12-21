import type { ComponentProps } from "react";
import { Button } from "../ui/Button";
import { useMutation } from "@tanstack/react-query";
import { createJobMutationOptions } from "@/queries/job";

export function SyncArtistButton({
  artistId,
  ...rest
}: ComponentProps<typeof Button> & { artistId: number }) {
  const createJob = useMutation(createJobMutationOptions());

  return (
    <Button
      title="Sync with lidarr"
      disabled={createJob.isPending}
      onClick={() => {
        createJob.mutate({ type: "syncArtist", artistId });
      }}
      {...rest}
    />
  );
}
