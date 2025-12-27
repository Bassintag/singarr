import {
  Navbar,
  NavbarApp,
  NavbarTitle,
  NavbarActions,
  NavbarSidebarToggle,
} from "@/components/layout/Navbar";
import { Sidebar, SidebarLink } from "@/components/layout/Sidebar";
import { LibrarySearchbar } from "@/components/library/LibrarySearchbar";
import { Button } from "@/components/ui/Button";
import { createFileRoute, Outlet } from "@tanstack/react-router";
import {
  SettingsIcon,
  UserIcon,
  Disc3Icon,
  MusicIcon,
  ClockIcon,
  CalendarIcon,
} from "lucide-react";

export const Route = createFileRoute("/(app)")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <>
      <Navbar>
        <NavbarSidebarToggle />
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
        <SidebarLink to="/jobs">
          <ClockIcon />
          History
        </SidebarLink>
        <SidebarLink to="/tasks">
          <CalendarIcon />
          Tasks
        </SidebarLink>
        <SidebarLink to="/settings">
          <SettingsIcon />
          Settings
        </SidebarLink>
      </Sidebar>
      <main className="mt-16 min-h-[calc(100dvh-64px)] flex flex-col grow md:ml-50">
        <Outlet />
      </main>
    </>
  );
}
