export function resolveImageUrl(path?: string) {
  if (path == null) return undefined;
  const basePath = import.meta.env.PUBLIC_IMAGES_PATH;
  if (basePath) path = `${basePath}${path}`;
  return new URL(
    path,
    import.meta.env.PUBLIC_IMAGES_URL ?? window.location.href
  ).href;
}
