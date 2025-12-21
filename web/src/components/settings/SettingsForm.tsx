import { settingsSchema, type Settings } from "@/domain/settings";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  useForm,
  type DefaultValues,
  type SubmitHandler,
} from "react-hook-form";
import z from "zod";
import { Button } from "../ui/Button";
import {
  Form,
  FormController,
  FormError,
  FormField,
  FormLabel,
} from "../ui/Form";
import { Input } from "../ui/Input";
import {
  NumberField,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from "../ui/NumberField";
import { Separator } from "../ui/Separator";

const settingsFormSchema = settingsSchema;

export type SettingsFormValues = z.infer<typeof settingsFormSchema>;

const resolver = zodResolver(settingsFormSchema);

export function SettingsForm({
  defaultValues,
  onSubmit,
}: {
  defaultValues?: DefaultValues<SettingsFormValues>;
  onSubmit: SubmitHandler<SettingsFormValues>;
}) {
  const form = useForm<Settings>({ defaultValues, resolver });

  return (
    <Form
      form={form}
      onSubmit={(e) => {
        form.handleSubmit(onSubmit)(e);
      }}
    >
      <div>General</div>
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
      <Separator />
      <div>Lidarr</div>
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
      <div>
        <Button type="submit">Submit</Button>
      </div>
    </Form>
  );
}
