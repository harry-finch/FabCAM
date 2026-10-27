import { computeFootprint } from '../sim/footprint';
import type { ProjectState } from './project.svelte';

/// Opening a drawing replaces document contents, not the configured blank/zero.
/// Keep the existing document boundary (operations/history cleared), compensating
/// only the legacy bbox-relative XY stock storage as the old drawing disappears.
export function clearForDrawingImport(p: ProjectState): void {
  const stock = { ...p.data.stock };
  const zero = { ...p.data.workOffset };
  const before = computeFootprint(p.stockSizingImport, stock, p.data.machine.workArea);
  p.clearProject();
  p.data.stock = stock;
  p.data.workOffset = zero;
  if (stock.mode === 'manual') {
    const after = computeFootprint(p.stockSizingImport, stock, p.data.machine.workArea);
    p.data.stock = {
      ...stock,
      offsetX: (stock.offsetX ?? 0) + before.minX - after.minX,
      offsetY: (stock.offsetY ?? 0) + before.minY - after.minY,
    };
  }
}

// Edit the effective minimum corner while preserving existing auto/manual
// bbox-relative storage. No reinterpretation/migration of old projects.
export function stockCornerPatch(
  axis: 'x' | 'y',
  value: number,
  footprint: { minX: number; minY: number },
  stock: { offsetX?: number; offsetY?: number },
) {
  return axis === 'x'
    ? { offsetX: (stock.offsetX ?? 0) + value - footprint.minX }
    : { offsetY: (stock.offsetY ?? 0) + value - footprint.minY };
}
