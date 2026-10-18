import { expect, it } from 'vitest';
import { GenerationRuns, generationExportIsCurrent } from './generation-run';

// Bounded two-request schedules: edit, undo/redo, cancellation, late completion.
for (let index = 0; index < 32; index++) {
  it(`TEST1 C2 schedule ${index}`, () => {
    const runs = new GenerationRuns(() => 100);
    const a = runs.start(1, 'A')!;
    expect(runs.start(1, 'A')).toBeNull();
    let context = 'B';
    expect(runs.accepts(a, 1, context)).toBe(false);
    if (index & 1) context = 'A';
    expect(runs.accepts(a, 1, context)).toBe(!!(index & 1));
    if (index & 2) context = 'B';
    expect(runs.accepts(a, 1, context)).toBe(context === 'A');
    runs.invalidate();
    const b = runs.start(2, context)!;
    expect(runs.finish(a, 1, 'success')).toBeNull();
    expect(runs.accepts(b, 2, context)).toBe(true);
    const outcome = index & 4 ? 'cancelled' : index & 8 ? 'failed' : 'success';
    expect(runs.finish(b, 2, outcome)?.outcome).toBe(outcome);
    expect(runs.finish(b, 2, 'success')).toBeNull();
    const captured = {};
    const current = outcome === 'success' ? captured : {};
    const blocked = !!(index & 16);
    expect(generationExportIsCurrent(blocked, context, context, true, current, captured)).toBe(
      outcome === 'success' && !blocked,
    );
    expect(
      generationExportIsCurrent(false, context, context + '-edit', true, current, captured),
    ).toBe(false);
    expect(generationExportIsCurrent(false, context, context, false, current, captured)).toBe(
      false,
    );
    expect(generationExportIsCurrent(false, context, context, true, {}, captured)).toBe(false);
  });
}
