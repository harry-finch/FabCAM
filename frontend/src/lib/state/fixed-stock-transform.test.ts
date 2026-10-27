import { expect, it, vi } from 'vitest';
import { History } from './history';
import { combineImports } from './file-transform';
import { identityFileTransform } from './project-types';
import { computeFootprint } from '../sim/footprint';
import type { ProjectState } from './project.svelte';
vi.mock('./mode_notice.svelte', () => ({ modeNotice: { current: null } }));
vi.mock('./text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
import {
  addImported,
  setImported,
  patchFileTransformForImport,
  resetFileTransformForImport,
} from './import-ops';
import { setStock, setWorkOffset } from './project-machine-ops';
import { clearForDrawingImport } from './stock-placement';
import { removeImport, removeImportedLayer } from './import-ops';
import { snapshotProject, restoreProject } from './project-file-ops';
vi.mock('./workspace.svelte', () => ({ workspace: {} }));
function fixture(mode: 'auto' | 'manual') {
  const points = [
    [10, 20],
    [30, 20],
    [30, 40],
    [10, 40],
  ];
  const segments = points.map(([x, y], i) => ({
    type: 'LINE',
    start: { x, y },
    end: { x: points[(i + 1) % 4][0], y: points[(i + 1) % 4][1] },
    bulge: 0,
    layer: '0',
    color: 7,
  }));
  const data = {
    imports: [
      {
        id: 1,
        fileTransform: identityFileTransform(),
        source: {
          filename: 'square.dxf',
          format: 'dxf',
          segments,
          objects: [1, 1, 1, 1],
          object_meta: [],
          layers: [],
          bbox: { min_x: 10, min_y: 20, max_x: 30, max_y: 40 },
          unit_source: 'test',
          unit_status: 'explicit',
          unit_scale: 1,
          warnings: [],
        },
      },
    ],
    machine: { workArea: { x: 500, y: 500 } },
    workOffset: { wcs: 'G54', x_mm: 0, y_mm: 0, z_mm: 0 },
    visibleLayers: new Set<string>(),
    operations: [],
    dirty: false,
    stock: { mode, customX: 150, customY: 150, offsetX: 7, offsetY: 8, margin: 5 },
  };
  const p = {
    data,
    gen: {},
    sel: {},
    history: new History(),
    target: () => data,
    get stockSizingImport() {
      return combineImports(data.imports as ProjectState['data']['imports']);
    },
  } as unknown as ProjectState;
  return { p, footprint: () => computeFootprint(p.stockSizingImport, p.data.stock) };
}
it('keeps manual stock fixed through coalesced undo/redo, reset and portable data', () => {
  const { p, footprint } = fixture('manual');
  const original = footprint();
  patchFileTransformForImport(p, 1, { translate: { x: 10 } }, 'x');
  patchFileTransformForImport(p, 1, { translate: { x: 40 } }, 'x');
  expect(footprint()).toEqual(original);
  p.history.undo(p.target());
  expect(footprint()).toEqual(original);
  expect(p.data.imports[0].fileTransform.translate.x).toBe(0);
  p.history.redo(p.target());
  expect(footprint()).toEqual(original);
  expect(p.data.imports[0].fileTransform.translate.x).toBe(40);
  patchFileTransformForImport(p, 1, { rotateDeg: 45, scale: 2 });
  expect(footprint()).toEqual(original);
  resetFileTransformForImport(p, 1);
  expect(footprint()).toEqual(original);
  const saved = JSON.parse(JSON.stringify(p.data));
  expect(computeFootprint(combineImports(saved.imports), saved.stock)).toEqual(original);
});
it('retains automatic stock following behavior', () => {
  const { p, footprint } = fixture('auto');
  const original = footprint();
  patchFileTransformForImport(p, 1, { translate: { x: 10 } });
  expect(footprint().minX).toBe(original.minX + 10);
});

import { DrawingInteraction } from '../canvas/drawing-interaction';
import { proportionalScale } from './file-transform';
import { buildOpEntry } from './op_defaults';
import { generationContextKey } from './generation-context';
it('commits one drag, preserves stock and cancels without history or source changes', () => {
  const { p, footprint } = fixture('manual');
  const originalStock = footprint();
  Object.defineProperty(p, 'geometryView', { get: () => combineImports(p.data.imports) });
  p.sel = { selectedObjects: new Set([1]) } as ProjectState['sel'];
  p.patchFileTransformForImport = (id, patch) => patchFileTransformForImport(p, id, patch);
  const canvas = {
    style: {},
    setPointerCapture: vi.fn(),
    hasPointerCapture: () => false,
    getBoundingClientRect: () => ({ left: 0, top: 0 }),
  } as unknown as HTMLCanvasElement;
  const movement = new DrawingInteraction(p, () => canvas, vi.fn());
  const event = (type: string, x: number, y: number) =>
    ({
      type,
      pointerId: 1,
      pointerType: 'mouse',
      button: 0,
      clientX: x,
      clientY: y,
      preventDefault: vi.fn(),
    }) as unknown as PointerEvent;
  const start = () => movement.start(event('pointerdown', 0, 0), 0, 0, 0, 2, false);
  expect(start()).toBe(true);
  movement.move(event('pointermove', 20, -10), 20, -10);
  expect(p.history.undoSize).toBe(0);
  movement.cancel();
  expect(p.history.undoSize).toBe(0);
  expect(start()).toBe(true);
  movement.move(event('pointermove', 20, -10), 20, -10);
  movement.end(event('pointerup', 20, -10));
  expect(p.history.undoSize).toBe(1);
  expect(p.data.imports[0].fileTransform.translate).toEqual({ x: 10, y: 5 });
  expect(footprint()).toEqual(originalStock);
  p.history.undo(p.target());
  expect(p.data.imports[0].fileTransform.translate).toEqual({ x: 0, y: 0 });
  p.history.redo(p.target());
  expect(p.data.imports[0].fileTransform.translate).toEqual({ x: 10, y: 5 });
  start();
  movement.end(event('pointercancel', 50, -50));
  expect(p.history.undoSize).toBe(1);
  start();
  p.data.imports = [...p.data.imports]; // source replacement during gesture
  movement.end(event('pointerup', 50, -50));
  expect(p.history.undoSize).toBe(1);
  expect(movement.start(event('pointerdown', 0, 0), 4, 0, 0, 2, false)).toBe(false); // stock
  expect(movement.start(event('pointerdown', 0, 0), 0, 0, 0, 2, true)).toBe(false); // modifiers
});
it('proportional sizing persists exact values and invalidates generated context across undo/redo', () => {
  const { p, footprint } = fixture('manual');
  Object.assign(p.data, {
    machine: { unit: 'mm', mode: 'mill', fastMoveZ: 5 },
    tools: [],
    fixtures: [],
    textLayers: [],
    reliefSources: [],
    groupOpsByTool: false,
  });
  Object.defineProperty(p, 'transformedImport', { get: () => combineImports(p.data.imports) });
  const original = footprint();
  p.data.operations = [
    buildOpEntry('profile', {
      nextId: 1,
      tools: [],
      reliefSources: [],
      selectionIds: [],
      objectMeta: [],
    }),
  ];
  const key = generationContextKey(p, 'grbl');
  const scale = proportionalScale(p.data.imports[0], 'width', 102.6)!;
  patchFileTransformForImport(p, 1, { scale });
  expect(footprint()).toEqual(original);
  const changed = generationContextKey(p, 'grbl');
  expect(changed).not.toBe(key);
  const saved = JSON.parse(JSON.stringify(p.data));
  expect(saved.imports[0].fileTransform.scale).toBe(scale);
  expect(computeFootprint(combineImports(saved.imports), saved.stock)).toEqual(original);
  p.history.undo(p.target());
  expect(generationContextKey(p, 'grbl')).toBe(key);
  p.history.redo(p.target());
  expect(generationContextKey(p, 'grbl')).toBe(changed);
});

it('moves one imported file with its approach marker while preserving other files and stock', () => {
  const { p, footprint } = fixture('manual');
  const second = {
    ...p.data.imports[0],
    id: 2,
    fileTransform: { ...identityFileTransform(), translate: { x: 100, y: 0 } },
  };
  p.data.imports = [...p.data.imports, second];
  const op = buildOpEntry('profile', {
    nextId: 1,
    tools: [],
    reliefSources: [],
    selectionIds: [1],
    objectMeta: [],
  });
  if (op.kind !== 'profile') throw new Error('Expected profile fixture');
  op.sourceObjects = [1];
  op.approachPoint = [10, 20];
  p.data.operations = [op];
  const stock = footprint();
  patchFileTransformForImport(p, 1, { translate: { x: 15, y: -5 } });
  expect(p.data.operations[0]).toMatchObject({ approachPoint: [25, 15] });
  expect(p.data.imports[1]).toEqual(second);
  expect(footprint()).toEqual(stock);
  expect(p.history.undoSize).toBe(1);
  p.history.undo(p.target());
  expect(p.data.operations[0]).toMatchObject({ approachPoint: [10, 20] });
  expect(footprint()).toEqual(stock);
});

it('resizes manual stock towards positive axes from work zero, atomically undoable', () => {
  const { p, footprint } = fixture('manual');
  const original = footprint();
  setStock(p, { customX: 300, customY: 200 });
  expect(footprint()).toMatchObject({ minX: 0, minY: 0, maxX: 300, maxY: 200 });
  expect(p.data.workOffset).toEqual({ wcs: 'G54', x_mm: 0, y_mm: 0, z_mm: 0 });
  p.history.undo(p.target());
  expect(footprint()).toEqual(original);
  p.history.redo(p.target());
  expect(footprint()).toMatchObject({ minX: 0, minY: 0, maxX: 300, maxY: 200 });
});
it('anchors resizing to a nonzero work origin without moving it', () => {
  const { p, footprint } = fixture('manual');
  p.data.workOffset.x_mm = 12;
  p.data.workOffset.y_mm = -8;
  setStock(p, { customX: 300, customY: 200 });
  expect(footprint()).toMatchObject({ minX: 12, minY: -8, maxX: 312, maxY: 192 });
});
it('preserves manual stock and WCS when replacing and adding distant source geometry', () => {
  const { p, footprint } = fixture('manual');
  const original = footprint();
  const source = structuredClone(p.data.imports[0].source);
  for (const seg of source.segments) {
    seg.start.x += 1000;
    seg.end.x += 1000;
  }
  source.bbox.min_x += 1000;
  source.bbox.max_x += 1000;
  setImported(p, source);
  expect(p.data.imports[0].fileTransform).toEqual(identityFileTransform());
  expect(footprint()).toEqual(original);
  expect(p.data.workOffset).toEqual({ wcs: 'G54', x_mm: 0, y_mm: 0, z_mm: 0 });
  const second = structuredClone(source);
  second.bbox.min_y += 2000;
  second.bbox.max_y += 2000;
  for (const seg of second.segments) {
    seg.start.y += 2000;
    seg.end.y += 2000;
  }
  addImported(p, second);
  expect(footprint()).toEqual(original);
  p.history.undo(p.target());
  expect(p.data.imports).toHaveLength(1);
  expect(footprint()).toEqual(original);
  p.history.redo(p.target());
  expect(p.data.imports).toHaveLength(2);
  expect(footprint()).toEqual(original);
});
it('auto-fit follows imported geometry but never moves work zero', () => {
  const { p, footprint } = fixture('auto');
  const source = structuredClone(p.data.imports[0].source);
  const zero = { ...p.data.workOffset };
  setImported(p, source);
  expect(p.data.workOffset).toEqual(zero);
  expect(footprint().minX).toBe(source.bbox.min_x - p.data.stock.margin + 7);
});

it('first additive import is undoable without resetting earlier setup history', () => {
  const { p, footprint } = fixture('manual');
  const source = p.data.imports[0].source;
  p.data.imports = [];
  setStock(p, { customX: 300, customY: 200 });
  const original = footprint();
  addImported(p, source);
  expect(p.history.undoSize).toBe(2);
  p.history.undo(p.target());
  expect(p.data.imports).toEqual([]);
  expect(footprint()).toEqual(original);
  p.history.redo(p.target());
  expect(footprint()).toEqual(original);
});
it('replacement document boundary preserves blank/zero while clearing old machining', () => {
  const { p, footprint } = fixture('manual');
  const source = structuredClone(p.data.imports[0].source);
  p.data.workOffset = { wcs: 'G55', x_mm: 12, y_mm: -8, z_mm: 3 };
  const zero = { ...p.data.workOffset },
    original = footprint();
  p.clearProject = () => {
    p.data.imports = [];
    p.data.operations = [];
    p.data.workOffset = { wcs: 'G54', x_mm: 0, y_mm: 0, z_mm: 0 };
  };
  clearForDrawingImport(p);
  setImported(p, source);
  expect(p.data.workOffset).toEqual(zero);
  expect(footprint()).toEqual(original);
  expect(p.data.imports[0].source).toEqual(source);
});
it.each(['manual', 'auto'] as const)(
  'removes/adds geometry with undo and real v9 roundtrip in %s mode',
  (mode) => {
    const { p, footprint } = fixture(mode);
    const original = footprint();
    const source = structuredClone(p.data.imports[0].source);
    source.layers = [{ name: 'far', color: 7, segment_count: 4 }];
    source.segments.forEach((s) => {
      s.start.x += 1000;
      s.end.x += 1000;
      s.layer = 'far';
    });
    source.bbox.min_x += 1000;
    source.bbox.max_x += 1000;
    addImported(p, source);
    if (mode === 'manual') expect(footprint()).toEqual(original);
    else expect(footprint().maxX).toBeGreaterThan(original.maxX);
    const added = footprint();
    removeImportedLayer(p, 'far');
    expect(footprint()).toEqual(original);
    p.history.undo(p.target());
    expect(footprint()).toEqual(added);
    removeImport(p, 2);
    expect(footprint()).toEqual(original);
    p.history.undo(p.target());
    expect(footprint()).toEqual(added);
    Object.assign(p.data, {
      tools: [],
      fixtures: [],
      textLayers: [],
      reliefSources: [],
      serializedExtensions: {},
      machine: { ...p.data.machine, mode: 'mill' },
    });
    const saved = JSON.parse(JSON.stringify(snapshotProject(p)));
    restoreProject(p, saved);
    expect(footprint()).toEqual(added);
    expect(p.data.workOffset).toEqual(
      saved.workOffset ?? { wcs: 'G54', x_mm: 0, y_mm: 0, z_mm: 0 },
    );
    expect(p.data.imports).toEqual(saved.imports);
  },
);
it('repeated one-axis resizes and mode changes preserve geometry and Z, and invalidate generation', () => {
  const { p, footprint } = fixture('auto');
  Object.assign(p.data, {
    machine: { unit: 'mm', mode: 'mill', fastMoveZ: 5 },
    tools: [],
    fixtures: [],
    textLayers: [],
    reliefSources: [],
    groupOpsByTool: false,
  });
  Object.defineProperty(p, 'transformedImport', { get: () => combineImports(p.data.imports) });
  const geometry = JSON.stringify(p.data.imports);
  setWorkOffset(p, { x_mm: 15, y_mm: -20, z_mm: 4 });
  setStock(p, { mode: 'manual' });
  p.data.operations = [
    buildOpEntry('profile', {
      nextId: 1,
      tools: [],
      reliefSources: [],
      selectionIds: [],
      objectMeta: [],
    }),
  ];
  const key = generationContextKey(p, 'grbl');
  setStock(p, { customX: 300 });
  expect(generationContextKey(p, 'grbl')).not.toBe(key);
  setStock(p, { customY: 200 });
  expect(footprint()).toMatchObject({ minX: 15, minY: -20, maxX: 315, maxY: 180 });
  expect(p.data.workOffset.z_mm).toBe(4);
  p.history.undo(p.target());
  expect(footprint().maxY).toBe(130);
  p.history.redo(p.target());
  expect(footprint().maxY).toBe(180);
  setStock(p, { mode: 'auto' });
  setStock(p, { mode: 'manual' });
  expect(footprint()).toMatchObject({ minX: 15, minY: -20, maxX: 315, maxY: 180 });
  expect(JSON.stringify(p.data.imports)).toBe(geometry);
});
