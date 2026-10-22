/** Resolve current rendered rows on every keypress: keyed filters replace DOM nodes. */
export function focusMaterialResult(root: HTMLElement | undefined, index: number) {
  root?.querySelectorAll<HTMLButtonElement>('button[data-material-option]')[index]?.focus();
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
