import { cn } from "@/utils/cn";
import { Field as FieldPrimitve } from "@base-ui/react/field";
import { Fieldset as FieldsetPrimitve } from "@base-ui/react/fieldset";
import { Form as FormPrimitive } from "@base-ui/react/form";
import { createContext, use, type ComponentProps } from "react";
import {
  Controller,
  FormProvider,
  useFormContext,
  useFormState,
  type FieldPath,
  type FieldValues,
  type UseFormReturn,
} from "react-hook-form";
import { Button } from "./Button";

export function Form<T extends FieldValues>({
  form,
  className,
  children,
  ...rest
}: ComponentProps<typeof FormPrimitive> & { form: UseFormReturn<T> }) {
  return (
    <FormPrimitive className={cn("flex flex-col gap-8", className)} {...rest}>
      <FormProvider {...form}>{children}</FormProvider>
    </FormPrimitive>
  );
}

export function FormGroup({
  className,
  ...rest
}: ComponentProps<typeof FieldsetPrimitve.Root>) {
  return (
    <FieldsetPrimitve.Root
      className={cn("flex flex-col gap-4", className)}
      {...rest}
    />
  );
}

export function FormGroupTitle({
  className,
  ...rest
}: ComponentProps<typeof FieldsetPrimitve.Legend>) {
  return (
    <FieldsetPrimitve.Legend
      className={cn("text-lg pb-2  border-b border-gray-700", className)}
      {...rest}
    />
  );
}

const FormControllerContext = createContext<string>(null as never);

export function FormController<
  TFieldValues extends FieldValues = FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
  TTransformedValues = TFieldValues,
>({
  name,
  ...rest
}: ComponentProps<typeof Controller<TFieldValues, TName, TTransformedValues>>) {
  return (
    <FormControllerContext value={name}>
      <Controller name={name} {...rest} />
    </FormControllerContext>
  );
}

export function FormField({
  className,
  ...rest
}: ComponentProps<typeof FieldPrimitve.Root>) {
  const name = use(FormControllerContext);
  const { getFieldState, formState } = useFormContext();
  const { invalid, isDirty, isTouched } = getFieldState(name, formState);

  return (
    <FieldPrimitve.Root
      dirty={isDirty}
      touched={isTouched}
      invalid={invalid}
      className={cn("flex flex-col gap-1", className)}
      {...rest}
    />
  );
}

export function FormLabel({
  className,
  ...rest
}: ComponentProps<typeof FieldPrimitve.Label>) {
  return (
    <FieldPrimitve.Label
      className={cn("text-sm text-gray-400", className)}
      {...rest}
    />
  );
}

export function FormControl({
  className,
  ...rest
}: ComponentProps<typeof FieldPrimitve.Control>) {
  return <FieldPrimitve.Control className={cn("", className)} {...rest} />;
}

export function FormError({
  className,
  children,
  ...rest
}: ComponentProps<typeof FieldPrimitve.Error>) {
  const name = use(FormControllerContext);
  const { getFieldState, formState } = useFormContext();
  const { error } = getFieldState(name, formState);
  const body = error ? String(error?.message) : children;

  return (
    <FieldPrimitve.Error
      match={error != null}
      className={cn("text-failure text-xs", className)}
      {...rest}
    >
      {body}
    </FieldPrimitve.Error>
  );
}

export function SubmitButton(props: ComponentProps<typeof Button>) {
  const { isSubmitting } = useFormState();

  return <Button type="submit" disabled={isSubmitting} {...props} />;
}
