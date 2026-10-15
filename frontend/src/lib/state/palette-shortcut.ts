/** Display only: resolveShortcut remains the sole keyboard mapping. */
export function paletteShortcut(
  platform = typeof navigator === 'undefined' ? '' : navigator.platform,
): string {
  return /^mac/i.test(platform) ? '⌘K' : 'Ctrl+K';
}
