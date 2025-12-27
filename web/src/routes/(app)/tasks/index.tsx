import { TaskTable } from "@/components/task/TaskTable";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/(app)/tasks/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div>
      <TaskTable />
    </div>
  );
}
