import { readFileSync } from 'node:fs';
import { describe, expect, it, vi } from 'vitest';
import { drawingLayers, layerSwatch } from './drawing-layers';
import { combineImports } from './file-transform';
import { generationContextKey } from './generation-context';
import { toggleLayer, removeImportedLayer, removeImport } from './import-ops';
import { snapshotProject } from './project-file-ops';
import { History } from './history';
import type { ProjectFile } from './project-types';
import type { ProjectState } from './project.svelte';

// The node-only suite does not instantiate reactive text/workspace runtimes.
vi.mock('./text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('./workspace.svelte', () => ({ workspace: {} }));

const saved = () =>
  JSON.parse(
    readFileSync(
      new URL(
        '../../../../docs/shopbot-modernization/GEOMETRY-software-sources.ivac-project.json',
        import.meta.url,
      ),
      'utf8',
    ),
  ) as ProjectFile;

function projectFixture() {
  const file = saved();
  const data = {
    ...file,
    // The fixture omits view preferences; normal import restore reveals layers.
    visibleLayers: new Set(file.imports.flatMap((entry) => entry.source.layers.map((l) => l.name))),
    dirty: false,
    reliefSources: [],
    groupOpsByTool: false,
  };
  return {
    data,
    sel: { selectedEntities: new Set([1]) },
    gen: { generated: { sentinel: true } },
    history: new History(),
    target: () => data,
    get transformedImport() {
      return combineImports(data.imports);
    },
    get stockSizingImport() {
      return combineImports(data.imports);
    },
  } as unknown as ProjectState;
}

describe('imported drawing and layer presentation', () => {
  it('distinguishes three source documents, two exact-name layers and sixteen segments', () => {
    const imports = saved().imports;
    const before = JSON.stringify(imports);
    const layers = drawingLayers(imports);
    expect(imports).toHaveLength(3);
    expect(
      layers.map(({ name, segment_count, color }) => ({ name, segment_count, color })),
    ).toEqual([
      { name: 'svg', segment_count: 12, color: 7 },
      { name: 'Cuts', segment_count: 4, color: 7 },
    ]);
    expect(layers[0].sources.map((s) => s.filename)).toEqual(['nested.svg', 'detail.svg']);
    expect(layers[1].sources.map((s) => s.filename)).toEqual(['plate.dxf']);
    expect(layers.map((l) => [l.name, l.segment_count])).toEqual(
      combineImports(imports)!.layers.map((l) => [l.name, l.segment_count]),
    );
    expect(JSON.stringify(imports)).toBe(before);
  });
  it('retains first layer color and exact case, excluding empty metadata without changing imports', () => {
    const imports = saved().imports;
    imports[1].source.layers[0].color = 3;
    imports[2].source.layers.push({ name: 'cuts', color: 9, segment_count: 2 });
    imports[2].source.layers.push({ name: 'Empty', color: 7, segment_count: 0 });
    const layers = drawingLayers(imports);
    expect(layers.map((l) => l.name)).toEqual(['svg', 'Cuts', 'cuts']);
    expect(layers[0].color).toBe(7);
    expect(layers[2].color).toBe(9);
    expect(drawingLayers([])).toEqual([]);
  });
  it('uses the shared drawing palette, including ACI9 and theme colors', () => {
    expect(layerSwatch(1)).toBe('#ff0000');
    expect(layerSwatch(3)).toBe('#00ff00');
    expect(layerSwatch(9)).toBe('#808080');
    for (const color of [7, 256]) expect(layerSwatch(color)).toBe('var(--text-strong)');
    expect(layerSwatch(8)).toBe('var(--text-muted)');
    expect(layerSwatch(99)).toBe('var(--text-faint)');
  });
});

describe('existing view-state and CAM boundaries', () => {
  it('real layer toggles preserve full CAM inputs, selected IDs, generated result and history', () => {
    const p = projectFixture();
    const key = generationContextKey(p, 'cps');
    const generated = p.gen.generated;
    const sources = JSON.stringify(p.data.operations);
    toggleLayer(p, 'Cuts');
    toggleLayer(p, 'svg');
    expect(p.data.visibleLayers.size).toBe(0);
    expect(p.transformedImport!.segments).toHaveLength(16);
    expect(generationContextKey(p, 'cps')).toBe(key);
    expect(p.gen.generated).toBe(generated);
    expect([...p.sel.selectedEntities]).toEqual([1]);
    expect(JSON.stringify(p.data.operations)).toBe(sources);
    expect(p.data.dirty).toBe(false);
    expect(p.history.undoLabel()).toBeNull();
    toggleLayer(p, 'Cuts');
    toggleLayer(p, 'svg');
    expect([...p.data.visibleLayers].sort()).toEqual(['Cuts', 'svg']);
    expect(generationContextKey(p, 'cps')).toBe(key);
  });
  it('portable save retains imports/colors/sources while view visibility stays outside the file', () => {
    const p = projectFixture();
    toggleLayer(p, 'svg');
    const file = snapshotProject(p);
    expect(file.visibleLayers).toEqual([]);
    expect(file.selectedEntities).toEqual([]);
    expect(file.imports).toEqual(saved().imports);
    expect(file.operations).toEqual(saved().operations);
    expect(JSON.parse(JSON.stringify(file)).imports[2].source.layers[0].name).toBe('Cuts');
  });
  it('layer deletion removes all backing SVG geometry; Undo/Redo preserve source references', () => {
    const p = projectFixture();
    const key = generationContextKey(p, 'cps');
    const sources = JSON.stringify(p.data.operations);
    removeImportedLayer(p, 'svg');
    expect(p.data.imports).toHaveLength(3);
    expect(p.data.imports.map((e) => e.source.segments.length)).toEqual([0, 0, 4]);
    expect(drawingLayers(p.data.imports).map((l) => l.name)).toEqual(['Cuts']);
    expect(generationContextKey(p, 'cps')).not.toBe(key);
    expect(JSON.stringify(p.data.operations)).toBe(sources);
    p.history.undo(p.target());
    expect(p.transformedImport!.segments).toHaveLength(16);
    expect(generationContextKey(p, 'cps')).toBe(key);
    expect(p.data.visibleLayers.has('svg')).toBe(false); // existing non-history view state
    p.history.redo(p.target());
    expect(p.transformedImport!.segments).toHaveLength(4);
  });
  it('removing a document keeps the remaining shared layer, and Undo restores counts', () => {
    const p = projectFixture();
    removeImport(p, 2);
    expect(p.data.imports).toHaveLength(2);
    expect(drawingLayers(p.data.imports).find((l) => l.name === 'svg')!.segment_count).toBe(8);
    expect(p.gen.generated).toBeNull();
    p.history.undo(p.target());
    expect(drawingLayers(p.data.imports).find((l) => l.name === 'svg')!.segment_count).toBe(12);
  });
});

it('preserves accepted repairs when a watched source changes', async () => {
  const { reimportFromPath } = await import('./import-ops');
  const p = projectFixture();
  const file = JSON.parse(
    readFileSync(
      new URL(
        '../../../../docs/shopbot-modernization/geom1/square-repaired.ivac-project.json',
        import.meta.url,
      ),
      'utf8',
    ),
  );
  p.data.drawing = file.drawing;
  p.setError = vi.fn();
  const originals = p.data.imports;
  const edits = p.data.drawing;
  expect(await reimportFromPath(p, '/changed/source.svg')).toBe(false);
  expect(p.data.imports).toBe(originals);
  expect(p.data.drawing).toBe(edits);
  expect(p.sourceFileStaleNotice).toEqual({ path: '/changed/source.svg', auto_reload: false });
  expect(p.setError).toHaveBeenCalled();
});
