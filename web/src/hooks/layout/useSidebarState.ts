import { create } from "zustand";

export interface SidebarState {
  open: boolean;
  setOpen(open: boolean): void;
}

export const useSidebarState = create<SidebarState>((set) => ({
  open: false,
  setOpen: (open) => {
    set({ open });
  },
}));
