import type { GenerateResponse } from '../api/types';
import type { WorkOffset } from './project-types';

// Programs and core diagnostics use work coordinates. The drawing workspace,
// stock and fixtures retain authored drawing coordinates. Re-express the same
// emitted path for rendering/simulation; never change the program text.
export function inDrawingFrame<
  T extends Pick<GenerateResponse, 'toolpath' | 'regions' | 'remaining_material'>,
>(r: T, origin: WorkOffset): T {
  const { x_mm: x, y_mm: y } = origin;
  if (!x && !y) return r;
  const point = <T extends { x: number; y: number }>(p: T): T => ({ ...p, x: p.x + x, y: p.y + y });
  return {
    ...r,
    toolpath: r.toolpath.map((s) => ({
      ...s,
      from: point(s.from),
      to: point(s.to),
      ...(s.arc ? { arc: { ...s.arc, cx: s.arc.cx + x, cy: s.arc.cy + y } } : {}),
    })),
    remaining_material: r.remaining_material?.map((analysis) => ({
      ...analysis,
      regions: analysis.regions.map((region) => ({
        ...region,
        outer: region.outer.map(point),
        holes: region.holes?.map((ring) => ring.map(point)),
      })),
    })),
    regions: r.regions?.map((region) => ({
      ...region,
      outer: region.outer.map(point),
      holes: region.holes?.map((ring) => ring.map(point)),
    })),
  };
}

export function supportedWcs(post: string): string[] {
  return post === 'linuxcnc' ? ['G54', 'G55', 'G56', 'G57', 'G58', 'G59'] : ['G54'];
}
