import type { Job } from "./job";

export interface JobStartEvent {
  type: "jobStart";
  job: Job;
}

export interface JobEndEvent {
  type: "jobEnd";
  job: Job;
}

export interface JobLogEvent {
  type: "jobLog";
  jobId: number;
  log: string;
}

export type AppEvent = JobStartEvent | JobEndEvent | JobLogEvent;
