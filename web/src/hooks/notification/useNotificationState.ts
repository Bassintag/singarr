import type { AppEvent } from "@/domain/event";
import type { Job } from "@/domain/job";
import { addSeconds } from "date-fns";
import { produce } from "immer";
import { create } from "zustand";
import { useShallow } from "zustand/shallow";

export interface Notification {
  job: Job;
  log?: string;
  removeAt?: Date;
}

export interface NotificationState {
  queue: Notification[];

  handle: (event: AppEvent) => void;
  next: () => void;
}

export const useNotificationState = create<NotificationState>((set) => ({
  queue: [],

  handle: (event) => {
    set(
      produce((state: NotificationState) => {
        if (event.type === "jobStart") {
          state.queue.push({ job: event.job });
        } else if (event.type === "jobEnd") {
          const index = state.queue.findIndex((n) => n.job.id === event.job.id);
          if (index >= 0) {
            state.queue[index].job = event.job;
          }
        } else if (event.type === "jobLog") {
          const job = state.queue.find((j) => j.job.id === event.jobId);
          if (job) {
            job.log = event.log;
          }
        }
      })
    );
  },

  next: () => {
    set(
      produce((state: NotificationState) => {
        const notification = state.queue[0];
        if (notification.removeAt != null) return;
        const id = notification.job.id;
        notification.removeAt = addSeconds(new Date(), 1);
        setTimeout(() => {
          set(
            produce((state: NotificationState) => {
              const notification = state.queue[0];
              if (notification != null && notification.job.id === id) {
                state.queue.shift();
              }
            })
          );
        }, 1000);
      })
    );
  },
}));

export function useCurrentNotification(): Notification | undefined {
  return useNotificationState(useShallow((s) => s.queue[0]));
}
