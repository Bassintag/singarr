import {
  Navbar,
  NavbarApp,
  NavbarTitle,
  NavbarActions,
} from "@/components/layout/Navbar";
import { Sidebar, SidebarLink } from "@/components/layout/Sidebar";
import { LibrarySearchbar } from "@/components/library/LibrarySearchbar";
import { Button } from "@/components/ui/Button";
import { createFileRoute, Outlet } from "@tanstack/react-router";
import { SettingsIcon, UserIcon, Disc3Icon, MusicIcon } from "lucide-react";

export const Route = createFileRoute("/(app)")({
  component: RouteComponent,
});

function RouteComponent() {
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
    </>
  );
}
