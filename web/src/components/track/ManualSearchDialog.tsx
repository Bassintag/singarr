import { getProvidersResultsQueryOption } from "@/queries/provider";
import { useQuery } from "@tanstack/react-query";
import { useState, type PropsWithChildren } from "react";
import { Button } from "../ui/Button";
import {
  Dialog,
  DialogBody,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "../ui/Dialog";
import { ProviderResultList } from "./ProviderResultList";
import { LoaderIcon } from "lucide-react";
import { trackQueryOptions } from "@/queries/track";

export function ManualSearchDialog({
  trackId,
  children,
}: { trackId: number } & PropsWithChildren) {
  const [open, setOpen] = useState(false);
  const { data: track } = useQuery({
    ...trackQueryOptions(trackId),
    enabled: open,
  });
  const { data: results } = useQuery({
    ...getProvidersResultsQueryOption({ trackId }),
    enabled: open,
  });

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      {children}
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Manual search</DialogTitle>
          {track && (
            <DialogDescription>
              {track.artist.name} - {track.album.title} - {track.title}
            </DialogDescription>
          )}
        </DialogHeader>
        <DialogBody>
          {results ? (
            <ProviderResultList results={results} />
          ) : (
            <div className="flex flex-row items-center justify-center gap-2 text-gray-400 text-sm py-4">
              <LoaderIcon className="animate-spin size-5.5" />
              Loading providers...
            </div>
          )}
        </DialogBody>
        <DialogFooter>
          <DialogClose render={<Button variant="outline">Close</Button>} />
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
