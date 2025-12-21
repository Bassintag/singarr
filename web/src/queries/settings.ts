import type { Settings } from "@/domain/settings";
import { fetchApi } from "@/utils/api";
import { mutationOptions, queryOptions } from "@tanstack/react-query";

export function settingsQueryOptions() {
  return queryOptions({
    queryKey: ["settings"],
    queryFn: () => fetchApi<Settings>("settings"),
  });
}

export function setSettingsMutationOptions() {
  return mutationOptions({
    mutationFn: (body: Settings) => {
      return fetchApi<Settings>("settings", { method: "PUT", json: body });
    },
    onSuccess: (data, _variables, _onResult, { client }) => {
      client.setQueryData(["settings"], data);
    },
  });
}
