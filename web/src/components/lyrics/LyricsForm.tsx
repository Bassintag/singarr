import { zodResolver } from "@hookform/resolvers/zod";
import {
  useForm,
  type DefaultValues,
  type SubmitHandler,
} from "react-hook-form";
import z from "zod";
import {
  Form,
  FormControl,
  FormController,
  FormError,
  FormField,
  FormLabel,
} from "../ui/Form";
import { TextArea } from "../ui/TextArea";
import type { ComponentProps } from "react";
import { TrackSelect } from "../track/TrackSelect";
import { Route } from "@/routes/(app)/artists/$id";

const lyricsFormSchema = z.object({
  content: z.string().min(1),
  trackId: z.int(),
});

export type LyricsFormValues = z.infer<typeof lyricsFormSchema>;

const resolver = zodResolver(lyricsFormSchema);

export function LyricsForm({
  defaultValues,
  onSubmit,
  ...rest
}: Omit<ComponentProps<typeof Form>, "form" | "onSubmit"> & {
  defaultValues?: DefaultValues<LyricsFormValues>;
  onSubmit: SubmitHandler<LyricsFormValues>;
}) {
  const { id: artistId } = Route.useParams();
  const form = useForm<LyricsFormValues>({ defaultValues, resolver });

  return (
    <Form {...rest} form={form} onSubmit={form.handleSubmit(onSubmit)}>
      <FormController
        control={form.control}
        name="trackId"
        render={({ field }) => (
          <FormField>
            <FormLabel>Track</FormLabel>
            <TrackSelect {...field} defaultValue={null} query={{ artistId }} />
            <FormError />
          </FormField>
        )}
      />
      <FormController
        control={form.control}
        name="content"
        defaultValue=""
        render={({ field }) => (
          <FormField>
            <FormLabel>Content</FormLabel>
            <FormControl
              render={<TextArea className="h-48 resize-none" {...field} />}
            />
            <FormError />
          </FormField>
        )}
      />
    </Form>
  );
}
