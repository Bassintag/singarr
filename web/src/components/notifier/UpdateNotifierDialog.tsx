import type { Notifier } from "@/domain/notifier";
import {
  deleteNotifierMutationOptions,
  updateNotifierMutationOptions,
} from "@/queries/notifier";
import { useMutation } from "@tanstack/react-query";
import { useId, useState, type ReactElement } from "react";
import { Button } from "../ui/Button";
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
import { toastPromise } from "@/hooks/notification/useNotificationState";

export function UpdateNotifierDialog({
  notifier,
  children,
}: {
  notifier: Notifier;
  children: ReactElement;
}) {
  const deleteNotifier = useMutation(deleteNotifierMutationOptions());
  const updateNotifier = useMutation(updateNotifierMutationOptions());
  const [open, setOpen] = useState(false);
  const formId = useId();

  const isPending = deleteNotifier.isPending || updateNotifier.isPending;

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger render={children} />
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Update notifier</DialogTitle>
        </DialogHeader>
        <DialogBody>
          <NotifierForm
            id={formId}
            defaultValues={notifier}
            onSubmit={async (values) => {
              await toastPromise(
                updateNotifier.mutateAsync(
                  { ...values, id: notifier.id },
                  {
                    onSuccess: () => {
                      setOpen(false);
                    },
                  }
                ),
                { title: "Saving", success: "Saved" }
              );
            }}
          />
        </DialogBody>
        <DialogFooter>
          <Button
            variant="danger"
            onClick={() => {
              deleteNotifier.mutate(notifier.id);
            }}
            disabled={isPending}
          >
            Delete
          </Button>
          <Button type="submit" form={formId} disabled={isPending}>
            Save
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
