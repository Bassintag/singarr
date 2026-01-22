import type { JobEndEvent, JobLogEvent, JobStartEvent } from "@/domain/event";
import type { Job } from "@/domain/job";
import { addMilliseconds } from "date-fns";
import { produce } from "immer";
import type { ReactNode } from "react";
import * as uuid from "uuid";
import { create } from "zustand";
import { useShallow } from "zustand/shallow";
import { useSocketState } from "../socket/useSocketState";
import { JobNotificationTitle } from "@/components/notification/JobNotification";

export interface Notification {
  id: string;
  title?: ReactNode;
  message?: ReactNode;
  status: "default" | "loading" | "success" | "error";
  duration: number | null;
}

export type CreateNotification = Omit<Notification, "id" | "duration"> & {
  duration?: Notification["duration"];
};
export type UpdateNotification = Partial<CreateNotification>;

export interface NotificationState {
  queue: Notification[];
  removeAt: Date | null;

  push: (data: CreateNotification) => Notification;
  update: (id: string, data: UpdateNotification) => void;
  remove: (id: string) => void;
}

export const useNotificationState = create<NotificationState>((set) => ({
  queue: [],
  removeAt: null,

  push: ({ duration = 2_000, ...notification }) => {
    const data = { ...notification, duration, id: uuid.v4() };
    set(
      produce((state: NotificationState) => {
        state.queue.push(data);
      })
    );
    return data;
  },

  update: (id, data) => {
    set(
      produce((state: NotificationState) => {
        const index = state.queue.findIndex((n) => n.id === id);
        if (index >= 0) {
          const current = state.queue[index];
          Object.assign(current, data);
          state.queue[index] = current;
        }
      })
    );
  },

  remove: (id) => {
    set(
      produce((state: NotificationState) => {
        const index = state.queue.findIndex((n) => n.id === id);
        if (index >= 0) {
          state.queue.splice(index, 1);
        }
      })
    );
  },
}));

useNotificationState.subscribe((state) => {
  if (state.removeAt != null || state.queue.length === 0) return;
  const notification = state.queue[0];
  if (!notification.duration) return;
  useNotificationState.setState({
    removeAt: addMilliseconds(new Date(), notification.duration),
  });
  setTimeout(() => {
    useNotificationState.setState(
      produce((state: NotificationState) => {
        state.queue.shift();
        state.removeAt = null;
      })
    );
  }, notification.duration);
});

export function useCurrentNotification(): Notification | undefined {
  return useNotificationState(useShallow((s) => s.queue[0]));
}

export function toast(data: CreateNotification) {
  return useNotificationState.getState().push(data);
}

export function toastPromise<T>(
  promise: Promise<T>,
  {
    success = "Done",
    error = "Error",
    ...data
  }: Omit<CreateNotification, "status"> & {
    success?: ReactNode;
    error?: ReactNode;
  }
) {
  return new Promise<T>((resolve, reject) => {
    const state = useNotificationState.getState();
    const notification = state.push({
      status: "loading",
      ...data,
    });
    promise.then((value) => {
      state.update(notification.id, { title: success, status: "success" });
      resolve(value);
    });
    promise.catch((e) => {
      state.update(notification.id, { title: error, status: "error" });
      reject(e);
    });
  });
}

// Jobs

const pendingJobs = new Map<number, string>();

function jobToNotification(job: Job) {
  let duration: CreateNotification["duration"] = null;
  let status: CreateNotification["status"];
  let message: CreateNotification["message"] = undefined;
  switch (job.status) {
    case "pending":
      status = "default";
      break;
    case "running":
      status = "loading";
      break;
    case "failed":
      duration = 5_000;
      status = "error";
      message = job.error;
      break;
    case "done":
      duration = 1_000;
      status = "success";
      break;
  }
  return {
    title: JobNotificationTitle({ job }),
    status,
    duration,
    message,
  } satisfies CreateNotification;
}

function handleJobStart(e: JobStartEvent) {
  const state = useNotificationState.getState();
  const notification = state.push(jobToNotification(e.job));
  pendingJobs.set(e.job.id, notification.id);
}

function handleJobEnd(e: JobEndEvent) {
  const notificationId = pendingJobs.get(e.job.id);
  if (notificationId == null) return;
  const state = useNotificationState.getState();
  state.update(notificationId, jobToNotification(e.job));
  pendingJobs.delete(e.job.id);
}

function handleJobLog(e: JobLogEvent) {
  const notificationId = pendingJobs.get(e.jobId);
  if (notificationId == null) return;
  const state = useNotificationState.getState();
  state.update(notificationId, { message: e.log });
}

useSocketState.getState().listen((e) => {
  switch (e.type) {
    case "jobStart":
      handleJobStart(e);
      break;
    case "jobEnd":
      handleJobEnd(e);
      break;
    case "jobLog":
      handleJobLog(e);
      break;
  }
});
