import { AppNotifications } from "@/components/notification/Notification";
import { useNotificationState } from "@/hooks/notification/useNotificationState";
import { useSocketListener } from "@/hooks/socket/useSocketListener";
import { isDone } from "@/utils/job";
import { Outlet, createRootRoute } from "@tanstack/react-router";
import { useEffect } from "react";

export const Route = createRootRoute({
  component: RootComponent,
});

function RootComponent() {
  useSocketListener((e) => {
    const state = useNotificationState.getState();
    console.log("Caught event:", e);
    state.handle(e);
  });

  useEffect(() => {
    return useNotificationState.subscribe((state) => {
      const notificaiton = state.queue[0];
      if (notificaiton == null || !isDone(notificaiton.job)) return;
      state.next();
    });
  }, []);

  return (
    <>
      <Outlet />
      <AppNotifications />
    </>
  );
}
