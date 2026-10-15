import { describe, it, expect } from 'vitest';
import { drawGrid, drawAxes, drawWorkArea, drawStock } from './chrome';
import { stubCtx, flipY } from './stub-ctx';

const GRID_COLORS = { minor: '#111', major: '#222' };

describe('drawGrid', () => {
  it.each([0.000001, 0.05, 0.6, 5, 6, 10, 60, 1000, 1000000])(
    'keeps an origin-anchored grid visible and bounded at %s px/mm',
    (scale) => {
      const s = stubCtx();
      drawGrid(s.ctx, 800, 600, scale, 127, -83, GRID_COLORS);
      const vertical = s.ops('moveTo').filter((c) => c.strokeStyle === '#111' && c.args[1] === 0);
      const pitch = Number(vertical[1].args[0]) - Number(vertical[0].args[0]);
      expect(pitch).toBeGreaterThanOrEqual(6 - 1e-9);
      expect(pitch).toBeLessThanOrEqual(60 + 1e-9);
      expect((127 - Number(vertical[0].args[0])) / pitch).toBeCloseTo(
        Math.round((127 - Number(vertical[0].args[0])) / pitch),
      );
      expect(s.ops('stroke')).toHaveLength(2);
      expect(s.ops('moveTo').length).toBeLessThan(270);
    },
  );

  it('preserves the 1mm / 10mm grid at ordinary zoom', () => {
    const s = stubCtx();
    drawGrid(s.ctx, 100, 100, 10, 0, 100, GRID_COLORS);
    const minor = s.ops('moveTo').filter((c) => c.strokeStyle === '#111' && c.args[1] === 0);
    expect(minor.slice(0, 3).map((c) => c.args[0])).toEqual([0, 10, 20]);
    const major = s.ops('moveTo').filter((c) => c.strokeStyle === '#222' && c.args[1] === 0);
    expect(major.slice(0, 2).map((c) => c.args[0])).toEqual([0, 100]);
  });

  it.each([0, NaN, Infinity])('ignores an invalid scale %s', (scale) => {
    const s = stubCtx();
    drawGrid(s.ctx, 100, 100, scale, 0, 100, GRID_COLORS);
    expect(s.calls).toHaveLength(0);
  });

  it('strokes minor then major passes in their own colors', () => {
    const s = stubCtx();
    drawGrid(s.ctx, 100, 100, 10, 0, 100, GRID_COLORS);
    const strokes = s.ops('stroke');
    expect(strokes).toHaveLength(2);
    expect(strokes[0].strokeStyle).toBe('#111');
    expect(strokes[1].strokeStyle).toBe('#222');
  });
});

describe('drawAxes', () => {
  it('draws the X axis at offY and the Y axis at offX', () => {
    const s = stubCtx();
    drawAxes(s.ctx, 200, 100, 30, 70, { x: 'red', y: 'green' });
    const moves = s.ops('moveTo');
    expect(moves[0].args).toEqual([0, 70]); // X axis horizontal line
    expect(moves[0].strokeStyle).toBe('red');
    expect(moves[1].args).toEqual([30, 0]); // Y axis vertical line
    expect(s.ops('lineTo')[1].strokeStyle).toBe('green');
  });
});

describe('drawWorkArea', () => {
  it('no-ops without a positive work area', () => {
    const s = stubCtx();
    drawWorkArea(s.ctx, flipY, null, '#888');
    drawWorkArea(s.ctx, flipY, { x: 0, y: 100 }, '#888');
    expect(s.calls).toHaveLength(0);
  });

  it('strokes a dashed rect from (0,0) to (x,y)', () => {
    const s = stubCtx();
    drawWorkArea(s.ctx, flipY, { x: 100, y: 50 }, '#888');
    expect(s.ops('setLineDash')[0].args).toEqual([[6, 4]]);
    expect(s.ops('strokeRect')[0].args).toEqual([0, -50, 100, 50]);
  });
});

describe('drawStock', () => {
  it('no-ops on an empty footprint', () => {
    const s = stubCtx();
    drawStock(s.ctx, flipY, { minX: 0, minY: 0, maxX: 0, maxY: 10 }, '#888');
    expect(s.calls).toHaveLength(0);
  });

  it('strokes the footprint rect', () => {
    const s = stubCtx();
    drawStock(s.ctx, flipY, { minX: 10, minY: 20, maxX: 110, maxY: 70 }, '#888');
    expect(s.ops('strokeRect')[0].args).toEqual([10, -70, 100, 50]);
  });
});
