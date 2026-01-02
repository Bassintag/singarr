import { createNotifierSchema } from "@/domain/notifier";
import { zodResolver } from "@hookform/resolvers/zod";
import type { ComponentProps } from "react";
import {
  useForm,
  useFormContext,
  useWatch,
  type DefaultValues,
  type SubmitHandler,
} from "react-hook-form";
import type z from "zod";
import {
  Form,
  FormController,
  FormError,
  FormField,
  FormGroup,
  FormLabel,
} from "../ui/Form";
import { Input } from "../ui/Input";
import { NotifierTypeSelect } from "./NotifierTypeSelect";
import { cn } from "@/utils/cn";

const notifierFormSchema = createNotifierSchema;

export type NotifierFormValues = z.infer<typeof notifierFormSchema>;

const resolver = zodResolver(notifierFormSchema);

export function NotifierForm({
  className,
  defaultValues,
  onSubmit,
  ...rest
}: Omit<ComponentProps<"form">, "onSubmit"> & {
  defaultValues?: DefaultValues<NotifierFormValues>;
  onSubmit: SubmitHandler<NotifierFormValues>;
}) {
  const form = useForm<NotifierFormValues>({ defaultValues, resolver });

  return (
    <Form
      form={form}
      onSubmit={form.handleSubmit(onSubmit)}
      className={cn("gap-4", className)}
      autoComplete="off"
      {...rest}
    >
      <FormController
        control={form.control}
        name="params.type"
        render={({ field }) => (
          <FormField>
            <FormLabel>Type</FormLabel>
            <NotifierTypeSelect {...field} />
            <FormError />
          </FormField>
        )}
      />
      <FormGroup>
        <NotifierFormParamsGroup />
      </FormGroup>
    </Form>
  );
}

function NotifierFormParamsGroup() {
  const form = useFormContext<NotifierFormValues>();
  const type = useWatch({ control: form.control, name: "params.type" });
  switch (type) {
    case "discord":
      return <NotifierFormDiscordParamsGroup />;
  }
}

function NotifierFormDiscordParamsGroup() {
  const form = useFormContext<NotifierFormValues>();

  return (
    <>
      <FormController
        control={form.control}
        name="params.webhookUrl"
        defaultValue=""
        render={({ field }) => (
          <FormField>
            <FormLabel>Webhook URL</FormLabel>
            <Input
              placeholder="https://discord.com/api/webhooks/..."
              {...field}
            />
            <FormError />
          </FormField>
        )}
      />
    </>
  );
}
