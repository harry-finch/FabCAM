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
