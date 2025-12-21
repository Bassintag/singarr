import type { Job } from "@/domain/job";

export function isDone(job: Pick<Job, "status">) {
  return job.status === "done" || job.status === "failed";
}
