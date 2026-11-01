import { expect, it, vi } from 'vitest';
vi.mock('./text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('./workspace.svelte', () => ({ workspace: {} }));
vi.mock('../api/wasm', () => ({ presetCommand: vi.fn() }));
import { readFileSync } from 'node:fs';
import type { ProjectState } from './project.svelte';
import type { ProjectFile } from './project-types';
import { defaultWorkOffset } from './project-types';
import { restoreProject, snapshotProject } from './project-file-ops';
import { generationContextKey, buildGenerationProject } from './generation-context';
import { combineImports } from './file-transform';
import { History } from './history';
import { preparePresetReuse, confirmPresetReuse, reuseRecords, reuseState } from './preset-reuse';
import type { PresetCollection } from './preset-collections';
it('actual V10 portable save/restore retains exact advisory values, unknown extensions and authored generation freshness', async () => {
  const file = JSON.parse(
    readFileSync(
      new URL(
        '../../../../tests/fixtures/alpha32/browser-pocket.ivac-project.json',
        import.meta.url,
      ),
      'utf8',
    ),
  ) as ProjectFile;
  file.machine!.physicalId = 'target-machine';
  file.tools!.forEach((t) => (t.physicalId = `target-tool-${t.id}`));
  file.tools!.forEach((t) => {
    t.substrate = 'HSS custom';
    t.coating = 'uncoated';
  });
  const originalTools = structuredClone(file.tools);
  const data = {
    ...file,
    workOffset: defaultWorkOffset(),
    serializedExtensions: { vendor: { exact: 15.122706142614561 } },
    reliefSources: [],
    groupOpsByTool: false,
  };
  const p = {
    data,
    sel: {},
    gen: {},
    history: new History(),
    documentEpoch: 1,
    target: () => data,
    get transformedImport() {
      return combineImports(data.imports);
    },
    get geometryView() {
      return combineImports(data.imports);
    },
    get stockSizingImport() {
      return combineImports(data.imports);
    },
  } as unknown as ProjectState;
  const stages = structuredClone(p.data.operations[0].stages!);
  stages.main.depth_per_pass_mm = 1.122706142614561;
  const c: PresetCollection = {
    format: 'fabcam-preset-collection',
    version: 2,
    id: 'lab',
    name: 'Lab',
    presets: [
      {
        id: 'source',
        name: 'Original',
        tested: true,
        test_date: null,
        notes: 'private trial',
        context: {
          machine_id: 'source-machine',
          machine_label: 'Source',
          material_id: 'source-material',
          material_label: 'Source',
          signature: JSON.stringify({
            operation: { type: 'pocket' },
            tools: [],
            material: { revision: '1' },
          }),
        },
        parameters: { stages, rates: [], pocket_overlap: null, planar_stepover_mm: null },
      },
    ],
  };
  const key = generationContextKey(p, 'grbl');
  await confirmPresetReuse(
    p,
    await preparePresetReuse(p, 1, { data: c, token: 'v1' }, 'source', 'main', 'main', [
      'depth_per_pass_mm',
    ]),
    async () => [{ json: JSON.stringify(c), token: 'v1' }],
  );
  expect(p.data.tools).toEqual(originalTools);
  p.history.undo(p.target());
  expect(reuseRecords(p.data.serializedExtensions)).toHaveLength(0);
  expect(p.data.tools).toEqual(originalTools);
  p.history.redo(p.target());
  expect(reuseRecords(p.data.serializedExtensions)).toHaveLength(1);
  const changed = generationContextKey(p, 'grbl');
  expect(changed).not.toBe(key);
  const saved = JSON.parse(JSON.stringify(snapshotProject(p)));
  expect(saved.version).toBe(10);
  expect(JSON.stringify(saved)).not.toContain('private trial');
  restoreProject(p, saved);
  expect(snapshotProject(p)).toEqual(saved);
  expect(p.data.tools).toEqual(originalTools);
  expect(p.data.operations[0].stages!.main.depth_per_pass_mm).toBe(1.122706142614561);
  const records = reuseRecords(p.data.serializedExtensions);
  expect(records).toHaveLength(1);
  expect(reuseState(p.data.operations[0], records[0], p.data)).toBe('copied');
  expect(p.data.serializedExtensions.vendor).toEqual({ exact: 15.122706142614561 });
  // Advisory data has no generation authority and is intentionally absent from all wire transports.
  const wire = buildGenerationProject(p);
  expect(wire).not.toHaveProperty('fabcamPresetValueReuse');
  p.data.operations[0].stages!.main.depth_per_pass_mm = 1.5;
  expect(reuseState(p.data.operations[0], records[0], p.data)).toBe('modified');
  const restoredKey = generationContextKey(p, 'grbl');
  p.data.serializedExtensions.vendor = { exact: 42 };
  expect(generationContextKey(p, 'grbl')).toBe(restoredKey);
});
