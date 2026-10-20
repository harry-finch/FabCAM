import { CancelledError, type WiacClient } from '../api/client';
import type { DrawingResponse, ImportResponse } from '../api/types';
import type { components } from '../api/generated';
import type { ImportEntry } from './project-types';
import type { OpEntry } from './op_types';
import type { Command } from './history';

export type GeometryBinding = components['schemas']['Binding'];
export interface DrawingState {
  version: 1;
  originalBindings: Pick<OpEntry, 'id' | 'sourceObjects' | 'sourceLayers' | 'geometryBinding'>[];
  inputKey: string;
  response: DrawingResponse;
}
export const drawingInputKey = (imports: readonly ImportEntry[]) =>
  JSON.stringify(imports.map(({ id, source, fileTransform }) => ({ id, source, fileTransform })));

// Adapter only: coordinates and contour membership come from Rust. UI object
// numbers are transient display handles and never persisted as CAM authority.
export function drawingImport(
  state: DrawingState,
  imports: readonly ImportEntry[],
): ImportResponse {
  const e = state.response.evaluated;
  const key = (id: components['schemas']['Identity']) => `${id.import}/${id.entity}`;
  const ids = new Map(e.contours.map((id, i) => [key(id), i + 1]));
  const layers = new Map<string, { name: string; color: number; segment_count: number }>();
  for (const s of e.segments) {
    const row = layers.get(s.layer) ?? { name: s.layer, color: s.color, segment_count: 0 };
    row.segment_count++;
    layers.set(s.layer, row);
  }
  return {
    filename: imports.map((i) => i.source.filename).join(' + '),
    format: 'edited',
    unit_scale: 1,
    warnings: [],
    unit_status: 'unknown',
    unit_source: '',
    bbox: e.bbox,
    segments: e.segments,
    objects: e.membership.map((id) => ids.get(key(id)) ?? 0),
    object_meta: e.contours.map((id, i) => ({
      id: i + 1,
      closed: e.closed[i],
      layer: e.segments.find((_, j) => key(e.membership[j]) === key(id))?.layer ?? '',
      color: 7,
      bbox: e.contour_bounds[i],
    })),
    layers: [...layers.values()],
  };
}
export function initialBinding(op: OpEntry): GeometryBinding {
  if (op.sourceObjects?.length)
    return {
      kind: 'unresolved',
      previous: [],
      reason: 'Legacy contour selection requires explicit review for edited geometry',
    };
  return op.sourceLayers?.length
    ? { kind: 'layers', names: [...op.sourceLayers] }
    : { kind: 'all' };
}
export function selectedBinding(op: OpEntry, drawing: DrawingState): GeometryBinding {
  if (op.sourceObjects?.length) {
    const ids = op.sourceObjects.map((n) => drawing.response.evaluated.contours[n - 1]);
    return ids.every(Boolean)
      ? { kind: 'contours', ids }
      : { kind: 'unresolved', previous: [], reason: 'Selected geometry is missing' };
  }
  return initialBinding(op);
}
export function drawingCommand(
  before: DrawingState | undefined,
  after: DrawingState | undefined,
  oldOps: OpEntry[],
  newOps: OpEntry[],
): Command {
  const copy = <T>(v: T): T => (v === undefined ? v : (JSON.parse(JSON.stringify(v)) as T));
  const previous = copy(before),
    next = copy(after),
    previousOps = copy(oldOps),
    nextOps = copy(newOps);
  return {
    label: after ? 'Apply drawing repairs' : 'Reset drawing repairs',
    apply(state) {
      Object.assign(state as object, { drawing: copy(next), operations: copy(nextOps) });
    },
    revert(state) {
      Object.assign(state as object, { drawing: copy(previous), operations: copy(previousOps) });
    },
  };
}

export function provenLegacyBinding(op: OpEntry, response: DrawingResponse): GeometryBinding {
  if (!op.sourceObjects?.length) return initialBinding(op);
  const ids = op.sourceObjects.map((id) => response.legacy_contours[id - 1]);
  if (ids.every((id) => id != null))
    return { kind: 'contours', ids: ids as components['schemas']['Identity'][] };
  return initialBinding(op);
}

export async function generateEditedDrawing(
  p: import('./project.svelte').ProjectState,
  client: WiacClient,
  machining: import('../api/types').GenerateRequest,
  signal: AbortSignal,
) {
  if (!client.generateDrawing || !p.data.drawing)
    throw new Error('Backend does not support GEOM1. No legacy fallback is allowed.');
  const response = await client.generateDrawing({
    version: 1,
    drawing: p.data.drawing.response.drawing,
    bindings: p.data.operations.map((op) => ({
      operation: op.id,
      binding: op.geometryBinding ?? initialBinding(op),
    })),
    machining,
  });
  if (signal.aborted) throw new CancelledError();
  return response;
}

/** Reserve original operation IDs while their reset bindings exist. */
export function nextDrawingOperationId(operations: readonly OpEntry[], drawing?: DrawingState) {
  return (
    [...operations, ...(drawing?.originalBindings ?? [])].reduce(
      (max, op) => Math.max(max, op.id),
      0,
    ) + 1
  );
}

export function resetDrawingOperations(
  drawing: DrawingState,
  imports: readonly ImportEntry[],
  operations: readonly OpEntry[],
): OpEntry[] {
  if (drawing.inputKey !== drawingInputKey(imports))
    throw new Error(
      'Undo source/transform changes before resetting repairs. Original contour selections cannot be safely restored onto changed imports.',
    );
  return operations.map((op) => {
    const old = drawing.originalBindings.find((b) => b.id === op.id);
    if (old)
      return {
        ...op,
        sourceObjects: old.sourceObjects,
        sourceLayers: old.sourceLayers,
        geometryBinding: old.geometryBinding,
      };
    if (
      op.sourceObjects?.length ||
      op.geometryBinding?.kind === 'contours' ||
      op.geometryBinding?.kind === 'unresolved'
    )
      throw new Error(
        'An operation created after cleanup has an explicit contour selection. Choose its intended layers before resetting repairs.',
      );
    return { ...op, geometryBinding: undefined };
  });
}
