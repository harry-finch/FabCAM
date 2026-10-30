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
import { validateStages, withPocketScope, type PocketScope } from './cutting-stages';
import { captureTemplate, validTemplate } from './operation-templates';
import { assertCuttingCapability } from '../api/cutting-contract';
import type { VersionResponse } from '../api/types';
import { reportCuttingText } from './report-cutting';
function fixture() {
  const tools = defaultProjectTools();
  const op = buildOpEntry('pocket', {
    nextId: 1,
    tools,
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
  op.stages!.main = {
    tool_id: op.toolId,
    rpm: 12000,
    feed_mm_min: 500,
    plunge_mm_min: 120,
    depth_per_pass_mm: 0.5,
  };
  op.stages = {
    contract: 'pocket_finishing_v2',
    main: op.stages!.main,
    finishing: {
      scope: 'walls_and_floor',
      cutting: { ...op.stages!.main },
      lateral_allowance_mm: 0.2,
      floor_allowance_mm: 0.1,
    },
  };
  op.finishToolId = op.toolId;
  const file: ProjectFile = {
    kind: 'ivac-project',
    version: 10,
    imports: [],
    visibleLayers: [],
    selectedEntities: [],
    machine: { ...defaultProjectMachine(), gcodeDialect: 'linuxcnc' },
    tools,
    operations: [op],
  };
  const p = {
    data: {
      ...file,
      fixtures: [],
      textLayers: [],
      reliefSources: [],
      workOffset: defaultWorkOffset(),
      serializedExtensions: {},
      stock: { customX: 100, customY: 100, margin: 5, mode: 'manual', thickness: 5, visible: true },
    },
    sel: {},
    history: new History(),
  } as unknown as ProjectState;
  return { p, op };
}
it('new Pocket explicitly seeds v2 Main and finishing off', () => {
  const op = buildOpEntry('pocket', {
    nextId: 1,
    tools: defaultProjectTools(),
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
  expect(op.stages?.contract).toBe('pocket_finishing_v2');
  expect(op.stages?.finishing).toBeNull();
});
it('scope preview is pure, explicit, atomically undoable and round trips as v10', () => {
  for (const scope of ['walls_only', 'floor_only', 'walls_and_floor'] as PocketScope[]) {
    const { p, op } = fixture();
    const before = JSON.stringify(op);
    const stages = withPocketScope(op.stages!, scope);
    expect(JSON.stringify(op)).toBe(before);
    validateStages(stages);
    const cmd = updateOperationCommand(op.id, { stages });
    cmd.apply(p.data as unknown as CommandTarget);
    const saved = JSON.parse(JSON.stringify(snapshotProject(p)));
    validateProjectV10(saved);
    restoreProject(p, saved);
    expect(p.data.operations[0].stages).toEqual(stages);
    cmd.revert(p.data as unknown as CommandTarget);
    expect(JSON.stringify(p.data.operations[0])).toBe(before);
    cmd.apply(p.data as unknown as CommandTarget);
    expect(p.data.operations[0].stages).toEqual(stages);
  }
});
it('missing, unknown, contradictory scope and wrong operation refuse without loading mutations', () => {
  for (const mode of ['missing', 'unknown', 'contradiction', 'profile', 'v1_extra']) {
    const { p } = fixture();
    const before = JSON.stringify(snapshotProject(p));
    const file = JSON.parse(before);
    const op = file.operations[0];
    if (mode === 'missing') delete op.stages.finishing.scope;
    if (mode === 'unknown') op.stages.finishing.scope = 'guess';
    if (mode === 'contradiction') op.stages.finishing.scope = 'walls_only';
    if (mode === 'profile') op.kind = 'profile';
    if (mode === 'v1_extra') op.stages.contract = 'main_finishing_v1';
    expect(() => restoreProject(p, file)).toThrow();
    expect(JSON.stringify(snapshotProject(p))).toBe(before);
  }
});
it('templates retain explicit scope and reject silent reassignment to Profile', () => {
  const { p, op } = fixture();
  const template = captureTemplate(op, p.data.tools, 'Scoped pocket', 'test');
  expect(template.stages?.contract).toBe('pocket_finishing_v2');
  expect(template.stages?.finishing?.scope).toBe('walls_and_floor');
  expect(validTemplate(template)).toBe(true);
  template.settings.kind = 'profile';
  expect(validTemplate(template)).toBe(false);
});
it('requires both capabilities and never silently falls back to v1', () => {
  const project = { operations: [{ params: { stages: { contract: 'pocket_finishing_v2' } } }] };
  const v = { capabilities: ['cutting-parameters-v10'] } as VersionResponse;
  expect(() => assertCuttingCapability(v, project)).toThrow(/Pocket finishing/);
  expect(() =>
    assertCuttingCapability(
      { ...v, capabilities: [...v.capabilities, 'pocket-finishing-v2'] },
      project,
    ),
  ).not.toThrow();
  expect(() => assertCuttingCapability(v, { operations: [] })).not.toThrow();
});
it('machining report exposes the authored scope', () => {
  expect(reportCuttingText(fixture().op)).toContain('walls and floor');
});
