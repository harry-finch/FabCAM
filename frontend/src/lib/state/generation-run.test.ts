import { describe, it, expect } from 'vitest';
import { GenerationRuns, elapsedSeconds } from './generation-run';

describe('request-scoped generation timing', () => {
  function setup() {
    let now = 100;
    return {
      runs: new GenerationRuns(() => now),
      advance: (ms: number) => {
        now += ms;
      },
      now: () => now,
    };
  }
  it('starts immediately, advances monotonically and freezes the successful duration', () => {
    const t = setup();
    const a = t.runs.start(1, 'A')!;
    expect(elapsedSeconds(a, t.now())).toBe('0.0');
    t.advance(4800);
    expect(elapsedSeconds(a, t.now())).toBe('4.8');
    t.advance(600);
    const done = t.runs.finish(a, 1, 'success')!;
    t.advance(9000);
    expect(elapsedSeconds(done, t.now())).toBe('5.4');
  });
  for (const outcome of ['failed', 'cancelled', 'obsolete', 'blocked'] as const) {
    it(`freezes ${outcome} timing and permits a fresh run`, () => {
      const t = setup();
      const a = t.runs.start(1, 'A')!;
      t.advance(1250);
      const done = t.runs.finish(a, 1, outcome)!;
      expect(done.outcome).toBe(outcome);
      expect(elapsedSeconds(done, 90000)).toBe('1.3');
      const b = t.runs.start(1, 'B')!;
      expect(b.id).not.toBe(a.id);
      expect(elapsedSeconds(b, t.now())).toBe('0.0');
      expect(t.runs.finish(a, 1, 'success')).toBeNull();
      expect(t.runs.owns(b, 1)).toBe(true);
    });
  }
  it('refuses concurrent starts, including cancellation still awaiting acknowledgement', () => {
    const t = setup();
    const a = t.runs.start(1, 'A')!;
    expect(t.runs.start(1, 'A')).toBeNull();
    expect(t.runs.owns(a, 1)).toBe(true);
  });
  it('rejects modified inputs before accepting an output', () => {
    const t = setup();
    const a = t.runs.start(1, 'A')!;
    expect(t.runs.accepts(a, 1, 'B')).toBe(false);
    expect(t.runs.accepts(a, 1, 'A')).toBe(true);
  });
  it('invalidates old callbacks on project switch or teardown, even for identical inputs', () => {
    const t = setup();
    const a = t.runs.start(1, 'A')!;
    expect(t.runs.owns(a, 2)).toBe(false);
    t.runs.invalidate();
    const b = t.runs.start(2, 'A')!;
    expect(t.runs.finish(a, 1, 'failed')).toBeNull();
    expect(t.runs.accepts(b, 2, 'A')).toBe(true);
    t.runs.invalidate();
    expect(t.runs.finish(b, 2, 'success')).toBeNull();
  });
  it('does not add an artificial delay for fast results', () => {
    const t = setup();
    const a = t.runs.start(1, 'A')!;
    t.advance(2);
    expect(t.runs.finish(a, 1, 'success')!.endedAt).toBe(102);
    expect(t.runs.start(1, 'A')).not.toBeNull();
  });
});
