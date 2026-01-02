import { useId, useState, type ReactElement } from "react";
import {
  Dialog,
  DialogBody,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTrigger,
} from "../ui/Dialog";
import { LyricsForm } from "./LyricsForm";
import { useMutation } from "@tanstack/react-query";
import { createJobMutationOptions } from "@/queries/job";
import { Button } from "../ui/Button";

export function ImportLyricsDialog({ children }: { children: ReactElement }) {
  const createJob = useMutation(createJobMutationOptions());
  const [open, setOpen] = useState(false);
  const formId = useId();

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger render={children} />
      <DialogContent>
        <DialogHeader>
          <DialogHeader>Import lyrics</DialogHeader>
        </DialogHeader>
        <DialogBody>
          <LyricsForm
            id={formId}
            onSubmit={async ({ content, trackId }) => {
              await createJob.mutateAsync(
                {
                  type: "importLyrics",
                  synced: false,
                  trackId,
                  content,
                },
                {
                  onSuccess: () => {
                    setOpen(false);
                  },
                }
              );
            }}
          />
        </DialogBody>
        <DialogFooter>
          <Button type="submit" form={formId}>
            Import
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
