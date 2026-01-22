import {
  Navbar,
  NavbarApp,
  NavbarTitle,
  NavbarActions,
  NavbarSidebarToggle,
} from "@/components/layout/Navbar";
import { Sidebar, SidebarLink, SidebarTag } from "@/components/layout/Sidebar";
import { LibrarySearchbar } from "@/components/library/LibrarySearchbar";
import { Button } from "@/components/ui/Button";
import { Menu, MenuContent, MenuItem, MenuTrigger } from "@/components/ui/Menu";
import { useTokenState } from "@/hooks/token/useTokenState";
import { countsStatsQueryOptions } from "@/queries/stats";
import { statusQueryOptions } from "@/queries/status";
import { router } from "@/router";
import { useQuery, useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute, Link, Outlet } from "@tanstack/react-router";
import {
  SettingsIcon,
  UserIcon,
  Disc3Icon,
  MusicIcon,
  ClockIcon,
  CalendarIcon,
  BellIcon,
  LogOutIcon,
  TriangleAlertIcon,
} from "lucide-react";
import { Suspense } from "react";

export const Route = createFileRoute("/(app)")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: stats } = useQuery(countsStatsQueryOptions());

  return (
    <>
      <Navbar>
        <NavbarSidebarToggle />
        <NavbarApp>
          <Link to="/">
            <NavbarTitle>Singarr</NavbarTitle>
          </Link>
        </NavbarApp>
        <NavbarActions>
          <LibrarySearchbar />
          <Suspense>
            <SettingsMenu />
          </Suspense>
        </NavbarActions>
      </Navbar>
      <Sidebar>
        <SidebarLink to="/artists">
          <UserIcon />
          Artists
          {stats && <SidebarTag>{stats.artist}</SidebarTag>}
        </SidebarLink>
        <SidebarLink to="/albums">
          <Disc3Icon />
          Albums
          {stats && <SidebarTag>{stats.album}</SidebarTag>}
        </SidebarLink>
        <SidebarLink to="/tracks">
          <MusicIcon />
          Tracks
          {stats && <SidebarTag>{stats.track}</SidebarTag>}
        </SidebarLink>
        <SidebarLink to="/wanted">
          <TriangleAlertIcon />
          Wanted
          {stats && <SidebarTag>{stats.wanted}</SidebarTag>}
        </SidebarLink>
        <SidebarLink to="/jobs">
          <ClockIcon />
          History
        </SidebarLink>
        <SidebarLink to="/tasks">
          <CalendarIcon />
          Tasks
        </SidebarLink>
        <SidebarLink to="/notifiers">
          <BellIcon />
          Notifications
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

function SettingsMenu() {
  const { data: status } = useSuspenseQuery(statusQueryOptions());
  const setTokens = useTokenState((s) => s.set);

  return (
    status.auth && (
      <Menu>
        <MenuTrigger
          render={
            <Button size="icon" variant="ghost">
              <SettingsIcon />
            </Button>
          }
        />
        <MenuContent>
          <MenuItem
            onClick={() => {
              setTokens(null);
              router.navigate({ to: "/login" });
            }}
          >
            <LogOutIcon /> Log Out
          </MenuItem>
        </MenuContent>
      </Menu>
    )
  );
}
