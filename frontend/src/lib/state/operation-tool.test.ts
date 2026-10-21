import { expect, it } from 'vitest';
import { selectOperationToolCommand, toolFitsOperation } from './operation-tool';
import { buildOpEntry } from './op_defaults';
import { operationRates } from './feeds-speeds';
import { History } from './history';
import type { CommandTarget } from './commands';
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
function state(kind: 'profile' | 'pocket' | 'drill' = 'profile') {
  return {
    tools: [structuredClone(tool)],
    operations: [
      buildOpEntry(kind, {
        nextId: 1,
        tools: [tool],
        reliefSources: [],
        selectionIds: [],
        objectMeta: [],
      }),
    ],
    machine: { mode: 'mill' },
    dirty: false,
  } as CommandTarget;
}
it('adds, assigns and restores both entities in one undo/redo step', () => {
  const s = state(),
    history = new History(),
    next = { ...tool, id: 9, name: 'New cutter' };
  const before = structuredClone(s);
  history.exec(selectOperationToolCommand(1, 'toolId', next, 'inventory'), s);
  expect(s.tools).toHaveLength(2);
  expect(s.operations[0].toolId).toBe(9);
  expect(history.undoSize).toBe(1);
  history.undo(s);
  expect(s.tools).toEqual(before.tools);
  expect(s.operations).toEqual(before.operations);
  history.redo(s);
  expect(s.tools).toHaveLength(2);
  expect(s.operations[0].toolId).toBe(9);
});
it('remaps an id collision without overwriting another tool and reuses the copy', () => {
  const s = state(),
    next = { ...tool, name: 'Different library cutter', diameter: 3 };
  selectOperationToolCommand(1, 'toolId', next, 'inventory').apply(s);
  expect(s.tools[0]).toEqual(tool);
  expect(s.operations[0].toolId).toBe(8);
  selectOperationToolCommand(1, 'toolId', next, 'inventory').apply(s);
  expect(s.tools).toHaveLength(2);
});
it('filters by both operation and machine and rejects invalid selection before stocking', () => {
  const s = state();
  expect(toolFitsOperation(s.operations[0], tool, ['drag'])).toBe(false);
  expect(toolFitsOperation(s.operations[0], { ...tool, kind: 'v_bit' }, ['mill'])).toBe(false);
  expect(() =>
    selectOperationToolCommand(1, 'toolId', { ...tool, kind: 'laser_beam' }, 'inventory').apply(s),
  ).toThrow('incompatible');
  expect(s.tools).toHaveLength(1);
});
it('keeps saved manual cutting values, the captured context and the separate finishing tool', () => {
  const s = state('pocket'),
    op = s.operations[0];
  if (op.kind !== 'pocket') throw new Error('fixture');
  op.finishToolId = 11;
  op.appliedCutting = {
    context_key: 'original-context',
    doc_origin: 'manual',
    slots: [7, 11].map((id) => ({
      tool_id: id,
      pass: id === 7 ? 'rough' : 'finish',
      rpm: 9000,
      feed_mm_min: 321,
      plunge_mm_min: 123,
      origins: { rpm: 'manual', feed: 'manual', plunge: 'manual' },
    })),
  };
  const next = { ...tool, id: 9 };
  selectOperationToolCommand(1, 'toolId', next, 'inventory').apply(s);
  expect(operationRates(s.operations[0], next, 'rough')?.feed_mm_min).toBe(321);
  expect(s.operations[0].appliedCutting?.context_key).toBe('original-context');
  expect('finishToolId' in s.operations[0] && s.operations[0].finishToolId).toBe(11);
  expect(s.operations[0].appliedCutting?.slots.find((s) => s.tool_id === 11)?.pass).toBe('finish');
});
it('stocks finishing and spot tools through the same command', () => {
  const s = state('drill'),
    op = s.operations[0];
  if (op.kind !== 'drill') throw new Error('fixture');
  op.spotFirst = { spotToolId: 7, spotDepthMm: -1 };
  selectOperationToolCommand(1, 'spotToolId', { ...tool, id: 8, kind: 'v_bit' }, 'inventory').apply(
    s,
  );
  expect(s.operations[0].kind === 'drill' && s.operations[0].spotFirst?.spotToolId).toBe(8);
  selectOperationToolCommand(
    1,
    'finishToolId',
    { ...tool, id: 9, kind: 'drill' },
    'inventory',
  ).apply(s);
  expect(s.tools).toHaveLength(3);
});

it('switches a finishing tool back to the main tool while retaining its last edited rates', () => {
  const s = state('pocket'),
    op = s.operations[0];
  if (op.kind !== 'pocket') throw new Error('fixture');
  op.finishToolId = 8;
  op.appliedCutting = {
    context_key: 'frozen',
    doc_origin: 'manual',
    slots: [
      {
        tool_id: 8,
        pass: 'finish',
        rpm: 9000,
        feed_mm_min: 321,
        plunge_mm_min: 123,
        origins: { rpm: 'manual', feed: 'manual', plunge: 'manual' },
      },
    ],
  };
  selectOperationToolCommand(1, 'finishToolId', tool, 'project').apply(s);
  expect('finishToolId' in s.operations[0] && s.operations[0].finishToolId).toBeUndefined();
  expect(operationRates(s.operations[0], tool, 'finish')?.feed_mm_min).toBe(321);
  expect(s.operations[0].appliedCutting?.slots).toHaveLength(1);
});
