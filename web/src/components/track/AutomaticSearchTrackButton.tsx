import type { ComponentProps } from "react";
import { Button } from "../ui/Button";
import { useMutation } from "@tanstack/react-query";
import { createJobMutationOptions } from "@/queries/job";

export function AutomaticSearchTrackButton({
  trackId,
  ...rest
}: ComponentProps<typeof Button> & { trackId: number }) {
  const createJob = useMutation(createJobMutationOptions());
  return (
    <Button
      type="button"
      title="Automatic Search"
      disabled={createJob.isPending}
      onClick={() => {
        createJob.mutate({ type: "searchTrack", trackId });
      }}
      {...rest}
    />
  );
}
