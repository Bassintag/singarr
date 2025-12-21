import type { AppEvent } from "@/domain/event";
import { listenToEvents } from "@/socket";
import { useEffect, useEffectEvent } from "react";

export const useListenToEvents = (onEvent: (event: AppEvent) => void) => {
  const callback = useEffectEvent(onEvent);
  useEffect(() => listenToEvents(callback), []);
};
