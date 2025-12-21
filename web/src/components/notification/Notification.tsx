import type { Job, JobPayload } from "@/domain/job";
import {
  useCurrentNotification,
  useNotificationState,
  type Notification,
} from "@/hooks/notification/useNotificationState";
import { albumQueryOptions } from "@/queries/album";
import { artistQueryOptions } from "@/queries/artist";
import { trackQueryOptions } from "@/queries/track";
import { cn } from "@/utils/cn";
import { useQuery } from "@tanstack/react-query";
import { CheckIcon, LoaderIcon, XIcon } from "lucide-react";
import {
  createContext,
  use,
  useEffect,
  useRef,
  type ComponentProps,
} from "react";

type NotificationContextValue = Notification;

const NotificationContext = createContext<NotificationContextValue>(
  null as never
);

function useNotification<T extends JobPayload["type"]>() {
  return use(NotificationContext) as Notification & {
    job: Job & { payload: { type: T } };
  };
}

function Notification({ className, ...rest }: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "z-20 fixed bottom-4 left-4 min-w-64 max-w-96 p-4 overflow-hidden flex flex-col gap-2 bg-gray-800 border border-gray-700 rounded transition-colors data-[status=done]:border-success data-[status=failed]:border-failure",
        className
      )}
      {...rest}
    />
  );
}

function NotificationTitle({
  className,
  children,
  ...rest
}: ComponentProps<"div">) {
  const {
    job: { status },
  } = useNotification();

  return (
    <div
      className={cn(
        "flex flex-row gap-2 items-center text-sm [&>svg]:size-3.5 [&>svg]:shrink-0",
        className
      )}
      {...rest}
    >
      {status === "done" ? (
        <CheckIcon className="text-success" />
      ) : status === "failed" ? (
        <XIcon className="text-failure" />
      ) : (
        <LoaderIcon className="animate-spin" />
      )}
      <div className="truncate">{children}</div>
    </div>
  );
}

function NotificationProgress({ className, ...rest }: ComponentProps<"div">) {
  const ref = useRef<HTMLDivElement>(null);
  const removeAt = useNotification().removeAt;

  useEffect(() => {
    if (removeAt == null) return;
    const start = new Date().getTime();
    const end = removeAt.getTime();
    const interval = setInterval(() => {
      if (!ref.current) return;
      const now = Date.now();
      ref.current.style.width = `${((now - start) / (end - start)) * 100}%`;
    }, 50);
    return () => {
      clearInterval(interval);
    };
  }, [removeAt, ref]);

  return (
    <div
      className={cn(className, "absolute bottom-0 left-0 right-0 h-1", {
        hidden: removeAt == null,
      })}
      {...rest}
    >
      <div
        className="absolute left-0 top-0 bottom-0 bg-gray-600 transition-[width] duration-50"
        ref={ref}
      />
    </div>
  );
}

function ImportLyricsNotification() {
  const notification = useNotification<"importLyrics">();
  const { data: track } = useQuery(
    trackQueryOptions(notification.job.payload.trackId)
  );
  return (
    <NotificationTitle>
      Importing {track?.title}
      {notification.job.payload.provider && (
        <> from {notification.job.payload.provider}</>
      )}
    </NotificationTitle>
  );
}

function ArtistNotification({ prefix }: { prefix: string }) {
  const { data: artist } = useQuery(
    artistQueryOptions(
      useNotification<"scanArtist" | "searchArtist" | "syncArtist">().job
        .payload.artistId
    )
  );
  return (
    <NotificationTitle>
      {prefix} {artist?.name}
    </NotificationTitle>
  );
}

function AlbumNotification({ prefix }: { prefix: string }) {
  const { data: album } = useQuery(
    albumQueryOptions(
      useNotification<"scanAlbum" | "searchAlbum">().job.payload.albumId
    )
  );
  return (
    <NotificationTitle>
      {prefix} {album?.title}
    </NotificationTitle>
  );
}

function TrackNotification({ prefix }: { prefix: string }) {
  const { data: track } = useQuery(
    trackQueryOptions(
      useNotification<"scanTrack" | "searchTrack">().job.payload.trackId
    )
  );
  return (
    <NotificationTitle>
      {prefix} {track?.title}
    </NotificationTitle>
  );
}

function SyncLibraryNotification() {
  return <NotificationTitle>Syncing library</NotificationTitle>;
}

const elements: Record<JobPayload["type"], React.ReactElement> = {
  importLyrics: <ImportLyricsNotification />,
  scanArtist: <ArtistNotification prefix="Scanning" />,
  scanAlbum: <AlbumNotification prefix="Scanning" />,
  scanTrack: <TrackNotification prefix="Scanning" />,
  searchArtist: <ArtistNotification prefix="Searching" />,
  searchAlbum: <AlbumNotification prefix="Searching" />,
  searchTrack: <TrackNotification prefix="Searching" />,
  syncLibrary: <SyncLibraryNotification />,
  syncArtist: <ArtistNotification prefix="Syncing" />,
};

export function AppNotifications() {
  const notificaiton = useCurrentNotification();
  if (notificaiton == null) return null;
  const element = elements[notificaiton.job.payload.type];
  if (element == null) return null;
  let description: string | undefined;
  switch (notificaiton.job.status) {
    case "failed":
      description = notificaiton.job.error;
      break;
    case "running":
      description = notificaiton.log;
      break;
  }
  return (
    <Notification
      key={notificaiton.job.id}
      data-status={notificaiton.job.status}
      onClick={() => useNotificationState.getState().next()}
    >
      <NotificationContext value={notificaiton}>
        {element}
        {description != null && (
          <div className="text-gray-500 text-xs">{description}</div>
        )}
        <NotificationProgress />
      </NotificationContext>
    </Notification>
  );
}
