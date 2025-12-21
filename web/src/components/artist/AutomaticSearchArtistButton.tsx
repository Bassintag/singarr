import type { ComponentProps } from "react";
import { Button } from "../ui/Button";
import { useMutation } from "@tanstack/react-query";
import { createJobMutationOptions } from "@/queries/job";

export function AutomaticSearchArtistButton({
  artistId,
  ...rest
}: ComponentProps<typeof Button> & { artistId: number }) {
  const createJob = useMutation(createJobMutationOptions());
  return (
    <Button
      type="button"
      title="Automatic Search"
      disabled={createJob.isPending}
      onClick={() => {
        createJob.mutate({ type: "searchArtist", artistId });
      }}
      {...rest}
    />
  );
}
