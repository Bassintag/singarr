import { lyricContentQueryOptions, lyricQueryOptions } from "@/queries/lyrics";
import { useQuery } from "@tanstack/react-query";
import {
  Dialog,
  DialogHeader,
  DialogContent,
  DialogDescription,
  DialogTitle,
  DialogTrigger,
  DialogBody,
  DialogFooter,
  DialogClose,
} from "../ui/Dialog";
import { useState, type ReactElement } from "react";
import { Button } from "../ui/Button";
import { DeleteLyricsButton } from "./DeleteLyricButton";

export function LyricsDialog({
  lyricId,
  children,
}: {
  lyricId: number;
  children: ReactElement;
}) {
  const [open, setOpen] = useState(false);
  const { data: lyric } = useQuery({
    ...lyricQueryOptions(lyricId),
    enabled: open,
  });
  const { data: content } = useQuery({
    ...lyricContentQueryOptions(lyricId),
    enabled: open,
  });

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger render={children} />
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Lyrics</DialogTitle>
          <DialogDescription>{lyric?.filePath}</DialogDescription>
        </DialogHeader>
        <DialogBody className="whitespace-pre-wrap font-mono text-sm">
          {content?.text}
        </DialogBody>
        <DialogFooter>
          <DeleteLyricsButton lyricId={lyricId}>Delete</DeleteLyricsButton>
          <DialogClose render={<Button variant="outline">Close</Button>} />
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
