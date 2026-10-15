import { it, expect } from 'vitest';
import { paletteKey } from './palette-keyboard';
const event = (key: string) => ({
  key,
  repeat: false,
  isComposing: false,
  ctrlKey: false,
  metaKey: false,
  altKey: false,
});
it('leaves Tab and Shift-Tab to the modal focus trap', () =>
  expect(paletteKey(event('Tab'), true, true)).toBe('pass'));
it('dispatches only unmodified search Enter and arrows', () => {
  expect(paletteKey(event('Enter'), true, true)).toBe('run');
  expect(paletteKey(event('ArrowDown'), true, true)).toBe('down');
  expect(paletteKey(event('ArrowUp'), true, true)).toBe('up');
  expect(paletteKey({ ...event('Enter'), ctrlKey: true }, true, true)).toBe('ignore');
  expect(paletteKey(event('Enter'), false, false)).toBe('ignore');
});
it('keeps IME confirmation editable and refuses held Enter', () => {
  expect(paletteKey({ ...event('Enter'), isComposing: true }, true, true)).toBe('ignore');
  expect(paletteKey({ ...event('Enter'), repeat: true }, false, false)).toBe('block');
  expect(paletteKey({ ...event('Escape'), isComposing: true }, true, true)).toBe('ignore');
});
it('allows explicit Escape in both views', () => {
  expect(paletteKey(event('Escape'), true, true)).toBe('close');
  expect(paletteKey(event('Escape'), false, false)).toBe('close');
});
