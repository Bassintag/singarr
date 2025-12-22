import { createTokensSchema } from "@/domain/token";
import { zodResolver } from "@hookform/resolvers/zod";
import type { ComponentProps } from "react";
import { useForm, type SubmitHandler } from "react-hook-form";
import type z from "zod";
import {
  Form,
  FormController,
  FormError,
  FormField,
  FormGroup,
  FormGroupTitle,
  FormLabel,
  SubmitButton,
} from "../ui/Form";
import { Input } from "../ui/Input";

const createTokensFormSchema = createTokensSchema;

export type CreateTokensFormValues = z.infer<typeof createTokensFormSchema>;

const resolver = zodResolver(createTokensFormSchema);

export const CreateTokensForm = ({
  onSubmit,
}: Omit<ComponentProps<"form">, "onSubmit"> & {
  onSubmit: SubmitHandler<CreateTokensFormValues>;
}) => {
  const form = useForm<CreateTokensFormValues>({ resolver, mode: "onChange" });

  return (
    <Form form={form} onSubmit={form.handleSubmit(onSubmit)}>
      <FormGroup>
        <FormGroupTitle>Login</FormGroupTitle>
        <FormController
          control={form.control}
          name="username"
          defaultValue=""
          render={({ field }) => (
            <FormField>
              <FormLabel>Username</FormLabel>
              <Input placeholder="Username" {...field} />
              <FormError />
            </FormField>
          )}
        />
        <FormController
          control={form.control}
          name="password"
          defaultValue=""
          render={({ field }) => (
            <FormField>
              <FormLabel>Password</FormLabel>
              <Input placeholder="Password" type="password" {...field} />
              <FormError />
            </FormField>
          )}
        />
        <div>
          <SubmitButton>Submit</SubmitButton>
        </div>
      </FormGroup>
    </Form>
  );
};
