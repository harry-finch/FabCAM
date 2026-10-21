/** Resolve current rendered rows on every keypress: keyed filters replace DOM nodes. */
export function focusMaterialResult(root: HTMLElement | undefined, index: number) {
  root?.querySelectorAll<HTMLButtonElement>('button[data-material-option]')[index]?.focus();
}
