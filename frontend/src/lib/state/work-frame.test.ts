import { it, expect } from 'vitest';
import { inDrawingFrame, supportedWcs } from './work-frame';
import { stockCornerPatch } from './stock-placement';
import { parseFixturePolygon } from './fixture-editor';
import type { GenerateResponse } from '../api/types';
it('keeps emitted program and source lines while aligning path, arc and regions with the drawing', () => {
  const r = {
    gcode: 'M3,0,0,-2',
    toolpath: [
      {
        from: { x: 0, y: 0, z: 0 },
        to: { x: 10, y: 0, z: -2 },
        kind: 'cut',
        gcode_line: 7,
        op_id: 1,
        arc: { cx: 5, cy: 0, ccw: true },
      },
    ],
    regions: [{ op_id: 1, outer: [{ x: 0, y: 0 }], holes: [[{ x: 2, y: 2 }]] }],
    remaining_material: [
      {
        op_id: 1,
        status: 'ready',
        reason: 'resolved_path_at_depth',
        depth_mm: -2,
        area_mm2: 5,
        largest_area_mm2: 5,
        regions: [{ op_id: 1, outer: [{ x: 1, y: 2 }], holes: [[{ x: 3, y: 4 }]] }],
      },
    ],
  } as GenerateResponse;
  const p = inDrawingFrame(r, { x_mm: 10, y_mm: 20, z_mm: 0, wcs: 'G54' });
  expect(p.gcode).toBe(r.gcode);
  expect(p.toolpath[0].gcode_line).toBe(7);
  expect(p.toolpath[0].from).toEqual({ x: 10, y: 20, z: 0 });
  expect(p.toolpath[0].arc?.cx).toBe(15);
  expect(p.regions?.[0].holes?.[0][0]).toEqual({ x: 12, y: 22 });
  expect(p.remaining_material?.[0].regions[0].outer[0]).toEqual({ x: 11, y: 22 });
  expect(p.remaining_material?.[0].regions[0].holes?.[0][0]).toEqual({ x: 13, y: 24 });
  expect(p.remaining_material?.[0].area_mm2).toBe(5);
  expect(r.remaining_material?.[0].regions[0].outer[0]).toEqual({ x: 1, y: 2 });
  expect(r.toolpath[0].from.x).toBe(0);
});
it('edits effective stock corner without reinterpreting legacy bbox-relative placement', () => {
  expect(stockCornerPatch('x', 0, { minX: -190, minY: -190 }, { offsetX: 0 })).toEqual({
    offsetX: 190,
  });
  expect(stockCornerPatch('y', 5, { minX: 10, minY: 20 }, { offsetY: 5 })).toEqual({
    offsetY: -10,
  });
});
it('permits only qualified work systems for the selected post', () => {
  expect(supportedWcs('cps')).toEqual(['G54']);
  expect(supportedWcs('grbl')).toEqual(['G54']);
  expect(supportedWcs('linuxcnc')).toContain('G59');
});
it('refuses self-crossing, duplicate, degenerate and non-finite fixture polygons', () => {
  for (const s of ['0,0\n10,10\n0,10\n10,0', '0,0\n0,0\n10,10', 'NaN,0\n1,0\n1,1', '0,0\n1,0\n2,0'])
    expect(parseFixturePolygon(s)).toBeNull();
  expect(parseFixturePolygon('0,0\n10,0\n10,10\n5,5\n0,10')).toHaveLength(5);
});
