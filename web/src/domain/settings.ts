import z from "zod";

export const settingsSchema = z.object({
  rootFolder: z.string(),
  auth: z.union([
    z.object({ enabled: z.literal(false) }),
    z.object({
      enabled: z.literal(true),
      credentials: z.object({
        username: z.string().min(1),
        password: z.string().min(1),
      }),
    }),
  ]),
  lidarr: z.object({
    baseUrl: z.string(),
    httpTimeout: z.int(),
    apiKey: z.string(),
  }),
});

export type Settings = z.infer<typeof settingsSchema>;
