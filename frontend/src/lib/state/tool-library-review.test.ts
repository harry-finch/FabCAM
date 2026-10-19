import { describe, it, expect } from 'vitest';
import { changeToolKind, toolLibraryImpacts } from './tool-library-review';
import { defaultToolForMode } from './tool_mode_defaults';
import { syncStockedFromInventory, stockTool } from './tool_inventory';

describe('UX6-B library review contracts', () => {
  it('preserves hidden geometry and cutting defaults across type changes without inventing dimensions', () => {
    const tool = {
      ...defaultToolForMode('mill', 3),
      name: 'Authored tool',
      cornerRadiusMm: 0.7,
      speed: 13450,
      feedRate: 745,
      plungeRate: 180,
      holder: { kind: 'cylinder' as const, diameter_mm: 20, length_mm: 30 },
    };
    const v = changeToolKind(tool, 'v_bit');
    expect(v).toEqual({ ...tool, kind: 'v_bit' });
    expect(v.tipAngleDeg).toBeUndefined();
    expect(changeToolKind(v, 'endmill')).toEqual(tool);
    expect(tool.kind).toBe('endmill');
  });
  it('reviews changed same-ID copies and operation names, not unrelated/local IDs', () => {
    const original = defaultToolForMode('mill', 1);
    const local = { ...original, id: 8, name: 'Local copy' };
    const changed = { ...original, diameter: 6, name: 'New definition' };
    const ops = [
      { name: 'Manual contour', toolId: 1 },
      { name: 'Local pocket', toolId: 8 },
    ];
    expect(toolLibraryImpacts([changed], [original, local], ops)).toEqual([
      {
        id: 1,
        before: original.name,
        after: 'New definition',
        kindBefore: original.kind,
        kindAfter: changed.kind,
        diameterBefore: original.diameter,
        diameterAfter: changed.diameter,
        operations: ['Manual contour'],
      },
    ]);
    expect(syncStockedFromInventory([changed], [original, local])).toEqual([changed, local]);
    expect(ops).toEqual([
      { name: 'Manual contour', toolId: 1 },
      { name: 'Local pocket', toolId: 8 },
    ]);
  });
  it('library deletion retains stocked copies while explicit add collisions get a new ID', () => {
    const original = defaultToolForMode('mill', 1);
    expect(toolLibraryImpacts([], [original], [{ name: 'Used', toolId: 1 }])).toEqual([]);
    expect(syncStockedFromInventory([], [original])).toBeNull();
    const different = { ...original, diameter: 6 };
    expect(stockTool(different, [original])?.id).toBe(2);
    expect(toolLibraryImpacts([different], [original], [])[0]?.id).toBe(1);
  });
  it('detects wear/holder changes and matches existing serialization-sensitive sync', () => {
    const original = defaultToolForMode('mill', 1);
    const reordered = Object.fromEntries(Object.entries(original).reverse()) as typeof original;
    expect(toolLibraryImpacts([reordered], [original], [])).toHaveLength(1);
    expect(syncStockedFromInventory([reordered], [original])).not.toBeNull();
    expect(toolLibraryImpacts([{ ...original, wearOffsetMm: 0.05 }], [original], [])).toHaveLength(
      1,
    );
    expect(
      toolLibraryImpacts(
        [{ ...original, holder: { kind: 'cylinder', diameter_mm: 20, length_mm: 30 } }],
        [original],
        [],
      ),
    ).toHaveLength(1);
  });
});
