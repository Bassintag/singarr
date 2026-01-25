import type { AppEvent } from "@/domain/event";
import type { Page, Pageable } from "@/domain/generic";
import type { Job, JobPayload } from "@/domain/job";
import { useSocketState } from "@/hooks/socket/useSocketState";
import { fetchApi } from "@/utils/api";
import { invalidateAll } from "@/utils/query";
import { mutationOptions, queryOptions } from "@tanstack/react-query";

export function jobsQueryOptions(query?: Pageable) {
  return queryOptions({
    queryKey: ["jobs", "list", query],
    queryFn: () => {
      return fetchApi<Page<Job>>("jobs", { query });
    },
  });
}

export function jobQueryOptions(id: number) {
  return queryOptions({
    queryKey: ["jobs", "details", id],
    queryFn: () => {
      return fetchApi<Job>(`jobs/${id}`);
    },
  });
}

export function createJobMutationOptions() {
  return mutationOptions({
    mutationFn: async (body: JobPayload) => {
      const buffered: AppEvent[] = [];
      let jobId: number | null = null;

      return new Promise<void>((resolve, reject) => {
        const handleEvent = (e: AppEvent) => {
          console.log("handleEvent", e, jobId)
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
