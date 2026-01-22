const format = Intl.NumberFormat("en-US", { style: "percent" });

export function formatPercent(value: number) {
  return format.format(value);
}
