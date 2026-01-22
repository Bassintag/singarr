import { SettingsForm } from "@/components/settings/SettingsForm";
import {
  Banner,
  BannerAction,
  BannerActions,
  BannerContent,
  BannerHeader,
  BannerTitle,
} from "@/components/ui/Banner";
import { toastPromise } from "@/hooks/notification/useNotificationState";
import {
  setSettingsMutationOptions,
  settingsQueryOptions,
} from "@/queries/settings";
import { useMutation, useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { SaveIcon } from "lucide-react";
import { useId } from "react";

export const Route = createFileRoute("/(app)/settings/")({
  component: RouteComponent,
});

function RouteComponent() {
  const formId = useId();

  return (
    <>
      <Banner>
        <BannerActions>
          <BannerAction type="submit" form={formId}>
            <SaveIcon />
            Save
          </BannerAction>
        </BannerActions>
        <BannerContent>
          <BannerHeader>
            <BannerTitle>Settings</BannerTitle>
          </BannerHeader>
        </BannerContent>
      </Banner>
      <div className="px-6 py-4">
        <Form formId={formId} />
      </div>
    </>
  );
}

function Form({ formId }: { formId: string }) {
  const { data: settings } = useQuery(settingsQueryOptions());
  const setSettings = useMutation(setSettingsMutationOptions());

  return (
    settings && (
      <SettingsForm
        id={formId}
        defaultValues={settings}
        onSubmit={async (values) => {
          await toastPromise(setSettings.mutateAsync(values), {
            title: "Saving",
            success: "Saved",
          });
        }}
      />
    )
  );
}
