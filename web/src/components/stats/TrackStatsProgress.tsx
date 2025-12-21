import type { ComponentProps } from "react";
import { ProgressBar, ProgressLabel, Progress } from "../ui/Progress";
import type { TrackStats } from "@/domain/generic";

export function TrackStatsProgress({
  stats,
  ...rest
}: ComponentProps<typeof Progress> & { stats: TrackStats }) {
  return (
    <Progress {...rest}>
      <ProgressBar
        style={{
          width: `${(stats.withLyricsCount / stats.tracksCount) * 100}%`,
        }}
      />
      <ProgressLabel>
        {stats.withLyricsCount}/{stats.tracksCount}
      </ProgressLabel>
    </Progress>
  );
}
