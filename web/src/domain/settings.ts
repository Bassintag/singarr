import z from "zod";

export const settingsSchema = z.object({
  rootFolder: z.string(),
  lidarr: z.object({
    baseUrl: z.string(),
    httpTimeout: z.int(),
    apiKey: z.string(),
  }),
});

export type Settings = z.infer<typeof settingsSchema>;
