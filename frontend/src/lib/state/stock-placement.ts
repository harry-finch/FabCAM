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
