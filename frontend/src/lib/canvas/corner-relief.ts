import { cornerReliefUi, cornerKey, toggleCorner } from '../state/corner-relief-ui.svelte';
import type { BBox, CornerRef } from '../api/types';
import type { ProjectFn } from './render/types';
import { drawSegment } from './render/segment';
export type ReliefHit = { x: number; y: number; reference: CornerRef; disabled: boolean };
export function drawCornerRelief(
  ctx: CanvasRenderingContext2D,
  project2: ProjectFn,
  scale: number,
  themeVar: (name: string) => string,
  accent: string,
): ReliefHit[] {
  const reliefHits: ReliefHit[] = [];
  if (cornerReliefUi.editing && cornerReliefUi.response) {
    ctx.save();
    ctx.strokeStyle = themeVar('--warn');
    ctx.lineWidth = 2;
    if (!cornerReliefUi.response.error)
      for (const seg of cornerReliefUi.response.segments) drawSegment(ctx, seg, project2);
    for (const candidate of cornerReliefUi.response.candidates) {
      const xy = project2(candidate.vertex.x, candidate.vertex.y);
      const selected = cornerReliefUi.corners.some(
        (c) => cornerKey(c) === cornerKey(candidate.reference),
      );
      reliefHits.push({
        x: xy[0],
        y: xy[1],
        reference: candidate.reference,
        disabled: !!candidate.reason,
      });
      ctx.beginPath();
      ctx.arc(xy[0], xy[1], selected ? 6 : 4, 0, 2 * Math.PI);
      ctx.strokeStyle = candidate.reason ? themeVar('--text-muted') : accent;
      ctx.fillStyle = selected ? accent : themeVar('--surface');
      ctx.fill();
      ctx.stroke();
      if (selected && candidate.center) {
        const center = project2(candidate.center.x, candidate.center.y);
        ctx.setLineDash([3, 3]);
        ctx.beginPath();
        ctx.arc(center[0], center[1], (cornerReliefUi.diameter * scale) / 2, 0, 2 * Math.PI);
        ctx.stroke();
        ctx.setLineDash([]);
      }
    }
    ctx.restore();
  }

  return reliefHits;
}
export function reliefFitBounds(b: BBox): BBox {
  if (!cornerReliefUi.editing || !cornerReliefUi.response) return b;
  const r = cornerReliefUi.diameter / 2;
  const out = { ...b };
  for (const c of cornerReliefUi.response.candidates) {
    if (!c.center) continue;
    out.min_x = Math.min(out.min_x, c.center.x - r);
    out.max_x = Math.max(out.max_x, c.center.x + r);
    out.min_y = Math.min(out.min_y, c.center.y - r);
    out.max_y = Math.max(out.max_y, c.center.y + r);
  }
  return out;
}

export function reliefDisplay<T>(fallback: T): T | import('../api/types').ImportResponse {
  return cornerReliefUi.editing && cornerReliefUi.original ? cornerReliefUi.original : fallback;
}

export function clickCorner(hits: ReliefHit[], x: number, y: number, button: number): boolean {
  if (!cornerReliefUi.editing || button !== 0) return false;
  const hit = hits.find((h) => Math.hypot(h.x - x, h.y - y) <= 9);
  if (hit && !hit.disabled) toggleCorner(hit.reference);
  return true;
}
