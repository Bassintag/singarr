import type { AppEvent } from "@/domain/event";
import type { Job, JobPayload } from "@/domain/job";
import { useSocketState } from "@/hooks/socket/useSocketState";
import { fetchApi } from "@/utils/api";
import { invalidateAll } from "@/utils/query";
import { mutationOptions } from "@tanstack/react-query";

export function createJobMutationOptions() {
  return mutationOptions({
    mutationFn: async (body: JobPayload) => {
      const buffered: AppEvent[] = [];
      let jobId: number | null = null;

      return new Promise<void>((resolve, reject) => {
        const handleEvent = (e: AppEvent) => {
          if (e.type === "jobEnd" && e.job.id === jobId) {
            unsubscribe();
            resolve();
          }
        };

        const unsubscribe = useSocketState.getState().listen((e) => {
          if (jobId === null) {
            buffered.push(e);
          } else {
            handleEvent(e);
          }
        });

        fetchApi<Job>("/jobs", {
          method: "POST",
          json: body,
        })
          .then((job) => {
            jobId = job.id;
            for (const e of buffered) {
              handleEvent(e);
            }
          })
          .catch((e) => {
            unsubscribe();
            reject(e);
          });
      });
    },
    onSettled: async (_1, _2, _3, _4, { client }) => {
      await invalidateAll(client);
    },
  });
}
