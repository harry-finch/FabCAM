import { expect, it, vi } from 'vitest';
import { focusMaterialResult } from './material-browser-focus';
it('resolves replacement rows after filtering instead of focusing stale keyed nodes', () => {
  const previous = { focus: vi.fn() },
    current = { focus: vi.fn() };
  let rows = [previous];
  const root = { querySelectorAll: vi.fn(() => rows) };
  focusMaterialResult(root as unknown as HTMLElement, 0);
  rows = [current];
  focusMaterialResult(root as unknown as HTMLElement, 0);
  expect(root.querySelectorAll).toHaveBeenCalledTimes(2);
  expect(previous.focus).toHaveBeenCalledOnce();
  expect(current.focus).toHaveBeenCalledOnce();
});
it('handles an empty result/page without selecting or focusing an unrelated control', () => {
  expect(() => focusMaterialResult(undefined, 0)).not.toThrow();
  expect(() =>
    focusMaterialResult({ querySelectorAll: () => [] } as unknown as HTMLElement, -1),
  ).not.toThrow();
});

it('moves result focus within the current page without selecting records', async () => {
  const { recordFocusIndex } = await import('./material-browser-focus');
  expect(recordFocusIndex('Home', 4, 10)).toBe(0);
  expect(recordFocusIndex('End', 4, 10)).toBe(9);
  expect(recordFocusIndex('ArrowDown', 9, 10)).toBe(9);
  expect(recordFocusIndex('ArrowUp', 0, 10)).toBe(0);
  expect(recordFocusIndex('ArrowDown', 2, 10)).toBe(3);
  expect(recordFocusIndex('Enter', 2, 10)).toBeUndefined();
  expect(recordFocusIndex('End', 0, 0)).toBeUndefined();
});
