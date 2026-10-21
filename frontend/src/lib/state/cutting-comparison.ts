/** Presentation only. Missing optional proposals retain current operation values. */
export function comparisonValue(
  current: number | undefined,
  proposed: number | null | undefined,
  applicable = true,
) {
  const value = proposed ?? current;
  return {
    current: applicable ? current : undefined,
    proposed: applicable ? value : undefined,
    applicable,
    retained: applicable && proposed == null && current != null,
    changed: applicable && proposed != null && proposed !== current,
  };
}

/** Keyboard focus movement never changes selection or applies a record. */
export function recordFocusIndex(key: string, index: number, count: number): number | undefined {
  if (!count) return undefined;
  if (key === 'Home') return 0;
  if (key === 'End') return count - 1;
  if (key === 'ArrowDown') return Math.min(count - 1, index + 1);
  if (key === 'ArrowUp') return Math.max(0, index - 1);
  return undefined;
}
