import z from "zod";

export const discordParamsSchema = z.object({
  type: z.literal("discord"),
  webhookUrl: z.url(),
});

export const notifierParamsSchema = z.discriminatedUnion("type", [
  discordParamsSchema,
]);

export type NotifierParams = z.infer<typeof notifierParamsSchema>;

export interface Notifier {
  id: number;
  createdAt: string;
  params: NotifierParams;
}

export const createNotifierSchema = z.object({
  params: notifierParamsSchema,
});

export type CreateNotifier = z.infer<typeof createNotifierSchema>;
