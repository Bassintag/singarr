import { AppNotifications } from "@/components/notification/Notification";
import { Outlet, createRootRoute } from "@tanstack/react-router";

export const Route = createRootRoute({
  component: RootComponent,
});

function RootComponent() {
  return (
    <>
      <Outlet />
      <AppNotifications />
    </>
  );
}
