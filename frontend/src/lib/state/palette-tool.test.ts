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
it('retains legacy assignment behavior without rewriting data', () => {
  const op = profile();
  const before = JSON.stringify(op);
  expect(assignmentProblem(op, tool, ['mill'])).toBeUndefined();
  expect(JSON.stringify(op)).toBe(before);
});
it('refuses new tool IDs absent from saved cutting roles', () => {
  const op = profile();
  op.appliedCutting = { slots: [], context_key: 'manual', doc_origin: 'manual' };
  expect(assignmentProblem(op, tool, ['mill'])).toBe('palette.review_rates');
  expect(op.appliedCutting.slots).toEqual([]);
});

it('allows only covered saved roles and leaves their values untouched', () => {
  const op = profile();
  const slot = {
    tool_id: 7,
    pass: 'rough' as const,
    rpm: 11000,
    feed_mm_min: 900,
    plunge_mm_min: 450,
    origins: { rpm: 'manual' as const, feed: 'manual' as const, plunge: 'manual' as const },
  };
  op.appliedCutting = { slots: [slot], context_key: 'frozen', doc_origin: 'manual' };
  const before = JSON.stringify(op);
  expect(assignmentProblem(op, tool, ['mill'])).toBeUndefined();
  expect(JSON.stringify(op)).toBe(before);
  const pocket = buildOpEntry('pocket', {
    nextId: 2,
    tools: [tool],
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
  pocket.appliedCutting = op.appliedCutting;
  if (pocket.kind === 'pocket') pocket.finishToolId = tool.id;
  expect(assignmentProblem(pocket, tool, ['mill'])).toBe('palette.review_rates');
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
