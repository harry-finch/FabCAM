import { it, expect } from 'vitest';
import { paletteShortcut } from './palette-shortcut';
import { resolveShortcut } from './app-menu';
it.each(['MacIntel', 'MacARM', 'macOS'])('displays the macOS modifier for %s', (platform) =>
  expect(paletteShortcut(platform)).toBe('⌘K'),
);
it.each(['Win32', 'Linux x86_64', 'Linux aarch64', ''])(
  'displays the Ctrl modifier for %s',
  (platform) => expect(paletteShortcut(platform)).toBe('Ctrl+K'),
);
it.each([false, true])(
  'keeps the existing shortcut independent of document contents, meta=%s',
  (meta) => {
    const event = {
      key: 'k',
      metaKey: meta,
      ctrlKey: !meta,
      altKey: false,
      shiftKey: false,
      repeat: false,
      isComposing: false,
      target: { tagName: 'BUTTON' },
    } as unknown as KeyboardEvent;
    expect(resolveShortcut(event)).toEqual({ action: 'palette', preventDefault: true });
  },
);
