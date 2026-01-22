import {
  useCurrentNotification,
  useNotificationState,
  type Notification as NotificationType,
} from "@/hooks/notification/useNotificationState";
import { cn } from "@/utils/cn";
import { CheckIcon, LoaderIcon, XIcon } from "lucide-react";
import {
  createContext,
  use,
  useEffect,
  useRef,
  type ComponentProps,
} from "react";
import { AnimatePresence, motion } from "motion/react";

type NotificationContextValue = NotificationType;

const NotificationContext = createContext<NotificationContextValue>(
  null as never
);

function Notification({
  className,
  ...rest
}: ComponentProps<typeof motion.div>) {
  const { status } = use(NotificationContext);

  return (
    <motion.div
      data-status={status}
      initial={{ translateX: "-100%", opacity: 0 }}
      animate={{
        translateX: "0",
        opacity: 100,
        transition: { ease: "easeInOut", duration: 0.25 },
      }}
      exit={{
        translateX: "-100%",
        opacity: 0,
        transition: { ease: "easeOut", duration: 0.25 },
      }}
      className={cn(
        "z-20 fixed bottom-4 left-4 min-w-64 max-w-96 p-4 overflow-hidden flex flex-col gap-2 bg-gray-800 border border-gray-700 rounded transition-colors data-[status=success]:border-success data-[status=error]:border-failure",
        className
      )}
      {...rest}
    />
  );
}

function NotificationTitle({ className, ...rest }: ComponentProps<"div">) {
  const { status, title } = use(NotificationContext);

  return (
    <div
      className={cn(
        "flex flex-row gap-2 items-center text-sm [&>svg]:size-3.5 [&>svg]:shrink-0",
        className
      )}
      {...rest}
    >
      {status === "success" ? (
        <CheckIcon className="text-success" />
      ) : status === "error" ? (
        <XIcon className="text-failure" />
      ) : status === "loading" ? (
        <LoaderIcon className="animate-spin" />
      ) : null}
      <div className="truncate">{title}</div>
    </div>
  );
}

function Notificationmessage({ className, ...rest }: ComponentProps<"div">) {
  const { message } = use(NotificationContext);

  return (
    message && (
      <div className={cn("text-xs text-gray-400", className)} {...rest}>
        {message}
      </div>
    )
  );
}

function NotificationProgress({ className, ...rest }: ComponentProps<"div">) {
  const ref = useRef<HTMLDivElement>(null);
  const removeAt = useNotificationState((s) => s.removeAt);

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

export function AppNotifications() {
  const notification = useCurrentNotification();
  return (
    <AnimatePresence>
      {notification && (
        <NotificationContext value={notification}>
          <Notification key={notification.id}>
            <NotificationTitle />
            <Notificationmessage />
            <NotificationProgress />
          </Notification>
        </NotificationContext>
      )}
    </AnimatePresence>
  );
}
