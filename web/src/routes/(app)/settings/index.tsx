import { SettingsForm } from "@/components/settings/SettingsForm";
import {
  Banner,
  BannerAction,
  BannerActions,
  BannerContent,
  BannerHeader,
  BannerTitle,
} from "@/components/ui/Banner";
import {
  setSettingsMutationOptions,
  settingsQueryOptions,
} from "@/queries/settings";
import { useMutation, useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { SaveIcon } from "lucide-react";
import { useId } from "react";

export const Route = createFileRoute("/(app)/settings/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: settings } = useSuspenseQuery(settingsQueryOptions());
  const setSettings = useMutation(setSettingsMutationOptions());
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
      <div className="p-4">
        <SettingsForm
          id={formId}
          defaultValues={settings}
          onSubmit={async (values) => {
            console.log(values);
            await setSettings.mutateAsync(values);
          }}
        />
      </div>
    </>
  );
}
