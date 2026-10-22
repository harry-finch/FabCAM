import { drawingInputKey } from './drawing';
import { reliefError, reliefOperationError } from './corner-relief';
import { buildProject } from '../api/build-project';
import { toCpsPostRequest } from '../components/cps-post-form';
import type { ProjectState } from './project.svelte';

// Runtime-only identity of the actual CAM inputs. Saving a project clears its
// file dirty flag, but cannot make an old program current. View state is excluded.
export function generationContextKey(p: ProjectState, post: string): string {
  return JSON.stringify({
    post,
    drawing: p.data.drawing,
    drawingBindings: p.data.operations.map((op) => op.geometryBinding),
    cornerReliefError: cornerReliefGenerationError(p),
    cps: post === 'cps' ? toCpsPostRequest(p.data.machine.cpsPost) : undefined,
    project: buildGenerationProject(p),
  });
}

export function buildGenerationProject(p: ProjectState, allowEmpty = false) {
  return buildProject(
    {
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
    },
    allowEmpty,
  );
}

export function cornerReliefGenerationError(p: ProjectState): string | null {
  const unresolvedBinding = p.data.operations.find(
    (op) => op.enabled && op.geometryBinding?.kind === 'unresolved',
  );
  if (unresolvedBinding)
    return `${unresolvedBinding.name}: select geometry again after drawing repair.`;
  if (
    !p.data.drawing &&
    p.data.operations.some((op) => op.enabled && op.geometryBinding?.kind === 'contours')
  )
    return 'Edited contour identity has no drawing snapshot — reselect geometry before generation.';
  if (p.data.drawing) {
    if (p.sourceFileStaleNotice)
      return 'External drawing changed — review the source notice before generating the accepted snapshot.';
    if (p.data.drawing.inputKey !== drawingInputKey(p.data.imports))
      return 'Drawing source or transform changed — review repairs before generation.';
    if (
      p.data.operations.some(
        (op) =>
          op.enabled &&
          ((op.geometryBinding?.kind === 'layers' &&
            op.geometryBinding.names.includes('__stock_outline')) ||
            (op.geometryBinding?.kind === 'all' && p.data.stock.visible)),
      )
    )
      return 'For repaired drawings, select imported contours or named layers instead of the synthetic stock outline.';
    if (p.data.cornerRelief || p.data.stock.flip)
      return 'Edited drawing with relief/two-sided setup is not yet qualified.';
    const unresolved = p.data.operations.find(
      (op) => op.enabled && op.geometryBinding?.kind === 'unresolved',
    );
    if (unresolved) return `${unresolved.name}: select geometry again after drawing repair.`;
  }
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

/** Canonical dialect identity shared by desktop and narrow-screen diagnostics. */
export function coerceGenerationPost(value: string): 'linuxcnc' | 'grbl' | 'hpgl' | 'cps' {
  return value === 'grbl' || value === 'hpgl' || value === 'cps' ? value : 'linuxcnc';
}
