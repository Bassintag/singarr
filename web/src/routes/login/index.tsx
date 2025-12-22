import {
  CreateTokensForm,
  type CreateTokensFormValues,
} from "@/components/token/CreateTokensForm";
import { useTokenState } from "@/hooks/token/useTokenState";
import { statusQueryOptions } from "@/queries/status";
import { createTokensMutationOptions } from "@/queries/token";
import { router } from "@/router";
import { useMutation, useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute, Navigate } from "@tanstack/react-router";

export const Route = createFileRoute("/login/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: status } = useSuspenseQuery(statusQueryOptions());
  const createToken = useMutation(createTokensMutationOptions());

  const handleSubmit = async (values: CreateTokensFormValues) => {
    await createToken.mutateAsync(values, {
      onSuccess: (pair) => {
        useTokenState.getState().set(pair);
        router.navigate({ to: "/albums" });
      },
    });
  };

  return status.auth ? (
    <div className="h-dvh w-dvw flex items-center justify-center">
      <main className="w-96 p-4 border border-gray-700 bg-gray-900 rounded">
        <CreateTokensForm onSubmit={handleSubmit} />
      </main>
    </div>
  ) : (
    <Navigate to="/" replace />
  );
}
