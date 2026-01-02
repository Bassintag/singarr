import { useId, useState, type ReactElement } from "react";
import {
  Dialog,
  DialogBody,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "../ui/Dialog";
import { NotifierForm } from "./NotifierForm";
import { Button } from "../ui/Button";
import { useMutation } from "@tanstack/react-query";
import { createNotifierMutationOptions } from "@/queries/notifier";

export function CreateNotifierDialog({ children }: { children: ReactElement }) {
  const createNotifier = useMutation(createNotifierMutationOptions());
  const [open, setOpen] = useState(false);
  const formId = useId();

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger render={children} />
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New notifier</DialogTitle>
        </DialogHeader>
        <DialogBody>
          <NotifierForm
            id={formId}
            onSubmit={async (values) => {
              await createNotifier.mutateAsync(values, {
                onSuccess: () => {
                  setOpen(false);
                },
              });
            }}
          />
        </DialogBody>
        <DialogFooter>
          <Button
            type="submit"
            form={formId}
            disabled={createNotifier.isPending}
          >
            Save
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
