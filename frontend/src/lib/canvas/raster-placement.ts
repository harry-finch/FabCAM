import type { OpEntry } from '../state/op_types';
import type { ReliefSource } from '../state/project-types';

/** Distinct raster sources paired with their first enabled operation. */
export function rasterSources(operations: readonly OpEntry[], sources: readonly ReliefSource[]) {
  const out: { op: OpEntry; src: ReliefSource }[] = [];
  const seen = new Set<number>();
  for (const op of operations) {
    if (op.kind !== 'raster_engrave' || !op.enabled) continue;
    const src = sources.find((s) => s.id === op.sourceId);
    if (!src || src.cols <= 0 || src.rows <= 0 || seen.has(src.id)) continue;
    seen.add(src.id);
    out.push({ op, src });
  }
  return out;
}

/** Prefer the selected image, then the topmost, without mutating cached order. */
export function rasterAt(
  placements: ReturnType<typeof rasterSources>,
  selected: number | null,
  x: number,
  y: number,
) {
  const ordered = [...placements].sort(
    (a, b) => (a.op.id === selected ? 1 : 0) - (b.op.id === selected ? 1 : 0),
  );
  for (let i = ordered.length - 1; i >= 0; i--) {
    const { src } = ordered[i];
    if (
      x >= src.origin.x &&
      x <= src.origin.x + src.cols * src.cell &&
      y >= src.origin.y &&
      y <= src.origin.y + src.rows * src.cell
    )
      return ordered[i];
  }
  return null;
}
