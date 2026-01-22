import type { ComponentProps } from "react";
import { Button } from "../ui/Button";
import { createJobMutationOptions } from "@/queries/job";
import { useMutation } from "@tanstack/react-query";

export function ScanLibraryButton(props: ComponentProps<typeof Button>) {
  const createJob = useMutation(createJobMutationOptions());

  return (
    <Button
      title="Scan disk for existing files"
      disabled={createJob.isPending}
      onClick={() => {
        createJob.mutate({ type: "scanLibrary" });
      }}
      {...props}
    />
  );
}
