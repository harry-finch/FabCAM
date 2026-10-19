import type { ProjectFn } from './types';
export function drawWorkZero(
  ctx: CanvasRenderingContext2D,
  project: ProjectFn,
  origin: { x_mm: number; y_mm: number; wcs: string },
  accent: string,
  label: string,
  background: string,
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
  ctx.font = 'bold 12px sans-serif';
  const text = `${label} · ${origin.wcs}`;
  const width = ctx.measureText(text).width;
  const lx = Math.max(
    4,
    Math.min(x + 12, (ctx.canvas.clientWidth || ctx.canvas.width) - width - 4),
  );
  const ly = Math.max(14, y - 12);
  ctx.strokeStyle = background;
  ctx.lineWidth = 4;
  ctx.strokeText(text, lx, ly);
  ctx.fillText(text, lx, ly);
  ctx.restore();
}
