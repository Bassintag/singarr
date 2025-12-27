// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface ViteTypeOptions {}

interface ImportMetaEnv {
  readonly PUBLIC_API_URL?: string;
  readonly PUBLIC_API_PATH?: string;
  readonly PUBLIC_IMAGES_URL?: string;
  readonly PUBLIC_IMAGES_PATH?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
