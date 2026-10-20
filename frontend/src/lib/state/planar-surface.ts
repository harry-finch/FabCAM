import type { PlanarSurfaceOp } from './op_types';
import type { ImportResponse } from '../api/types';

export function surfaceBounds(op: PlanarSurfaceOp, diameter = 0) {
  const p = op.surface;
  const [x, y] = p.origin;
  const m = p.margin_mm;
  const target = [x - m, y - m, x + p.width_mm + m, y + p.height_mm + m];
  if (
    ![...target, diameter, p.overtravel_mm, p.stepover_mm].every(Number.isFinite) ||
    p.width_mm <= 0 ||
    p.height_mm <= 0 ||
    m < 0 ||
    p.overtravel_mm < 0 ||
    diameter < 0 ||
    p.stepover_mm <= 0
  )
    return null;
  const center = [...target];
  const extension = diameter / 2 + p.overtravel_mm;
  const axis = p.direction === 'along_x' ? 0 : 1;
  center[axis] -= extension;
  center[axis + 2] += extension;
  return { target, center, cutter: center.map((v, i) => v + ((i < 2 ? -1 : 1) * diameter) / 2) };
}

/** Empty geometry carrying an authored zone bbox. Never introduces source chains. */
export function surfaceSizingView(
  operations: readonly PlanarSurfaceOp[],
  base: ImportResponse | null = null,
): ImportResponse | null {
  const boxes = operations
    .filter((op) => op.enabled)
    .map((op) => surfaceBounds(op)?.target)
    .filter((b): b is number[] => !!b);
  if (!boxes.length) return base;
  if (base) boxes.push([base.bbox.min_x, base.bbox.min_y, base.bbox.max_x, base.bbox.max_y]);
  return {
    ...(base ?? {}),
    filename: base?.filename ?? 'surfacing',
    format: base?.format ?? 'authored',
    unit_scale: base?.unit_scale ?? 1,
    warnings: base?.warnings ?? [],
    unit_status: base?.unit_status ?? 'unknown',
    unit_source: base?.unit_source ?? '',
    layers: base?.layers ?? [],
    segments: base?.segments ?? [],
    objects: base?.objects ?? [],
    object_meta: base?.object_meta ?? [],
    bbox: {
      min_x: Math.min(...boxes.map((b) => b[0])),
      min_y: Math.min(...boxes.map((b) => b[1])),
      max_x: Math.max(...boxes.map((b) => b[2])),
      max_y: Math.max(...boxes.map((b) => b[3])),
    },
  };
}
