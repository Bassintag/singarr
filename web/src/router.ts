import { createRouter } from "@tanstack/react-router";
import { routeTree } from "./routeTree.gen";
import { useSidebarState } from "./hooks/layout/useSidebarState";

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

export const router = createRouter({ routeTree });

router.subscribe("onBeforeNavigate", () => {
  useSidebarState.getState().setOpen(false);
});
