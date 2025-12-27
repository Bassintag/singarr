import { JobTable } from "@/components/job/JobTable";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/(app)/jobs/")({
  component: RouteComponent,
});

function RouteComponent() {
  return <JobTable />;
}
