import type { CreateNotifier, Notifier } from "@/domain/notifier";
import { fetchApi } from "@/utils/api";
import { mutationOptions, queryOptions } from "@tanstack/react-query";

export function notifiersQueryOptions() {
  return queryOptions({
    queryKey: ["notifiers", "list"],
    queryFn: () => fetchApi<Notifier[]>(`notifiers`),
  });
}

export function notifierQueryOptions(id: number) {
  return queryOptions({
    queryKey: ["notifiers", "details", id],
    queryFn: () => fetchApi<Notifier>(`notifiers/${id}`),
  });
}

export function createNotifierMutationOptions() {
  return mutationOptions({
    mutationFn: (data: CreateNotifier) => {
      return fetchApi<Notifier>(`notifiers`, { method: "POST", json: data });
    },
    onSuccess: async (data, _variables, _onResult, { client }) => {
      client.setQueryData(["notifiers", "details", data.id], data);
      await client.invalidateQueries({ queryKey: ["notifiers", "list"] });
    },
  });
}

export function updateNotifierMutationOptions() {
  return mutationOptions({
    mutationFn: ({ id, ...data }: CreateNotifier & { id: number }) => {
      return fetchApi<Notifier>(`notifiers/${id}`, {
        method: "PUT",
        json: data,
      });
    },
    onSuccess: async (data, _variables, _onResult, { client }) => {
      client.setQueryData(["notifiers", "details", data.id], data);
      await client.invalidateQueries({ queryKey: ["notifiers", "list"] });
    },
  });
}

export function deleteNotifierMutationOptions() {
  return mutationOptions({
    mutationFn: (id: number) => {
      return fetchApi<void>(`notifiers/${id}`, { method: "DELETE" });
    },
    onSuccess: async (_data, id, _onResult, { client }) => {
      await client.invalidateQueries({ queryKey: ["notifiers", "list"] });
      client.removeQueries({ queryKey: ["notifiers", "details", id] });
    },
  });
}
