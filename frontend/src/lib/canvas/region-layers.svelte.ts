import { RegionPathCache, drawRegions } from './render/regions';
import { remainingView } from '../state/remaining-material';
import { generationContextKey } from '../state/generation-context';
import { workspace } from '../state/workspace.svelte';
import type { ProjectState } from '../state/project.svelte';

export function createRegionLayers(project: ProjectState) {
  const intendedCache = new RegionPathCache();
  const remainingCache = new RegionPathCache();
  const remaining = $derived(
    remainingView(
      project.gen.generated?.remaining_material,
      project.gen.generating ||
        project.gen.inputContextKey !==
          generationContextKey(
            project,
            project.data.machine.gcodeDialect ?? workspace.get().last_post_processor,
          ),
      project.gen.remainingOpId,
    ),
  );
  return {
    get remaining() {
      return remaining;
    },
    draw(
      ctx: CanvasRenderingContext2D,
      scale: number,
      x: number,
      y: number,
      accent: string,
      warning: string,
    ) {
      const intended = project.gen.generated?.regions ?? [];
      if (project.data.regionsVisible && intended.length)
        drawRegions(ctx, intendedCache, intended, scale, x, y, project.sel.selectedOpId, accent);
      if (project.gen.remainingVisible && remaining?.status === 'ready')
        drawRegions(ctx, remainingCache, remaining.regions, scale, x, y, remaining.op_id, warning);
    },
  };
}
