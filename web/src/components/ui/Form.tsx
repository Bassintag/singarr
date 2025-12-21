import { cn } from "@/utils/cn";
import { Field as FieldPrimitve } from "@base-ui/react/field";
import { Form as FormPrimitive } from "@base-ui/react/form";
import { createContext, use, type ComponentProps } from "react";
import {
  Controller,
  FormProvider,
  useForm,
  type FieldPath,
  type FieldValues,
  type UseFormReturn,
} from "react-hook-form";

export function Form<T extends FieldValues>({
  form,
  className,
  children,
  ...rest
}: ComponentProps<typeof FormPrimitive> & { form: UseFormReturn<T> }) {
  return (
    <FormPrimitive className={cn("flex flex-col gap-4", className)} {...rest}>
      <FormProvider {...form}>{children}</FormProvider>
    </FormPrimitive>
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
  return (
    <FieldPrimitve.Root
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
  const { getFieldState, formState } = useForm();
  const { error } = getFieldState(name, formState);
  const body = error ? String(error?.message) : children;

  if (!body) {
    return null;
  }
  return (
    <FieldPrimitve.Error className={cn("text-failure", className)} {...rest}>
      {body}
    </FieldPrimitve.Error>
  );
}
