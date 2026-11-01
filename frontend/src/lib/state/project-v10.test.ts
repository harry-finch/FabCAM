import { expect, it, vi } from 'vitest';
vi.mock('./text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('./workspace.svelte', () => ({ workspace: {} }));
import { buildOpEntry } from './op_defaults';
import { defaultProjectMachine, defaultProjectTools } from './project-defaults';
import { defaultWorkOffset, type ProjectFile } from './project-types';
import { validateProjectV10 } from './project-v10';
import { restoreProject, snapshotProject } from './project-file-ops';
import { History } from './history';
import type { ProjectState } from './project.svelte';
import { updateOperationCommand, type CommandTarget } from './commands';
import { seedCutting } from './cutting-parameters';
function fixture(): ProjectFile {
  const tools = defaultProjectTools();
  const op = buildOpEntry('profile', {
    nextId: 1,
    tools,
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
  return {
    kind: 'ivac-project',
    version: 10,
    imports: [],
    visibleLayers: [],
    selectedEntities: [],
    machine: { ...defaultProjectMachine(), gcodeDialect: 'linuxcnc' },
    tools,
    operations: [op],
  };
}
function state(file = fixture()) {
  return {
    data: {
      ...file,
      fixtures: [],
      textLayers: [],
      reliefSources: [],
      workOffset: defaultWorkOffset(),
      serializedExtensions: {},
      stock: { customX: 100, customY: 100, margin: 5, mode: 'auto', thickness: 5, visible: true },
    },
    sel: {},
    history: new History(),
  } as unknown as ProjectState;
}
it('new operations author Main only and have no dormant Finish rates or schedule', () => {
  const file = fixture();
  const op = file.operations![0];
  expect(op.stages?.contract).toBe('main_finishing_v1');
  expect(op.stages?.finishing).toBeNull();
  expect(op.cutting).toBeUndefined();
  expect(op.step).toBeNull();
  expect(() => validateProjectV10(file)).not.toThrow();
  file.tools![0].speed = 999;
  expect(op.stages!.main.rpm).not.toBe(999);
});
it('incomplete stage values stay editable and persist exactly with identities and extensions', () => {
  const file = fixture();
  file.operations![0].stages!.main.rpm = null;
  file.operations![0].stages!.main.depth_per_pass_mm = 15.122706142614561;
  file.machine!.physicalId = '9007199254740993';
  file.tools![0].physicalId = '18446744073709551615';
  file.material = { id: '9007199254740995', revision: '1' };
  const p = state(file);
  p.data.serializedExtensions = { vendorMetadata: { coordinate: 15.122706142614561 } };
  const saved = JSON.parse(JSON.stringify(snapshotProject(p)));
  expect(saved.version).toBe(10);
  restoreProject(p, saved);
  expect(snapshotProject(p)).toEqual(saved);
  expect(p.data.operations[0].stages!.main.rpm).toBeNull();
});
it('all old and future projects refuse before document mutation or history edits', () => {
  const p = state();
  const before = JSON.stringify(snapshotProject(p));
  for (const version of [1, 2, 3, 4, 5, 6, 7, 8, 9, 11]) {
    expect(() => restoreProject(p, { ...fixture(), version } as ProjectFile)).toThrow(/v10/);
    expect(JSON.stringify(snapshotProject(p))).toBe(before);
  }
});
it('missing, mixed, wrong-tool, fractional, unknown and nonfinite stage shapes refuse', () => {
  for (const change of [
    'missing',
    'mixed',
    'tool',
    'fraction',
    'unknown',
    'overflow',
    'depth',
    'finish-tool',
    'finish-gap',
  ]) {
    const p = fixture(),
      op = p.operations![0],
      s = op.stages!;
    if (change === 'missing') delete op.stages;
    if (change === 'mixed') op.cutting = seedCutting(op, p.tools!);
    if (change === 'tool') s.main.tool_id = 999;
    if (change === 'fraction') s.main.rpm = 1.5;
    if (change === 'unknown') Object.assign(s, { rates: [] });
    if (change === 'overflow') s.main.rpm = 2 ** 32;
    if (change === 'depth') s.main.depth_per_pass_mm = Infinity;
    if (change === 'finish-tool') {
      s.finishing = { cutting: { ...s.main }, lateral_allowance_mm: 0.1, floor_allowance_mm: 0 };
    }
    if (change === 'finish-gap') Reflect.deleteProperty(s, 'finishing');
    expect(() => validateProjectV10(p), change).toThrow();
  }
});
it('one explicit toggle and tool assignment undo and redo atomically', () => {
  const p = state();
  const target = p.data as unknown as CommandTarget;
  const op = target.operations[0];
  const before = JSON.stringify(op);
  const stages = {
    ...op.stages!,
    contract: 'main_finishing_v1' as const,
    finishing: {
      cutting: { ...op.stages!.main },
      lateral_allowance_mm: 0.2,
      floor_allowance_mm: 0,
    },
  };
  const cmd = updateOperationCommand(op.id, { stages, finishToolId: op.toolId });
  cmd.apply(target);
  expect(target.operations[0].stages!.finishing!.lateral_allowance_mm).toBe(0.2);
  cmd.revert(target);
  expect(JSON.stringify(target.operations[0])).toBe(before);
  cmd.apply(target);
  expect(target.operations[0].finishToolId).toBe(op.toolId);
});
it('unchanged Drill/Spot/Chamfer roles round-trip alongside Main without remapping', () => {
  const file = fixture();
  const drill = buildOpEntry('drill', {
    nextId: 2,
    tools: file.tools!,
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
  if (drill.kind !== 'drill') throw new Error('fixture');
  drill.spotFirst = { spotToolId: drill.toolId, spotDepthMm: -0.5 };
  drill.chamferAfterWidthMm = 0.5;
  drill.finishToolId = drill.toolId;
  drill.cutting = seedCutting(drill, file.tools!);
  file.operations!.push(drill);
  const p = state(file),
    saved = JSON.parse(JSON.stringify(snapshotProject(p)));
  restoreProject(p, saved);
  expect(snapshotProject(p)).toEqual(saved);
  expect(p.data.operations[1].cutting!.rates.map((r) => r.role)).toEqual([
    'drill',
    'spot',
    'chamfer',
  ]);
});
it('geometry guards and retired metadata guards are retained', () => {
  const file = fixture();
  for (const field of ['fsContext'])
    expect(() => validateProjectV10({ ...file, [field]: null })).toThrow('Legacy');
  for (const field of ['appliedCutting', 'fsFullWidth', 'appliedContextKey']) {
    const p = fixture();
    Object.assign(p.operations![0], { [field]: null });
    expect(() => validateProjectV10(p)).toThrow('Legacy');
  }
  file.operations![0].geometryBinding = { kind: 'unresolved', previous: [], reason: 'missing' };
  expect(() => validateProjectV10(file)).toThrow();
});

it('INT-ALPHA-1 optional metadata round-trips without changing identity, stages or legacy absence', () => {
  for (const [substrate, coating] of [
    [undefined, undefined],
    ['HSS-Co custom', 'uncoated'],
    ['', 'DLC custom'],
  ] as const) {
    const file = fixture();
    Object.assign(file.tools![0], { substrate, coating });
    const original = structuredClone(file);
    const p = state(file);
    const saved = JSON.parse(JSON.stringify(snapshotProject(p)));
    restoreProject(p, saved);
    expect(p.data.tools).toEqual(original.tools);
    expect(p.data.operations).toEqual(original.operations);
    for (const field of ['substrate', 'coating']) {
      const bad = structuredClone(saved);
      bad.tools[0][field] = 42;
      const before = JSON.stringify(snapshotProject(p));
      expect(() => restoreProject(p, bad)).toThrow(/expected text/);
      expect(JSON.stringify(snapshotProject(p))).toBe(before);
    }
  }
});
