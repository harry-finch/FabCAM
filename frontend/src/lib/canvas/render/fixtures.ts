import type { Fixture } from '../../state/project-types';
import { unpackFixtureColor } from '../fixture-color';
import type { ProjectFn } from './types';

/// Paint each fixture as a translucent filled outline in its declared
/// color. Selected fixture gets a thicker accent stroke so it's
/// obvious which one the sidebar is editing.
export function drawFixtures(
  ctx: CanvasRenderingContext2D,
  p: ProjectFn,
  fixtures: readonly Fixture[],
  selectedFixtureId: number | null,
  accent: string,
  collisions: ReadonlySet<number> = new Set(),
  danger = accent,
) {
  if (!fixtures || fixtures.length === 0) return;
  const ordered = [
    ...fixtures.filter((f) => f.id !== selectedFixtureId),
    ...fixtures.filter((f) => f.id === selectedFixtureId),
  ];
  for (const f of ordered) {
    const { r, g, b, a } = unpackFixtureColor(f.color);
    const fill = `rgba(${r}, ${g}, ${b}, ${Math.max(0.15, (a / 255) * 0.5)})`;
    const stroke = `rgb(${r}, ${g}, ${b})`;
    const isSel = selectedFixtureId === f.id;
    ctx.fillStyle = fill;
    ctx.strokeStyle = collisions.has(f.id) ? danger : isSel ? accent : stroke;
    ctx.setLineDash(collisions.has(f.id) && isSel ? [5, 2] : []);
    ctx.lineWidth = isSel ? 2.4 : 1.4;
    const [ox, oy] = f.origin;
    if (f.kind.shape === 'box') {
      const hw = f.kind.width / 2;
      const hd = f.kind.depth / 2;
      const [x0, y0] = p(ox - hw, oy - hd);
      const [x1, y1] = p(ox + hw, oy + hd);
      const xMin = Math.min(x0, x1);
      const yMin = Math.min(y0, y1);
      const w = Math.abs(x1 - x0);
      const h = Math.abs(y1 - y0);
      ctx.fillRect(xMin, yMin, w, h);
      ctx.strokeRect(xMin, yMin, w, h);
    } else if (f.kind.shape === 'cylinder') {
      const [cx, cy] = p(ox, oy);
      const [edgeX] = p(ox + f.kind.radius, oy);
      const rPx = Math.abs(edgeX - cx);
      ctx.beginPath();
      ctx.arc(cx, cy, rPx, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();
    } else if (f.kind.shape === 'polygon') {
      if (f.kind.vertices.length < 2) continue;
      ctx.beginPath();
      const [vx0, vy0] = p(ox + f.kind.vertices[0][0], oy + f.kind.vertices[0][1]);
      ctx.moveTo(vx0, vy0);
      for (let i = 1; i < f.kind.vertices.length; i++) {
        const [vx, vy] = p(ox + f.kind.vertices[i][0], oy + f.kind.vertices[i][1]);
        ctx.lineTo(vx, vy);
      }
      ctx.closePath();
      ctx.fill();
      ctx.stroke();
    }
  }
  ctx.setLineDash([]);
}
