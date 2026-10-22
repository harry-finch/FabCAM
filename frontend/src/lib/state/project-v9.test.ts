import { expect, it, vi } from 'vitest';
vi.mock('./text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('./workspace.svelte', () => ({ workspace: {} }));
import { snapshotProject, restoreProject } from './project-file-ops';
import { History } from './history';
import type { ProjectState } from './project.svelte';
import { defaultWorkOffset } from './project-types';
import { validateProjectV9 } from './project-v9';
import { defaultProjectMachine, defaultProjectTools } from './project-defaults';
import { buildOpEntry } from './op_defaults';
import { seedCutting } from './cutting-parameters';
import type { ProjectFile } from './project-types';
function fixture(): ProjectFile {
  const tools = defaultProjectTools();
  const op = buildOpEntry('profile', {
    nextId: 1,
    tools,
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
  op.cutting = seedCutting(op, tools);
  return {
    kind: 'ivac-project',
    version: 9,
    imports: [],
    visibleLayers: [],
    selectedEntities: [],
    machine: defaultProjectMachine(),
    tools,
    operations: [op],
  };
}
it('v9 persists incomplete explicit values without source packs', () => {
  const p = fixture();
  p.operations![0].cutting!.rates[0].rpm = null;
  expect(() => validateProjectV9(JSON.parse(JSON.stringify(p)))).not.toThrow();
});
it('rejects old and future versions before mutation', () => {
  for (const version of [1, 2, 3, 4, 5, 6, 7, 8, 10]) {
    const p = fixture();
    const before = JSON.stringify(p);
    expect(() => validateProjectV9({ ...p, version } as ProjectFile)).toThrow();
    expect(JSON.stringify(p)).toBe(before);
  }
});
it('rejects missing, duplicate, mismatched and invalid role values', () => {
  for (const change of ['missing', 'duplicate', 'tool', 'fraction', 'overflow', 'unknown']) {
    const p = fixture();
    const c = p.operations![0].cutting!;
    if (change === 'missing') c.rates.pop();
    if (change === 'duplicate') c.rates.push({ ...c.rates[0] });
    if (change === 'tool') c.rates[0].tool_id = 999;
    if (change === 'fraction') c.rates[0].rpm = 1.5;
    if (change === 'overflow') c.rates[0].rpm = 2 ** 32;
    if (change === 'unknown') Object.assign(c, { surprise: 42 });
    expect(() => validateProjectV9(p)).toThrow();
  }
});
it('retains geometry guards and unrelated project data', () => {
  const p = fixture();
  p.workOffset = { wcs: 'G55', x_mm: 1, y_mm: 2, z_mm: 3 };
  p.groupOpsByTool = true;
  const restored = JSON.parse(JSON.stringify(p));
  validateProjectV9(restored);
  expect(restored).toEqual(p);
  Object.assign(p.operations![0], {
    geometryBinding: { kind: 'unresolved', previous: [], reason: 'missing' },
  });
  expect(() => validateProjectV9(p)).toThrow();
});
it('seed is a one-time copy with distinct finish values', () => {
  const p = fixture();
  p.tools![0].feedRateFinish = 400;
  const op = p.operations![0];
  op.cutting = seedCutting(op, p.tools!);
  p.tools![0].feedRateFinish = 999;
  expect(op.cutting.rates.find((r) => r.role === 'finish')?.feed_mm_min).toBe(400);
});

it('the real snapshot/restore path preserves incomplete values and rejects v8 without mutation', () => {
  const file = fixture();
  file.operations![0].cutting!.rates[0].rpm = null;
  file.machine!.physicalId = 'lab-router';
  file.tools![0].physicalId = 'lab-bit';
  file.tools![0].maxRpm = 24000;
  const p = {
    data: {
      ...file,
      fixtures: [],
      textLayers: [],
      reliefSources: [],
      workOffset: defaultWorkOffset(),
      serializedExtensions: {},
    },
    sel: {},
    history: new History(),
  } as unknown as ProjectState;
  const saved = JSON.parse(JSON.stringify(snapshotProject(p)));
  expect(saved.version).toBe(9);
  expect('fsContext' in saved).toBe(false);
  restoreProject(p, saved);
  expect(snapshotProject(p).operations).toEqual(saved.operations);
  expect(p.data.tools[0].maxRpm).toBe(24000);
  expect(p.data.machine.physicalId).toBe('lab-router');
  const before = JSON.stringify(snapshotProject(p));
  expect(() => restoreProject(p, { ...saved, version: 8 })).toThrow();
  expect(JSON.stringify(snapshotProject(p))).toBe(before);
});

it('refuses obsolete context/recipe fields even when null rather than silently loading them', () => {
  for (const field of ['fsContext']) {
    const p = { ...fixture(), [field]: null };
    expect(() => validateProjectV9(p)).toThrow('Legacy');
  }
  for (const field of ['appliedCutting', 'fsFullWidth', 'appliedContextKey']) {
    const p = fixture();
    Object.assign(p.operations![0], { [field]: null });
    expect(() => validateProjectV9(p)).toThrow('Legacy');
  }
});
