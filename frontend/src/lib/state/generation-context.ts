import { reliefError, reliefOperationError } from './corner-relief';
import { buildProject } from '../api/build-project';
import { toCpsPostRequest } from '../components/cps-post-form';
import type { ProjectState } from './project.svelte';

// Runtime-only identity of the actual CAM inputs. Saving a project clears its
// file dirty flag, but cannot make an old program current. View state is excluded.
export function generationContextKey(p: ProjectState, post: string): string {
  return JSON.stringify({
    post,
    cornerReliefError: cornerReliefGenerationError(p),
    cps: post === 'cps' ? toCpsPostRequest(p.data.machine.cpsPost) : undefined,
    project: buildGenerationProject(p),
  });
}

export function buildGenerationProject(p: ProjectState) {
  return buildProject({
    material: p.data.material,
    fsContext: p.data.fsContext,
    transformedImport: p.transformedImport,
    geometryView: p.geometryView,
    stockSizingImport: p.stockSizingImport,
    machine: p.data.machine,
    tools: p.data.tools,
    operations: p.data.operations,
    fixtures: p.data.fixtures,
    textLayers: p.data.textLayers,
    workOffset: p.data.workOffset,
    stock: p.data.stock,
    reliefSources: p.data.reliefSources,
    groupOpsByTool: p.data.groupOpsByTool,
  });
}

export function cornerReliefGenerationError(p: ProjectState): string | null {
  if (p.data.cornerRelief && (p.data.machine.unit !== 'mm' || p.data.stock.flip))
    return 'Corner relief requires metric single-sided milling.';
  return (
    reliefError(p.data.cornerRelief, p.data.imports, p.data.tools) ||
    reliefOperationError(p.data.cornerRelief, p.data.operations)
  );
}

export function checkCornerRelief(p: ProjectState): boolean {
  const problem = cornerReliefGenerationError(p);
  if (problem) p.setError(problem);
  return !problem;
}
