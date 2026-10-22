import { expect, it, vi } from 'vitest';
vi.mock('../api/wasm', () => ({ presetCommand: vi.fn() }));
import { presetCommand } from '../api/wasm';
import {
  cuttingTargetKey,
  reviewedCuttingCommand,
  captureTestedPreset,
  preparePresetLoad,
} from './cutting-workflow';
import { History } from './history';
import { buildOpEntry } from './op_defaults';
import {
  defaultProjectTools,
  defaultProjectMachine,
  defaultProjectStock,
} from './project-defaults';
import { updateOperationCommand, type CommandTarget } from './commands';
import type { ProjectState } from './project.svelte';
function fixture() {
  const tools = defaultProjectTools();
  return {
    operations: [
      buildOpEntry('pocket', {
        nextId: 1,
        tools,
        reliefSources: [],
        selectionIds: [],
        objectMeta: [],
      }),
    ],
    machine: defaultProjectMachine(),
    tools,
    material: { id: 'plywood', revision: '1' },
    stock: defaultProjectStock(),
    imports: [],
    fixtures: [],
    textLayers: [],
    reliefSources: [],
    dirty: false,
  } as unknown as CommandTarget;
}
it('reviewed copy is one Undo/Redo step and leaves Stock, machine and material unchanged', () => {
  const s = fixture(),
    h = new History();
  const before = structuredClone(s);
  const cutting = structuredClone(s.operations[0].cutting!);
  cutting.rates[0].feed_mm_min = 1234;
  h.exec(reviewedCuttingCommand(1, cuttingTargetKey(s, 1), { cutting, step: -2 }), s);
  expect(h.undoSize).toBe(1);
  expect(s.stock).toEqual(before.stock);
  expect(s.material).toEqual(before.material);
  expect(s.machine).toEqual(before.machine);
  expect(s.tools).toEqual(before.tools);
  h.undo(s);
  expect(s.operations).toEqual(before.operations);
  h.redo(s);
  expect(s.operations[0].cutting?.rates[0].feed_mm_min).toBe(1234);
});
it('rejects a stale preview before any mutation', () => {
  const s = fixture();
  const key = cuttingTargetKey(s, 1);
  s.machine.fastMoveZ = 12;
  const before = JSON.stringify(s);
  expect(() => reviewedCuttingCommand(1, key, { step: -2 }).apply(s)).toThrow('changed');
  expect(JSON.stringify(s)).toBe(before);
});
it('new secondary roles are incomplete and changing tools retains rates', () => {
  const s = fixture();
  const op = buildOpEntry('drill', {
    nextId: 2,
    tools: s.tools,
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
  s.operations = [op];
  const before = op.cutting!.rates[0].feed_mm_min;
  updateOperationCommand(2, { spotFirst: { spotToolId: 8, spotDepthMm: -0.5 } }).apply(s);
  expect(s.operations[0].cutting?.rates.find((r) => r.role === 'spot')?.rpm).toBeNull();
  updateOperationCommand(2, { toolId: 7 }).apply(s);
  expect(s.operations[0].cutting?.rates[0]).toMatchObject({ tool_id: 7, feed_mm_min: before });
});
it('software inspection cannot substitute for explicit tested attestation', async () => {
  const s = fixture();
  const p = { data: s } as unknown as ProjectState;
  vi.mocked(presetCommand).mockClear();
  await expect(captureTestedPreset(p, 1, 'Preset', '')).rejects.toThrow('Confirm testing');
  expect(presetCommand).not.toHaveBeenCalled();
});
it('an async context change rejects loading even when core returns a successful patch', async () => {
  const s = fixture();
  const p = { data: s } as unknown as ProjectState;
  vi.mocked(presetCommand).mockImplementationOnce(async () => {
    s.operations[0].depth = -9;
    return {};
  });
  await expect(
    preparePresetLoad(
      p,
      1,
      { format: 'fabcam-preset-collection', version: 1, id: 'lab', name: 'Lab', presets: [] },
      'test',
    ),
  ).rejects.toThrow('changed');
});
