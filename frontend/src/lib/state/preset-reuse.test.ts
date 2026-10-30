import { expect, it, vi } from 'vitest';
vi.mock('../api/wasm', () => ({ presetCommand: vi.fn() }));
import { buildOpEntry } from './op_defaults';
import {
  defaultProjectMachine,
  defaultProjectStock,
  defaultProjectTools,
} from './project-defaults';
import { History } from './history';
import { cuttingTargetKey } from './cutting-workflow';
import type { ProjectState } from './project.svelte';
import type { PresetCollection } from './preset-collections';
import {
  preparePresetReuse,
  confirmPresetReuse,
  reuseRecords,
  reuseState,
  REUSE_KEY,
} from './preset-reuse';
function setup(kind: 'profile' | 'pocket' = 'pocket') {
  const tools = defaultProjectTools();
  const op = buildOpEntry(kind, {
    nextId: 1,
    tools,
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
  op.stages!.main = {
    tool_id: op.toolId,
    rpm: 10000,
    feed_mm_min: 800,
    plunge_mm_min: 100,
    depth_per_pass_mm: 1,
  };
  const data = {
    operations: [op],
    tools,
    machine: defaultProjectMachine(),
    stock: defaultProjectStock(),
    material: { id: 'target', revision: '1' },
    workOffset: { wcs: 'G54', x_mm: 0, y_mm: 0, z_mm: 0 },
    imports: [],
    fixtures: [],
    textLayers: [],
    reliefSources: [],
    groupOpsByTool: false,
    serializedExtensions: { custom: { kept: true } },
    dirty: false,
  };
  const p = {
    data,
    history: new History(),
    documentEpoch: 1,
    target: () => data,
  } as unknown as ProjectState;
  const stages = JSON.parse(JSON.stringify(op.stages));
  stages.contract = kind === 'profile' ? 'pocket_finishing_v2' : 'main_finishing_v1';
  stages.main = {
    tool_id: 99,
    rpm: 12000,
    feed_mm_min: 1500,
    plunge_mm_min: 250,
    depth_per_pass_mm: 2.33,
  };
  const source: PresetCollection = {
    format: 'fabcam-preset-collection',
    version: 2,
    id: 'lab',
    name: 'Source lab',
    presets: [
      {
        id: 'p',
        name: 'Source preset',
        tested: true,
        test_date: '2026-01-01',
        notes: 'PRIVATE DO NOT COPY',
        context: {
          machine_id: 'other-machine',
          machine_label: 'Other machine',
          material_id: 'source',
          material_label: 'Source material',
          signature: JSON.stringify({
            operation: { type: kind === 'profile' ? 'pocket' : 'profile', strategy: 'cascade' },
            material: { id: 'source', revision: '1' },
            tools: [
              {
                role: 'main',
                physical_id: 'other-tool',
                geometry: { kind: 'endmill', diameter: 6 },
              },
            ],
          }),
        },
        parameters: { stages, rates: [], pocket_overlap: null, planar_stepover_mm: null },
      },
    ],
  };
  const row = { data: source, token: 'revision-1' };
  const read = async () => [{ json: JSON.stringify(source), token: row.token }];
  return { p, op, source, row, read };
}
for (const kind of ['profile', 'pocket'] as const)
  it(`cross-operation to ${kind} Main copies only independently selected fields`, async () => {
    for (const field of ['rpm', 'feed_mm_min', 'plunge_mm_min', 'depth_per_pass_mm'] as const) {
      const { p, op, row, read } = setup(kind);
      const before = JSON.parse(JSON.stringify(p.data));
      const review = await preparePresetReuse(p, 1, row, 'p', 'main', 'main', [field]);
      expect(review.record.source.context.tool_kind).toBe('endmill');
      expect(review.record.source.context.diameter_mm).toBe(6);
      expect(p.data).toEqual(before); // review/cancel is non-mutating
      await confirmPresetReuse(p, review, read);
      const next = p.data.operations[0];
      expect(next.stages!.main[field]).toBe(row.data.presets[0].parameters.stages!.main[field]);
      for (const k of ['rpm', 'feed_mm_min', 'plunge_mm_min', 'depth_per_pass_mm'] as const)
        if (k !== field) expect(next.stages!.main[k]).toBe(op.stages!.main[k]);
      expect({ ...next, stages: op.stages, stagePreset: undefined }).toEqual({
        ...op,
        stagePreset: undefined,
      });
      expect(p.data.stock).toEqual(before.stock);
      expect(p.data.tools).toEqual(before.tools);
      expect(p.data.material).toEqual(before.material);
      expect(p.data.serializedExtensions.custom).toEqual({ kept: true });
      expect(JSON.stringify(p.data.serializedExtensions)).not.toContain('PRIVATE');
      expect(reuseRecords(p.data.serializedExtensions)).toHaveLength(1);
    }
  });
it('requires enabled destination finishing; source finishing is selected explicitly', async () => {
  const { p, op, row, read } = setup();
  await expect(preparePresetReuse(p, 1, row, 'p', 'main', 'finishing', ['rpm'])).rejects.toThrow();
  op.stages!.finishing = {
    cutting: { ...op.stages!.main },
    lateral_allowance_mm: 0.2,
    floor_allowance_mm: 0,
    scope: 'walls_only',
  };
  op.finishToolId = op.toolId;
  const review = await preparePresetReuse(p, 1, row, 'p', 'main', 'finishing', ['rpm']);
  await confirmPresetReuse(p, review, read);
  expect(p.data.operations[0].stages!.main.rpm).toBe(10000);
  expect(p.data.operations[0].stages!.finishing!.cutting.rpm).toBe(12000);
  expect(p.data.operations[0].stages!.finishing!.lateral_allowance_mm).toBe(0.2);
  await expect(preparePresetReuse(p, 1, row, 'p', 'finishing', 'main', ['rpm'])).rejects.toThrow();
});
it('refuses V1 reinterpretation, unsupported fields, missing values and fractional RPM', async () => {
  const { p, row } = setup();
  for (const fields of [[], ['pocket_overlap'], ['tool_id'], ['rpm', 'rpm']])
    await expect(
      preparePresetReuse(p, 1, row, 'p', 'main', 'main', fields as never),
    ).rejects.toThrow();
  row.data.version = 1;
  await expect(preparePresetReuse(p, 1, row, 'p', 'main', 'main', ['rpm'])).rejects.toThrow();
  row.data.version = 2;
  for (const rpm of [null, 1.5, 0, -1, Infinity, 0x100000000]) {
    row.data.presets[0].parameters.stages!.main.rpm = rpm;
    await expect(preparePresetReuse(p, 1, row, 'p', 'main', 'main', ['rpm'])).rejects.toThrow();
  }
});
it('refuses stale target, document, source revision, source content and deletion without mutation', async () => {
  for (const change of ['target', 'document', 'token', 'content', 'delete']) {
    const { p, row, read } = setup();
    const r = await preparePresetReuse(p, 1, row, 'p', 'main', 'main', ['rpm']);
    if (change === 'target') p.data.operations[0].depth = -9;
    if (change === 'document') p.documentEpoch++;
    if (change === 'token') row.token = 'new';
    if (change === 'content') row.data.presets[0].parameters.stages!.main.rpm = 13000;
    const before = JSON.stringify(p.data);
    await expect(
      confirmPresetReuse(p, r, change === 'delete' ? async () => [] : read),
    ).rejects.toThrow();
    expect(JSON.stringify(p.data)).toBe(before);
    expect(p.history.undoSize).toBe(0);
  }
});
it('one atomic Undo/Redo restores parameters, prior attestation and extension; later source is irrelevant', async () => {
  const { p, row, read } = setup();
  const old = { prior: 'snapshot' } as never;
  p.data.operations[0].stagePreset = old;
  const before = JSON.parse(JSON.stringify(p.data));
  const key = cuttingTargetKey(p.data, 1);
  await confirmPresetReuse(
    p,
    await preparePresetReuse(p, 1, row, 'p', 'main', 'main', ['rpm']),
    read,
  );
  expect(p.history.undoSize).toBe(1);
  expect(p.data.operations[0].stagePreset).toBeUndefined();
  expect(cuttingTargetKey(p.data, 1)).not.toBe(key);
  const after = JSON.parse(JSON.stringify(p.data));
  p.history.undo(p.target());
  expect(p.data.operations).toEqual(before.operations);
  expect(p.data.serializedExtensions).toEqual(before.serializedExtensions);
  row.data.presets = [];
  p.history.redo(p.target());
  expect(p.data).toEqual(after);
  expect(
    reuseState(p.data.operations[0], reuseRecords(p.data.serializedExtensions)[0], p.data),
  ).toBe('copied');
  p.data.operations[0].stages!.main.rpm = 14000;
  expect(
    reuseState(p.data.operations[0], reuseRecords(p.data.serializedExtensions)[0], p.data),
  ).toBe('modified');
  const saved = JSON.parse(JSON.stringify(p.data));
  expect(reuseRecords(saved.serializedExtensions)).toHaveLength(1);
});
it('unknown advisory versions and malformed/orphaned records are never trusted or overwritten', async () => {
  const { p, row } = setup();
  p.data.serializedExtensions[REUSE_KEY] = { version: 99, private: 'opaque' };
  expect(reuseRecords(p.data.serializedExtensions)).toEqual([]);
  await expect(preparePresetReuse(p, 1, row, 'p', 'main', 'main', ['rpm'])).rejects.toThrow();
});

import { deleteOperationCommand } from './commands';
it('deletion prunes known advisory records atomically and duplication/kind changes cannot borrow their association', async () => {
  const { p, row, read } = setup();
  await confirmPresetReuse(
    p,
    await preparePresetReuse(p, 1, row, 'p', 'main', 'main', ['rpm']),
    read,
  );
  const record = reuseRecords(p.data.serializedExtensions)[0];
  const copy = { ...p.data.operations[0], id: 2 };
  expect(reuseState(copy, record, p.data)).toBe('unassociated');
  expect(reuseState({ ...p.data.operations[0], kind: 'profile' } as never, record, p.data)).toBe(
    'unassociated',
  );
  p.history.exec(deleteOperationCommand(1), p.target());
  expect(reuseRecords(p.data.serializedExtensions)).toHaveLength(0);
  p.history.undo(p.target());
  expect(reuseRecords(p.data.serializedExtensions)).toEqual([record]);
  p.data.operations[0].stagePreset = { prior: 'matching' } as never;
  expect(reuseState(p.data.operations[0], record, p.data)).toBe('superseded');
});
it('source Finishing can supply Main without changing the target tool or scope', async () => {
  const { p, row, read } = setup('profile');
  const st = row.data.presets[0].parameters.stages!;
  st.finishing = {
    cutting: { ...st.main, rpm: 14000 },
    lateral_allowance_mm: 0.1,
    floor_allowance_mm: 0,
    scope: 'walls_only',
  };
  await confirmPresetReuse(
    p,
    await preparePresetReuse(p, 1, row, 'p', 'finishing', 'main', ['rpm']),
    read,
  );
  expect(p.data.operations[0].stages!.main.rpm).toBe(14000);
  expect(p.data.operations[0].stages!.finishing).toBeNull();
});
