import type { ComponentProps } from "react";
import { Button } from "../ui/Button";
import { useMutation } from "@tanstack/react-query";
import { createJobMutationOptions } from "@/queries/job";

export function ScanArtistButton({
  artistId,
  ...rest
}: ComponentProps<typeof Button> & { artistId: number }) {
  const createJob = useMutation(createJobMutationOptions());
  return (
    <Button
      type="button"
      title="Scan files"
      disabled={createJob.isPending}
      onClick={() => {
        createJob.mutate({ type: "scanArtist", artistId });
      }}
      {...rest}
    />
  );
}
