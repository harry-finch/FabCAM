import { describe, expect, it } from 'vitest';
import { FixtureGesture } from './fixture-gesture';
import { fixtureAt } from './fixture-hit';
import { History } from '../state/history';
import { updateFixtureCommand, type CommandTarget } from '../state/commands';
import { drawFixtures } from './render/fixtures';
import { stubCtx, flipY } from './render/stub-ctx';
import { vi } from 'vitest';
vi.mock('../state/text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('../state/workspace.svelte', () => ({ workspace: {} }));
import { snapshotProject } from '../state/project-file-ops';
import type { ProjectState } from '../state/project.svelte';
import type { FixtureKind } from '../state/project-types';

const kinds: FixtureKind[] = [
  { shape: 'box', width: 20, depth: 10 },
  { shape: 'cylinder', radius: 5 },
  {
    shape: 'polygon',
    vertices: [
      [-5, -5],
      [5, -5],
      [5, 5],
      [-5, 5],
    ],
  },
];
function target(kind: FixtureKind): CommandTarget {
  return {
    fixtures: [
      { id: 1, name: 'Clamp', origin: [7, 60], z_bottom: 0, z_top: 10, color: 0xffffffff, kind },
    ],
    workOffset: { x_mm: 20, y_mm: 30, z_mm: 0, wcs: 'G54' },
    stock: {} as CommandTarget['stock'],
    operations: [],
    tools: [],
    imports: [],
    textLayers: [],
    reliefSources: [],
    machine: {} as CommandTarget['machine'],
    settings: {} as CommandTarget['settings'],
    groupOpsByTool: false,
    machineProfileId: null,
    dirty: false,
  };
}
describe('fixture gesture uses actual command target and history', () => {
  for (const kind of kinds)
    it(`${kind.shape}: click hit, live XY, one Undo/Redo, saved record`, () => {
      const t = target(kind),
        h = new History(),
        before = structuredClone(t);
      expect(fixtureAt(t.fixtures, 7, 60)).toBe(1);
      const drag = new FixtureGesture(1, 4, { x: 8, y: 62 }, t, h);
      for (let n = 1; n <= 100; n++) drag.move({ x: 8 - n, y: 62 + 2 * n });
      expect(t.fixtures[0].origin).toEqual([-93, 260]);
      expect(h.undoSize).toBe(0);
      expect(t.dirty).toBe(true);
      expect(t.fixtures[0].kind).toEqual(before.fixtures[0].kind);
      expect(t.fixtures[0].z_top).toBe(10);
      expect(t.workOffset).toEqual(before.workOffset);
      expect(t.stock).toEqual(before.stock);
      drag.finish();
      expect(h.undoSize).toBe(1);
      h.undo(t);
      expect(t.fixtures[0].origin).toEqual([7, 60]);
      expect(t.dirty).toBe(false);
      h.redo(t);
      expect(t.fixtures[0].origin).toEqual([-93, 260]);
      // Same snapshot path as Save. No visual-only placement or schema migration.
      const saved = snapshotProject({ data: t } as ProjectState);
      const reopened = JSON.parse(JSON.stringify(saved));
      expect(reopened.fixtures[0].origin).toEqual([-93, 260]);
      expect(reopened.workOffset).toEqual(before.workOffset);
      h.exec(updateFixtureCommand(1, { origin: [3, 4] }), t);
      expect(fixtureAt(t.fixtures, 3, 4)).toBe(1);
      expect(fixtureAt(t.fixtures, -93, 260)).toBeNull();
    });
  it('cancel/click preserve dirty state and consecutive drags do not merge', () => {
    const t = target(kinds[0]),
      h = new History();
    new FixtureGesture(1, 1, { x: 0, y: 0 }, t, h).finish();
    expect(h.undoSize).toBe(0);
    let drag = new FixtureGesture(1, 1, { x: 0, y: 0 }, t, h);
    drag.move({ x: 10, y: 10 });
    drag.finish(true);
    expect(t.fixtures[0].origin).toEqual([7, 60]);
    expect(t.dirty).toBe(false);
    drag = new FixtureGesture(1, 1, { x: 0, y: 0 }, t, h);
    drag.move({ x: 10, y: 10 });
    drag.finish();
    drag = new FixtureGesture(1, 1, { x: 0, y: 0 }, t, h);
    drag.move({ x: 10, y: 10 });
    drag.finish();
    expect(h.undoSize).toBe(2);
    h.undo(t);
    expect(t.fixtures[0].origin).toEqual([17, 70]);
  });
  it('critical outline remains red and selection adds a distinct dashed stroke', () => {
    const t = target(kinds[0]),
      s = stubCtx();
    drawFixtures(s.ctx, flipY, t.fixtures, 1, 'blue', new Set([1]), 'red');
    expect(s.ops('strokeRect')[0].strokeStyle).toBe('red');
    expect(s.ops('setLineDash')[0].args).toEqual([[5, 2]]);
  });
});

import { FixtureInteraction, frameFixture } from './fixture-interaction';
import { computeViewportTransform } from './viewport';
it('pointer selection is optional, captured mouse drag cancels cleanly and frame centers actual bounds', () => {
  const t = target(kinds[0]),
    h = new History();
  const sel = { selectedFixtureId: null as number | null, selectedTextLayerId: null };
  const p = {
    target: () => t,
    history: h,
    sel,
    selectFixture: (id: number) => {
      sel.selectedFixtureId = id;
    },
  } as unknown as ProjectState;
  const capture = vi.fn(),
    release = vi.fn(),
    activate = vi.fn();
  const canvas = {
    style: { cursor: '' },
    setPointerCapture: capture,
    hasPointerCapture: () => true,
    releasePointerCapture: release,
  } as unknown as HTMLCanvasElement;
  const tools = new FixtureInteraction(
    p,
    () => canvas,
    (x, y) => ({ x, y: -y }),
    activate,
  );
  const down = { pointerId: 1, pointerType: 'mouse', clientX: 7, clientY: -60 } as PointerEvent;
  tools.select(1, down, 7, -60);
  expect(sel.selectedFixtureId).toBe(1);
  expect(capture).not.toHaveBeenCalled();
  tools.select(1, down, 7, -60);
  expect(capture).toHaveBeenCalledWith(1);
  tools.move({ ...down, clientX: 8 } as PointerEvent);
  expect(t.fixtures[0].origin).toEqual([7, 60]);
  tools.move({ ...down, clientX: 27, clientY: -50 } as PointerEvent);
  expect(t.fixtures[0].origin).toEqual([27, 50]);
  tools.end({ ...down, type: 'pointercancel' } as PointerEvent);
  expect(t.fixtures[0].origin).toEqual([7, 60]);
  expect(h.undoSize).toBe(0);
  const bbox = { min_x: -25, max_x: 500, min_y: 0, max_y: 500 };
  const base = computeViewportTransform(bbox, { w: 900, h: 600 }, { zoom: 1, panX: 0, panY: 0 });
  const framed = frameFixture(
    t.fixtures[0],
    { scale: base.baseScale, offX: base.baseOffX, offY: base.baseOffY },
    900,
    600,
  );
  const transform = computeViewportTransform(bbox, { w: 900, h: 600 }, framed);
  expect(transform.project2(7, 60)).toEqual([450, 300]);
});
