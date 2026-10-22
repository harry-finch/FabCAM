import { describe, it, expect } from 'vitest';
import type { ImportResponse, Segment } from '../api/types';
import type { ImportEntry, ToolEntry, ProjectFile } from './project-types';
import type { OpEntry } from './op_types';
import { identityFileTransform } from './project-types';
import {
  reliefBounds,
  reliefSourceKey,
  reliefCommand,
  reliefError,
  reliefOperationError,
  effectiveDiameter,
  type CornerReliefModifier,
} from './corner-relief';
import { History } from './history';
import { validateProjectV9 as validateProjectFile } from './project-v9';
import { explicitProjectFixture } from './explicit-project.fixture';
const seg: Segment = {
  type: 'ARC',
  start: { x: 0, y: 0 },
  end: { x: 6, y: 0 },
  bulge: 1,
  layer: '0',
  color: 7,
};
const source = {
  segments: [seg],
  objects: [1],
  layers: [{ name: '0', color: 7, segment_count: 1 }],
  object_meta: [{ id: 1, bbox: reliefBounds([seg]) }],
  bbox: reliefBounds([seg]),
} as ImportResponse;
const imports: ImportEntry[] = [{ id: 1, source, fileTransform: identityFileTransform() }];
const tool = { id: 1, kind: 'endmill', diameter: 6.35, wearOffsetMm: 0.35 } as ToolEntry;
function modifier(): CornerReliefModifier {
  return {
    version: 1,
    sourceKey: reliefSourceKey(imports),
    toolId: 1,
    diameter: 6,
    recipe: {
      mode: 'dogbone',
      removal_side: 'interior',
      selected_objects: [1],
      selected_corners: [{ object_id: 1, corner_index: 0 }],
    },
    evaluated: source,
  };
}
describe('persistent corner relief', () => {
  it('includes exact arc extrema beyond endpoint bounds', () => {
    const b = reliefBounds([seg]);
    expect(b.min_x).toBeCloseTo(0, 10);
    expect(b.max_x).toBeCloseTo(6, 10);
    expect(b.min_y).toBeCloseTo(-3, 10);
    expect(b.max_y).toBeCloseTo(0, 10);
  });
  it('applies/removes atomically and retains immutable before/after for undo and redo', () => {
    const h = new History();
    const state: { cornerRelief?: CornerReliefModifier; dirty: boolean } = { dirty: false };
    const m = modifier();
    h.exec(reliefCommand(undefined, m), state);
    m.diameter = 100;
    expect(state.cornerRelief?.diameter).toBe(6);
    expect(state.dirty).toBe(true);
    h.undo(state);
    expect(state.cornerRelief).toBeUndefined();
    expect(state.dirty).toBe(false);
    h.redo(state);
    expect(state.cornerRelief?.diameter).toBe(6);
    h.exec(reliefCommand(state.cornerRelief, undefined), state);
    expect(state.cornerRelief).toBeUndefined();
    h.undo(state);
    expect(state.cornerRelief?.diameter).toBe(6);
  });
  it('invalidates on import transform, source or actual cutter diameter changes', () => {
    const m = modifier();
    expect(effectiveDiameter(tool)).toBe(6);
    expect(reliefError(m, imports, [tool])).toBeNull();
    expect(
      reliefError(
        m,
        [{ ...imports[0], fileTransform: { ...imports[0].fileTransform, scale: 2 } }],
        [tool],
      ),
    ).toContain('Drawing changed');
    expect(reliefError(m, imports, [{ ...tool, wearOffsetMm: 0 }])).toContain('Cutter changed');
    expect(reliefError(m, [], [tool])).toContain('Drawing changed');
  });
  it('round-trips v9 relief and refuses malformed geometry', () => {
    const file = explicitProjectFixture({
      kind: 'ivac-project',
      version: 9,
      imports,
      cornerRelief: modifier(),
    } as ProjectFile);
    const restored = JSON.parse(JSON.stringify(file)) as ProjectFile;
    expect(() => validateProjectFile(restored)).not.toThrow();
    expect(restored.cornerRelief).toEqual(file.cornerRelief);
    expect(() => validateProjectFile({ ...file, kind: 'ivac-project', version: 1 })).toThrow();
    expect(() =>
      validateProjectFile({ ...file, cornerRelief: { ...modifier(), diameter: 0 } }),
    ).toThrow();
  });
  it('does not silently move tabs or permit a different cutter for affected geometry', () => {
    const op = {
      id: 1,
      name: 'Pocket',
      enabled: true,
      kind: 'pocket',
      sourceLayers: null,
      toolId: 1,
    } as OpEntry;
    expect(reliefOperationError(modifier(), [op])).toBeNull();
    expect(reliefOperationError(modifier(), [{ ...op, toolId: 2 }])).toContain('cutter');
    expect(
      reliefOperationError(modifier(), [
        { ...op, tabPlacements: [{ objectId: 1, t: 0.5 }] } as OpEntry,
      ]),
    ).toContain('tabs');
    expect(reliefOperationError(modifier(), [{ ...op, sourceObjects: [2], toolId: 2 }])).toBeNull();
  });
});

it('an independent surfacing rectangle does not consume or rebind corner relief', () => {
  const surface = {
    id: 2,
    name: 'Surface',
    enabled: true,
    kind: 'planar_surface',
    toolId: 99,
    sourceLayers: null,
  } as OpEntry;
  expect(reliefOperationError(modifier(), [surface])).toBeNull();
});
