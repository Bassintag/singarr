import { SettingsForm } from "@/components/settings/SettingsForm";
import {
  Banner,
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

export const Route = createFileRoute("/settings/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: settings } = useSuspenseQuery(settingsQueryOptions());
  const setSettings = useMutation(setSettingsMutationOptions());

  return (
    <>
      <Banner>
        <BannerContent>
          <BannerHeader>
            <BannerTitle>Settings</BannerTitle>
          </BannerHeader>
        </BannerContent>
      </Banner>
      <div className="p-4">
        <SettingsForm
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
