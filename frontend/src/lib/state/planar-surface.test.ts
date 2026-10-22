import { describe, expect, it } from 'vitest';
import { buildOpEntry } from './op_defaults';
import type { PlanarSurfaceOp } from './op_types';
import type { ProjectFile, ToolEntry } from './project-types';
import { surfaceBounds, surfaceSizingView } from './planar-surface';
import { History } from './history';
import { updateOperationCommand } from './commands';
import { validateProjectV9 as validateProjectFile } from './project-v9';
import { explicitProjectFixture } from './explicit-project.fixture';
import { buildProject } from '../api/build-project';

const tool = {
  id: 2,
  name: 'Flat',
  kind: 'endmill',
  diameter: 6,
  speed: 18000,
  feedRate: 1200,
  plungeRate: 200,
} as ToolEntry;
function op() {
  return buildOpEntry('planar_surface', {
    nextId: 8,
    tools: [{ id: 1, kind: 'ball_nose' } as ToolEntry, tool],
    reliefSources: [],
    selectionIds: [99],
    objectMeta: [],
    surfaceZone: { minX: 20, minY: 30, maxX: 60, maxY: 50 },
  }) as PlanarSurfaceOp;
}
function file(operation = op()): ProjectFile {
  return explicitProjectFixture({
    tools: [tool],
    kind: 'ivac-project-surfacing',
    version: 6,
    imports: [],
    visibleLayers: [],
    selectedEntities: [],
    operations: [operation],
  });
}
describe('UX3 planar surfacing', () => {
  it('uses an explicit stock snapshot and flat tool, independent of geometry selection', () => {
    expect(op()).toMatchObject({
      toolId: 2,
      depth: -0.5,
      step: -0.5,
      surface: { origin: [20, 30], width_mm: 40, height_mm: 20 },
    });
    expect(op().sourceObjects).toBeUndefined();
  });
  it('requires explicit assignment when only rounded cutters exist', () => {
    expect(
      buildOpEntry('planar_surface', {
        nextId: 1,
        tools: [{ id: 1, kind: 'ball_nose' } as ToolEntry],
        reliefSources: [],
        selectionIds: [],
        objectMeta: [],
      }).toolId,
    ).toBe(0);
  });
  it('computes distinct target, center travel and full cutter envelope', () => {
    const operation = op();
    operation.surface.margin_mm = 2;
    expect(surfaceBounds(operation, 6)).toEqual({
      target: [18, 28, 62, 52],
      center: [14, 28, 66, 52],
      cutter: [11, 25, 69, 55],
    });
    operation.surface.direction = 'along_y';
    expect(surfaceBounds(operation, 6)?.center).toEqual([18, 24, 62, 56]);
  });
  it('rejects invalid displayed extents rather than hiding negative overtravel', () => {
    const operation = op();
    operation.surface.overtravel_mm = -1;
    expect(surfaceBounds(operation, 6)).toBeNull();
    operation.surface.overtravel_mm = NaN;
    expect(surfaceBounds(operation, 6)).toBeNull();
  });
  it('supplies standalone sizing without any synthetic source contour', () => {
    const view = surfaceSizingView([op()]);
    expect(view?.segments).toEqual([]);
    expect(view?.objects).toEqual([]);
    expect(view?.bbox).toEqual({ min_x: 20, min_y: 30, max_x: 60, max_y: 50 });
    expect(surfaceSizingView([{ ...op(), enabled: false }])).toBeNull();
  });
  it('persists parameters under a guarded format and refuses silent legacy use or malformed data', () => {
    const restored = JSON.parse(JSON.stringify(file())) as ProjectFile;
    expect(() => validateProjectFile(restored)).not.toThrow();
    expect(restored.operations?.[0]).toEqual(file().operations?.[0]);
    expect(() => validateProjectFile({ ...restored, kind: 'ivac-project', version: 1 })).toThrow();
    const malformed = op();
    malformed.surface.width_mm = 0;
    expect(() => validateProjectFile(file(malformed))).toThrow();
  });
  it('undoes/redo zone edits atomically, including edited margins and direction', () => {
    const operation = op();
    const state = { operations: [operation], dirty: false };
    const history = new History();
    const surface = {
      ...operation.surface,
      origin: [35, 40],
      direction: 'along_y' as const,
      margin_mm: 2,
    };
    history.exec(updateOperationCommand(operation.id, { surface }), state);
    expect(state.operations[0].surface).toEqual(surface);
    history.undo(state);
    expect(state.operations[0].surface).toEqual(operation.surface);
    history.redo(state);
    expect(state.operations[0].surface).toEqual(surface);
  });
  it('maps authored mm and manual cutting values through the existing wire adapter', () => {
    const operation = op();
    operation.cutting!.rates[0].feed_mm_min = 1900;
    const project = buildProject({
      transformedImport: surfaceSizingView([operation]),
      operations: [operation],
      tools: [tool],
      machine: {
        unit: 'mm',
        mode: 'mill',
        comments: true,
        arcs: true,
        fastMoveZ: 5,
        toolchangeStrategy: 'manual_m0_pause',
      } as Parameters<typeof buildProject>[0]['machine'],
    });
    expect(project?.segments).toEqual([]);
    expect(project?.operations[0].kind).toEqual({
      type: 'planar_surface',
      surface: operation.surface,
    });
    expect(
      project?.operations[0].params.cutting?.rates.find((s) => s.role === 'rough')?.feed_mm_min,
    ).toBe(1900);
    expect(project?.cutting_version).toBe(9);
  });
});
