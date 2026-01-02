import { CreateNotifierDialog } from "@/components/notifier/CreateNotifierDialog";
import { NotifiersGrid } from "@/components/notifier/NotifiersGrid";
import { Banner, BannerActions } from "@/components/ui/Banner";
import { Button } from "@/components/ui/Button";
import { createFileRoute } from "@tanstack/react-router";
import { PlusIcon } from "lucide-react";

export const Route = createFileRoute("/(app)/notifiers/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <>
      <Banner>
        <BannerActions>
          <CreateNotifierDialog>
            <Button variant="ghost" size="sm">
              <PlusIcon />
              New
            </Button>
          </CreateNotifierDialog>
        </BannerActions>
      </Banner>
      <div className="px-6 py-4">
        <NotifiersGrid />
      </div>
    </>
  );
}
