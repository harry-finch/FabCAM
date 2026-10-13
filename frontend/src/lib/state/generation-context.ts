import { buildProject } from '../api/build-project';
import { toCpsPostRequest } from '../components/cps-post-form';
import type { ProjectState } from './project.svelte';

// Runtime-only identity of the actual CAM inputs. Saving a project clears its
// file dirty flag, but cannot make an old program current. View state is excluded.
export function generationContextKey(p: ProjectState, post: string): string {
  return JSON.stringify({
    post,
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
