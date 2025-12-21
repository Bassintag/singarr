import {
  Navbar,
  NavbarActions,
  NavbarApp,
  NavbarTitle,
} from "@/components/layout/Navbar";
import { Sidebar, SidebarLink } from "@/components/layout/Sidebar";
import { LibrarySearchbar } from "@/components/library/LibrarySearchbar";
import { AppNotifications } from "@/components/notification/Notification";
import { Button } from "@/components/ui/Button";
import { useNotificationState } from "@/hooks/notification/useNotificationState";
import { useListenToEvents } from "@/hooks/useListenToEvents";
import { isDone } from "@/utils/job";
import { Outlet, createRootRoute } from "@tanstack/react-router";
import { Disc3Icon, MusicIcon, SettingsIcon, UserIcon } from "lucide-react";
import { useEffect } from "react";

export const Route = createRootRoute({
  component: RootComponent,
});

function RootComponent() {
  useListenToEvents((e) => {
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
      <Navbar>
        <NavbarApp>
          <NavbarTitle>Singarr</NavbarTitle>
        </NavbarApp>
        <NavbarActions>
          <LibrarySearchbar />
          <Button size="icon" variant="ghost">
            <SettingsIcon />
          </Button>
        </NavbarActions>
      </Navbar>
      <Sidebar>
        <SidebarLink to="/artists">
          <UserIcon />
          Artists
        </SidebarLink>
        <SidebarLink to="/albums">
          <Disc3Icon />
          Albums
        </SidebarLink>
        <SidebarLink to="/tracks">
          <MusicIcon />
          Tracks
        </SidebarLink>
        <SidebarLink to="/settings">
          <SettingsIcon />
          Settings
        </SidebarLink>
      </Sidebar>
      <div className="ml-50 mt-16">
        <Outlet />
      </div>
      <AppNotifications />
    </>
  );
}
