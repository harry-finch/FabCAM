export type PaletteKey = 'pass' | 'ignore' | 'block' | 'up' | 'down' | 'run' | 'close';
/** Tab always reaches the shared modal trap; IME confirmation never dispatches. */
export function paletteKey(
  event: Pick<KeyboardEvent, 'key' | 'repeat' | 'isComposing' | 'ctrlKey' | 'metaKey' | 'altKey'>,
  browsing: boolean,
  input: boolean,
): PaletteKey {
  if (event.key === 'Tab') return 'pass';
  if (event.isComposing) return 'ignore';
  if (event.repeat) return event.key === 'Enter' ? 'block' : 'ignore';
  if (event.key === 'Escape') return 'close';
  if (event.ctrlKey || event.metaKey || event.altKey) return 'ignore';
  if (browsing && input) {
    if (event.key === 'ArrowUp') return 'up';
    if (event.key === 'ArrowDown') return 'down';
    if (event.key === 'Enter') return 'run';
  }
  return 'ignore';
}
