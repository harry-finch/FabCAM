import { it, expect } from 'vitest';
import { assignmentProblem } from './palette-tool';
import { buildOpEntry } from './op_defaults';
import type { ToolEntry } from './project-types';
const tool: ToolEntry = {
  id: 7,
  name: 'Endmill',
  kind: 'endmill',
  diameter: 6,
  flutes: 2,
  speed: 10000,
  feedRate: 1000,
  plungeRate: 500,
  coolant: 'off',
};
const profile = () =>
  buildOpEntry('profile', {
    nextId: 1,
    tools: [tool],
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
it('requires a real stocked tool and actual cutting operation', () => {
  expect(assignmentProblem(profile(), undefined, ['mill'])).toBe('palette.stock_tool');
  expect(assignmentProblem(undefined, tool, ['mill'])).toBe('palette.select_operation');
  const pause = buildOpEntry('pause', {
    nextId: 1,
    tools: [],
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
  expect(assignmentProblem(pause, tool, ['mill'])).toBe('palette.select_operation');
});
it('checks actual kind, capabilities and finite diameter', () => {
  expect(assignmentProblem(profile(), { ...tool, kind: 'v_bit' }, ['mill'])).toBe(
    'palette.incompatible',
  );
  expect(assignmentProblem(profile(), tool, ['laser'])).toBe('palette.incompatible');
  expect(assignmentProblem(profile(), { ...tool, diameter: NaN }, ['mill'])).toBe(
    'palette.incompatible',
  );
});
it('checks assignment without rewriting authored values', () => {
  const op = profile();
  const before = JSON.stringify(op);
  expect(assignmentProblem(op, tool, ['mill'])).toBeUndefined();
  expect(JSON.stringify(op)).toBe(before);
});
it('accepts assignment with incomplete v9 values without mutating them', () => {
  const op = profile();
  op.cutting!.rates.forEach((r) => {
    r.rpm = null;
    r.feed_mm_min = null;
  });
  const before = JSON.stringify(op);
  expect(assignmentProblem(op, { ...tool, id: 99 }, ['mill'])).toBeUndefined();
  expect(JSON.stringify(op)).toBe(before);
});
it('refuses whirling tools for planar surfacing', () => {
  const op = buildOpEntry('planar_surface', {
    nextId: 1,
    tools: [tool],
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
    surfaceZone: { minX: 0, minY: 0, maxX: 50, maxY: 50 },
  });
  expect(assignmentProblem(op, { ...tool, whirl: true }, ['mill'])).toBe('palette.incompatible');
});

it('permits inventory assignment through the value-preserving selection command', () => {
  const op = profile();
  expect(assignmentProblem(op, { ...tool, id: 99 }, ['mill'])).toBeUndefined();
  expect(assignmentProblem(op, { ...tool, kind: 'drag_knife' }, ['mill'])).toBe(
    'palette.incompatible',
  );
});
