import type { Notifier } from "@/domain/notifier";
import { notifiersQueryOptions } from "@/queries/notifier";
import { useQuery } from "@tanstack/react-query";
import { sentenceCase } from "change-case";
import { UpdateNotifierDialog } from "./UpdateNotifierDialog";

export function NotifiersGrid() {
  const { data: notifiers } = useQuery(notifiersQueryOptions());

  return (
    <ul className="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
      {notifiers?.map((notifier) => (
        <NotifiersGridItem key={notifier.id} notifier={notifier} />
      ))}
    </ul>
  );
}

function NotifiersGridItem({ notifier }: { notifier: Notifier }) {
  return (
    <li>
      <UpdateNotifierDialog notifier={notifier}>
        <button className="block w-full text-start p-4 bg-gray-900 border-gray-700 border rounded transition-colors hover:bg-gray-800 active:bg-gray-950">
          {sentenceCase(notifier.params.type)}
        </button>
      </UpdateNotifierDialog>
    </li>
  );
}
