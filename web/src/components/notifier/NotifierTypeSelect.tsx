import type { NotifierParams } from "@/domain/notifier";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectItems,
  SelectTrigger,
} from "../ui/Select";
import type { ComponentProps } from "react";

const items: { value: NotifierParams["type"]; label: string }[] = [
  { value: "discord", label: "Discord" },
];

export function NotifierTypeSelect<
  Multiple extends boolean | undefined = false,
>(
  props: ComponentProps<
    typeof Select<(typeof items)[number]["value"], Multiple>
  >
) {
  return (
    <Select items={items} {...props}>
      <SelectTrigger />
      <SelectContent>
        <SelectItems>
          {items.map((item) => (
            <SelectItem key={item.value} value={item.value}>
              {item.label}
            </SelectItem>
          ))}
        </SelectItems>
      </SelectContent>
    </Select>
  );
}
