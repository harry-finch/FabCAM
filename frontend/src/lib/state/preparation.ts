import type { PocketOp } from './op_types';

/** Frame synthesis already enforces Difference in the core. Never overwrite source combination. */
export function pocketFramePatch(diameter: number, outside: boolean): Partial<PocketOp> {
  if (outside)
    return {
      frameShape: 'rectangle',
      framePaddingMm: 3 * diameter,
    };
  return {
    frameShape: undefined,
    framePaddingMm: undefined,
    frameCornerRadiusMm: undefined,
  };
}

/** XY only: callers merge this patch, retaining Z and WCS, including legacy values. */
export function workZeroPoint(
  b: { minX: number; minY: number; maxX: number; maxY: number },
  x: number,
  y: number,
) {
  return { x_mm: b.minX + (b.maxX - b.minX) * x, y_mm: b.minY + (b.maxY - b.minY) * y };
}

export function compatibleTextTool(
  tools: { id: number; kind: string }[],
  id: number,
  kind: string | null | undefined,
) {
  return !kind || tools.some((tool) => tool.id === id && tool.kind === kind);
}
