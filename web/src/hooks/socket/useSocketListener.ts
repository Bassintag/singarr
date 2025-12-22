import type { AppEvent } from "@/domain/event";
import { useEffect, useEffectEvent } from "react";
import { useSocketState } from "./useSocketState";

export const useSocketListener = (onEvent: (event: AppEvent) => void) => {
  const listen = useSocketState((s) => s.listen);
  const callback = useEffectEvent(onEvent);

  useEffect(() => {
    return listen(callback);
  }, [listen]);
};
