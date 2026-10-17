import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { GeneratedState } from './generated.svelte';
// Logic-only harness: identity shim for rune storage; rendered reactivity is checked separately.
beforeEach(() => vi.stubGlobal('$state', <T>(value: T) => value));
afterEach(() => vi.unstubAllGlobals());
it('completing one operation does not advance its index beyond the total', () => {
  const state = new GeneratedState();
  state.beginGenerate();
  state.notePipelineEvent({ kind: 'op_started', idx: 0, total: 1, name: 'Contour' });
  state.notePipelineEvent({ kind: 'op_completed', cached: true });
  expect(state.pipelineProgress).toMatchObject({ opIdx: 0, opTotal: 1, opFraction: 1 });
  expect(state.lastGenerateCachedCount).toBe(1);
  state.finishGenerate();
  state.endGenerate();
  expect(state.pipelineState).toBe('idle');
  expect(state.generating).toBe(false);
});
it('a new request invalidates previous export identity even if it subsequently fails', () => {
  const state = new GeneratedState();
  state.inputContextKey = 'old';
  state.beginGenerate();
  state.endGenerate();
  expect(state.inputContextKey).toBeNull();
});
