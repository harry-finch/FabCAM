import { expect, it } from 'vitest';
import { notificationError } from '../api/error-notification';
import { failGenerate, setGenerated } from './project-generate-ops';
import { inDrawingFrame } from './work-frame';
import type { ProjectState } from './project.svelte';
import type { WiacError } from '../api/types';

const refusal: WiacError = {
  kind: 'misconfigured',
  message: 'Program blocked',
  diagnostic_preview: {
    toolpath: [
      {
        from: { x: 0, y: 0, z: 0 },
        to: { x: 1, y: 0, z: 0 },
        kind: 'rapid',
        op_id: 1,
        gcode_line: 1,
      },
    ],
    warnings: [
      { kind: 'fixture_path_collision', message: 'clamp', op_id: 1, params: { fixture_id: '7' } },
    ],
  },
};
it.each([refusal, JSON.stringify(refusal), new Error(JSON.stringify(refusal))])(
  'transport normalization retains inspection-only preview',
  (input) => {
    expect(notificationError(input)).toEqual(refusal);
  },
);
it('failed generation replaces old executable sides, preserves unsaved edits, and keeps preview', () => {
  const p = {
    gen: { generated: { gcode: 'old' }, generatedBack: { gcode: 'old-back' }, generatedVersion: 1 },
    data: { dirty: true },
    error: null,
    playhead: 0,
    setError(this: { error: unknown }, error: unknown) {
      this.error = error as null;
    },
  } as unknown as ProjectState;
  failGenerate(p, refusal);
  expect(p.gen.generated?.gcode).toBe('');
  expect(p.gen.generatedBack).toBeNull();
  expect(p.gen.generated?.toolpath).toEqual(refusal.diagnostic_preview!.toolpath);
  expect(p.data.dirty).toBe(true);
  expect(p.gen.pipelineState).toBe('idle');
  setGenerated(p, { ...p.gen.generated!, gcode: 'new valid program', warnings: [] });
  expect(p.error).toBeNull();
  expect(p.gen.generated?.warnings).toEqual([]);
});
it('diagnostic geometry uses the same drawing-frame conversion', () => {
  const result = inDrawingFrame(refusal.diagnostic_preview!, {
    x_mm: 7,
    y_mm: -3,
    z_mm: 0,
    wcs: 'G54',
  });
  expect(result.toolpath[0].to).toEqual({ x: 8, y: -3, z: 0 });
  expect(refusal.diagnostic_preview!.toolpath[0].to.x).toBe(1);
});
