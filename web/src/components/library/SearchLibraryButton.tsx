import type { ComponentProps } from "react";
import { Button } from "../ui/Button";
import { useMutation } from "@tanstack/react-query";
import { createJobMutationOptions } from "@/queries/job";

export function SearchLibraryButton(props: ComponentProps<typeof Button>) {
  const createJob = useMutation(createJobMutationOptions());

  return (
    <Button
      title="Search missing"
      disabled={createJob.isPending}
      onClick={() => {
        createJob.mutate({ type: "searchLibrary" });
      }}
      {...props}
    />
  );
}
