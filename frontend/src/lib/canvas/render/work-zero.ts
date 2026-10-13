import type { ProjectFn } from './types';
export function drawWorkZero(
  ctx: CanvasRenderingContext2D,
  project: ProjectFn,
  origin: { x_mm: number; y_mm: number; wcs: string },
  accent: string,
  label: string,
) {
  const [x, y] = project(origin.x_mm, origin.y_mm);
  ctx.save();
  ctx.strokeStyle = accent;
  ctx.fillStyle = accent;
  ctx.lineWidth = 1.5;
  ctx.beginPath();
  ctx.arc(x, y, 6, 0, Math.PI * 2);
  ctx.moveTo(x - 10, y);
  ctx.lineTo(x + 10, y);
  ctx.moveTo(x, y - 10);
  ctx.lineTo(x, y + 10);
  ctx.stroke();
  ctx.font = '11px sans-serif';
  ctx.fillText(`${label} · ${origin.wcs}`, x + 12, y - 10);
  ctx.restore();
}
