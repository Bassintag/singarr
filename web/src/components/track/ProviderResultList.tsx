import type { ProviderResult } from "@/domain/provider";
import { Button } from "../ui/Button";
import {
  CheckIcon,
  DownloadIcon,
  EyeIcon,
  EyeOffIcon,
  XIcon,
} from "lucide-react";
import { formatPercent } from "@/utils/format";
import { formatDuration, intervalToDuration } from "date-fns";
import { Tag } from "../ui/Tag";
import { Accordion } from "@base-ui/react/accordion";
import { AnimatePresence, motion } from "motion/react";

export function ProviderResultList({ results }: { results: ProviderResult[] }) {
  return (
    <Accordion.Root
      render={
        <ol className="flex flex-col gap-2">
          {results.map((result) => (
            <ProviderResultListItem
              key={`${result.file.identifier}`}
              result={result}
            />
          ))}
        </ol>
      }
    />
  );
}

export function ProviderResultListItem({ result }: { result: ProviderResult }) {
  return (
    <Accordion.Item
      render={
        <div className="rounded border border-gray-700 overflow-hidden">
          <li className="flex flex-row gap-2 items-center p-2">
            <div className="grow flex flex-col gap-1">
              <div className="text-sm">
                {result.file.artistName} - {result.file.albumTitle} -{" "}
                {result.file.name}
              </div>
              <div className="flex flex-row items-center gap-2 text-sm">
                <Tag>{result.provider.name}</Tag>
                <Tag variant="secondary">
                  {result.file.synced ? (
                    <>
                      <CheckIcon className="text-success" />
                      Synced
                    </>
                  ) : (
                    <>
                      <XIcon className="text-failure" />
                      Plain
                    </>
                  )}
                </Tag>
                <div className="text-xs text-gray-400">
                  {formatPercent(result.score)} match -{" "}
                  {formatDuration(
                    intervalToDuration({
                      start: 0,
                      end: result.file.durationMs,
                    })
                  )}
                </div>
              </div>
            </div>
            <div className="flex flex-row gap-2">
              <Accordion.Trigger
                render={
                  <Button className="group" size="icon-sm" variant="outline">
                    <EyeIcon className="group-aria-expanded:hidden" />
                    <EyeOffIcon className="not-group-aria-expanded:hidden" />
                  </Button>
                }
              />
              <Button size="icon-sm" variant="outline">
                <DownloadIcon />
              </Button>
            </div>
          </li>
          <AnimatePresence>
            <Accordion.Panel
              className="p-2 bg-gray-900 font-mono text-xs whitespace-pre-wrap overflow-auto max-h-32"
              render={<motion.div>{result.file.content}</motion.div>}
            />
          </AnimatePresence>
        </div>
      }
    />
  );
}
