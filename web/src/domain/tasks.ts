import type { JobPayload } from "./job";

export interface Task {
  cron: string;
  payload: JobPayload;
}
