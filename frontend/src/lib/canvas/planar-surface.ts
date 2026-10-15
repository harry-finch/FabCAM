import { project } from '../state/project.svelte';
import { surfaceBounds } from '../state/planar-surface';
import { effectiveDiameterMm } from '../state/tool_wear';
import type { ProjectFn } from './render/types';

export function drawPlanarSurface(
  ctx: CanvasRenderingContext2D,
  project2: ProjectFn,
  accent: string,
  warning: string,
) {
  const op = project.data.operations.find((op) => op.id === project.sel.selectedOpId);
  if (op?.kind !== 'planar_surface' || !op.enabled) return;
  const tool = project.data.tools.find((tool) => tool.id === op.toolId);
  const bounds = surfaceBounds(op, tool ? effectiveDiameterMm(tool) : 0);
  if (!bounds) return;
  ctx.save();
  ctx.lineWidth = 1.5;
  for (const [i, b] of [bounds.target, bounds.cutter].entries()) {
    const a = project2(b[0], b[1]);
    const z = project2(b[2], b[3]);
    ctx.strokeStyle = i === 0 ? accent : warning;
    ctx.setLineDash(i === 0 ? [] : [4, 4]);
    ctx.strokeRect(a[0], a[1], z[0] - a[0], z[1] - a[1]);
  }
  ctx.restore();
}
