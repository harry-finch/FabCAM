import { describe, expect, it } from 'vitest';
import { pocketFramePatch, workZeroPoint, compatibleTextTool } from './preparation';
import { updateOperationCommand, setWorkOffsetCommand, setMaterialCommand } from './commands';
import { History } from './history';
import type { PocketOp } from './op_types';
import { PreviewRequests } from './recommendation-preview';
import { generatedMinimumZ, workshopWarnings } from './workshop-report';
import type { ToolpathSegment } from '../api/types';

const pocket = (): PocketOp => ({
  id: 7,
  kind: 'pocket',
  pocketStrategy: 'cascade',
  step: -1,
  enabled: true,
  name: 'Island',
  toolId: 1,
  sourceLayers: ['part'],
  sourceObjects: [3, 8],
  sourceCombine: 'union',
  depth: -2,
  startDepth: 0,
});
describe('preparation contracts', () => {
  it('converts atomically, preserves sources through save/reopen, and reverses with Undo/Redo', () => {
    const original = pocket();
    const target = { operations: [original], dirty: false };
    const h = new History();
    h.exec(updateOperationCommand(7, pocketFramePatch(6, true)), target);
    expect(h.undoSize).toBe(1);
    expect(target.operations[0]).toMatchObject({
      sourceCombine: 'union',
      sourceObjects: [3, 8],
      frameShape: 'rectangle',
      framePaddingMm: 18,
    });
    h.undo(target);
    expect(target.operations[0]).toMatchObject(original);
    h.redo(target);
    const reopened = JSON.parse(JSON.stringify(target));
    h.exec(updateOperationCommand(7, pocketFramePatch(0, false)), reopened);
    expect(reopened.operations[0]).toMatchObject(original);
    expect(reopened.operations[0].frameShape).toBeUndefined();
    h.undo(reopened);
    expect(reopened.operations[0].frameShape).toBe('rectangle');
  });
  it('does not invent prior combination for a legacy frame', () => {
    const old = {
      ...pocket(),
      sourceCombine: 'difference' as const,
      frameShape: 'rounded_rectangle' as const,
    };
    expect({ ...old, ...pocketFramePatch(0, false) }).toMatchObject({
      sourceCombine: 'difference',
      sourceLayers: ['part'],
      sourceObjects: [3, 8],
    });
  });
  it('places all nine XY points without moving stock, legacy Z or WCS', () => {
    const b = { minX: -20, minY: 10, maxX: 40, maxY: 90 };
    for (const x of [0, 0.5, 1])
      for (const y of [0, 0.5, 1]) {
        const target = {
          workOffset: { x_mm: 8, y_mm: 9, z_mm: 12, wcs: 'G55' },
          stock: { offsetZ: 15 },
          dirty: false,
        };
        const cmd = setWorkOffsetCommand(workZeroPoint(b, x, y));
        cmd.apply(target);
        expect(target.workOffset).toEqual({
          x_mm: -20 + 60 * x,
          y_mm: 10 + 80 * y,
          z_mm: 12,
          wcs: 'G55',
        });
        expect(target.stock).toEqual({ offsetZ: 15 });
        cmd.revert(target);
        expect(target.workOffset.x_mm).toBe(8);
      }
  });
  it('requires a real compatible text tool but allows geometry-only text', () => {
    const tools = [
      { id: 1, kind: 'endmill' },
      { id: 2, kind: 'v_bit' },
    ];
    expect(compatibleTextTool([], 1, 'v_bit')).toBe(false);
    expect(compatibleTextTool(tools, 1, 'v_bit')).toBe(false);
    expect(compatibleTextTool(tools, 2, 'v_bit')).toBe(true);
    expect(compatibleTextTool([], 0, null)).toBe(true);
  });
  it('material cancellation invalidates late requests and explicit application is undoable', async () => {
    const requests = new PreviewRequests();
    const old = { id: 'old', revision: '1' };
    const target = { material: old, dirty: false };
    const draft = { id: 'new', revision: '2' };
    const ticket = requests.start();
    const late = Promise.resolve(ticket);
    requests.cancel();
    expect(requests.owns(await late)).toBe(false);
    expect(target.material).toEqual(old);
    const cmd = setMaterialCommand(draft);
    cmd.apply(target);
    expect(target.material).toEqual(draft);
    cmd.revert(target);
    expect(target.material).toEqual(old);
  });
  it('reports actual segment endpoint minima, scoped to the operation, excluding rapids', () => {
    const path = [
      { op_id: 7, kind: 'cut', from: { z: -1 }, to: { z: -3.125 } },
      { op_id: 7, kind: 'rapid', from: { z: -99 }, to: { z: -99 } },
      { op_id: 8, kind: 'cut', from: { z: -12 }, to: { z: -20 } },
    ] as ToolpathSegment[];
    expect(generatedMinimumZ(path, 7)).toBe(-3.125);
    expect(generatedMinimumZ(path, 9)).toBeNull();
  });
});

it('retains critical simulation alerts in the workshop report', () => {
  const result = workshopWarnings(
    [],
    [
      {
        kind: 'rapid_through_material',
        segment_idx: 2,
        worst_x: 3,
        worst_y: 4,
        rapid_pz: 0,
        subkind: 'tip',
        worst_cell_z: 1,
      },
    ],
  );
  expect(result.bad).toBe(1);
  expect(result.top[0].message).toContain('Rapid through material');
});
