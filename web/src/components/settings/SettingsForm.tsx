import { settingsSchema, type Settings } from "@/domain/settings";
import { zodResolver } from "@hookform/resolvers/zod";
import type { ComponentProps } from "react";
import {
  useForm,
  useFormContext,
  useWatch,
  type DefaultValues,
  type SubmitHandler,
} from "react-hook-form";
import z from "zod";
import {
  Form,
  FormController,
  FormError,
  FormField,
  FormGroup,
  FormGroupTitle,
  FormLabel,
} from "../ui/Form";
import { Input } from "../ui/Input";
import {
  NumberField,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from "../ui/NumberField";
import { Switch } from "../ui/Switch";
import {
  Slider,
  SliderControl,
  SliderIndicator,
  SliderThumb,
  SliderTrack,
  SliderValue,
} from "../ui/Slider";

const settingsFormSchema = settingsSchema;

export type SettingsFormValues = z.infer<typeof settingsFormSchema>;

const resolver = zodResolver(settingsFormSchema);

export function SettingsForm({
  defaultValues,
  onSubmit,
  ...rest
}: Omit<ComponentProps<typeof Form>, "form" | "onSubmit"> & {
  defaultValues?: DefaultValues<SettingsFormValues>;
  onSubmit: SubmitHandler<SettingsFormValues>;
}) {
  const form = useForm<Settings>({ defaultValues, resolver });

  return (
    <Form
      {...rest}
      form={form}
      onSubmit={(e) => {
        form.handleSubmit(onSubmit)(e);
      }}
    >
      <SettingsFormGeneralGroup />
      <SettingsFormLyricsGroup />
      <SettingsFormAuthGroup />
      <SettingsFormLidarrGroup />
    </Form>
  );
}

function SettingsFormGeneralGroup() {
  const form = useFormContext<SettingsFormValues>();

  return (
    <FormGroup>
      <FormGroupTitle>General</FormGroupTitle>
      <FormController
        control={form.control}
        name="rootFolder"
        render={({ field }) => (
          <FormField>
            <FormLabel>Root dir</FormLabel>
            <Input {...field} />
            <FormError />
          </FormField>
        )}
      />
    </FormGroup>
  );
}

function SettingsFormLyricsGroup() {
  const form = useFormContext<SettingsFormValues>();

  return (
    <FormGroup>
      <FormGroupTitle>Lyrics</FormGroupTitle>
      <FormController
        control={form.control}
        name="lyrics.minScore"
        render={({ field }) => (
          <FormField>
            <Slider
              {...field}
              min={0}
              max={1}
              step={0.01}
              format={{ style: "percent" }}
            >
              <div className="flex flex-row justify-between gap-4">
                <FormLabel>Min Score</FormLabel>
                <SliderValue />
              </div>
              <SliderControl>
                <SliderTrack>
                  <SliderIndicator />
                  <SliderThumb />
                </SliderTrack>
              </SliderControl>
            </Slider>
            <FormError />
          </FormField>
        )}
      />
      <FormController
        control={form.control}
        name="lyrics.upgrade"
        render={({ field }) => (
          <FormField>
            <FormLabel>Upgrade unsynced</FormLabel>
            <Switch {...field} />
            <FormError />
          </FormField>
        )}
      />
    </FormGroup>
  );
}

function SettingsFormAuthGroup() {
  const form = useFormContext<SettingsFormValues>();
  const enabled = useWatch({ control: form.control, name: "auth.enabled" });

  return (
    <FormGroup>
      <FormGroupTitle>Auth</FormGroupTitle>
      <FormController
        control={form.control}
        name="auth.enabled"
        render={({ field }) => (
          <FormField>
            <FormLabel>Enabled</FormLabel>
            <Switch {...field} />
            <FormError />
          </FormField>
        )}
      />
      {enabled && (
        <>
          <FormController
            control={form.control}
            name="auth.credentials.username"
            render={({ field }) => (
              <FormField>
                <FormLabel>Username</FormLabel>
                <Input {...field} />
                <FormError />
              </FormField>
            )}
          />
          <FormController
            control={form.control}
            name="auth.credentials.password"
            render={({ field }) => (
              <FormField>
                <FormLabel>Password</FormLabel>
                <Input type="password" {...field} />
                <FormError />
              </FormField>
            )}
          />
        </>
      )}
    </FormGroup>
  );
}

function SettingsFormLidarrGroup() {
  const form = useFormContext<SettingsFormValues>();

  return (
    <FormGroup>
      <FormGroupTitle>Lidarr</FormGroupTitle>
      <FormController
        control={form.control}
        name="lidarr.baseUrl"
        render={({ field }) => (
          <FormField>
            <FormLabel>URL</FormLabel>
            <Input {...field} />
            <FormError />
          </FormField>
        )}
      />
      <FormController
        control={form.control}
        name="lidarr.apiKey"
        render={({ field }) => (
          <FormField>
            <FormLabel>Api Key</FormLabel>
            <Input {...field} />
            <FormError />
          </FormField>
        )}
      />
      <FormController
        control={form.control}
        name="lidarr.httpTimeout"
        render={({ field }) => (
          <FormField>
            <FormLabel>HTTP Timeout (s)</FormLabel>
            <NumberField {...field} className="w-48">
              <NumberFieldDecrement />
              <NumberFieldInput />
              <NumberFieldIncrement />
            </NumberField>
            <FormError />
          </FormField>
        )}
      />
    </FormGroup>
  );
}
