import { artistQueryOptions } from "@/queries/artist";
import { Route } from "@/routes/(app)/artists/$id";

export function useCurrentArtistQueryOptions() {
  const { id } = Route.useParams();
  return artistQueryOptions(id);
}
