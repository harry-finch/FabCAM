import { cuttingTargetKey } from './cutting-workflow';
import { updateOperationCommand } from './commands';
import type { Command } from './history';
import type { ProjectState } from './project.svelte';
import {
  listCollections,
  type PresetCollection,
  type StoredCollection,
} from './preset-collections';
import { validateStages } from './cutting-stages';
import {
  REUSE_KEY,
  stageValues,
  operationAnchor,
  targetReuseContext,
  writableReuseRecords,
  reuseFields,
  validReuseNumber,
  type ReuseField,
  type ReuseStage,
  type ReuseContext,
  type ReuseTarget,
  type ReuseRecord,
} from './preset-reuse-metadata';
export { REUSE_KEY, reuseRecords, reuseState } from './preset-reuse-metadata';
const clone = <T>(v: T): T => JSON.parse(JSON.stringify(v));
const reviewKey = (s: ReuseTarget, id: number) =>
  JSON.stringify([cuttingTargetKey(s, id), s.serializedExtensions]);
async function hash(s: string) {
  return Array.from(
    new Uint8Array(await crypto.subtle.digest('SHA-256', new TextEncoder().encode(s))),
    (n) => n.toString(16).padStart(2, '0'),
  ).join('');
}
export interface ReuseReview {
  id: number;
  epoch: number;
  key: string;
  sourceJson: string;
  token: string;
  sourceStage: ReuseStage;
  targetStage: ReuseStage;
  collectionId: string;
  presetId: string;
  fields: ReuseField[];
  before: Partial<Record<ReuseField, number | null>>;
  record: ReuseRecord;
}
function sourceContext(c: PresetCollection, id: string, stage: ReuseStage): ReuseContext {
  const p = c.presets.find((x) => x.id === id)!;
  if (p.context.signature.length > 65536) throw new Error('reuse.error.source');
  const s = JSON.parse(p.context.signature);
  if (!['profile', 'pocket'].includes(s?.operation?.type)) throw new Error('reuse.error.source');
  const tool = s.tools?.find((t: { role: string }) => t.role === stage);
  const strategy =
    s.operation.type === 'pocket'
      ? (s.operation.strategy ?? s.operation.pocket?.strategy)
      : s.operation.offset;
  return {
    operation: s.operation.type,
    stage,
    machine_id: p.context.machine_id,
    machine_label: p.context.machine_label,
    material_id: p.context.material_id,
    material_label: p.context.material_label,
    material_revision: s.material?.revision ?? null,
    tool_id:
      stage === 'main'
        ? p.parameters.stages!.main.tool_id
        : p.parameters.stages!.finishing!.cutting.tool_id,
    tool_physical_id: tool?.physical_id ?? null,
    tool_kind: tool?.geometry?.kind ?? null,
    diameter_mm: tool?.geometry?.diameter ?? null,
    strategy: typeof strategy === 'string' ? strategy : (strategy?.kind ?? null),
    surface_scope:
      stage === 'finishing' && p.parameters.stages!.contract === 'pocket_finishing_v2'
        ? (p.parameters.stages!.finishing?.scope ?? null)
        : null,
  };
}
export async function preparePresetReuse(
  p: ProjectState,
  id: number,
  source: { data: PresetCollection; token: string },
  presetId: string,
  sourceStage: ReuseStage,
  targetStage: ReuseStage,
  fields: readonly ReuseField[],
): Promise<ReuseReview> {
  const s = p.data as ReuseTarget;
  writableReuseRecords(s.serializedExtensions);
  const op = s.operations.find((o) => o.id === id);
  if (!op) throw new Error('reuse.error.target');
  const target = stageValues(op, targetStage);
  const collection = clone(source.data),
    preset = collection.presets.find((v) => v.id === presetId);
  if (collection.version !== 2 || !preset?.parameters.stages || !source.token)
    throw new Error('reuse.error.source');
  validateStages(preset.parameters.stages);
  const src =
    sourceStage === 'main'
      ? preset.parameters.stages.main
      : sourceStage === 'finishing'
        ? preset.parameters.stages.finishing?.cutting
        : null;
  if (!src) throw new Error('reuse.error.stage');
  if (
    !fields.length ||
    new Set(fields).size !== fields.length ||
    fields.some((f) => !reuseFields.includes(f))
  )
    throw new Error('reuse.error.fields');
  const values: Partial<Record<ReuseField, number>> = {},
    before: ReuseReview['before'] = {};
  for (const field of fields) {
    const value = src[field];
    if (!validReuseNumber(field, value)) throw new Error('reuse.error.numeric');
    values[field] = value;
    before[field] = target[field];
  }
  const key = reviewKey(s, id),
    epoch = p.documentEpoch,
    sourceJson = JSON.stringify(collection);
  const record: ReuseRecord = {
    operation_id: id,
    operation_anchor: operationAnchor(op),
    stage: targetStage,
    source: {
      collection_id: collection.id,
      collection_name: collection.name,
      preset_id: preset.id,
      preset_name: preset.name,
      revision: source.token,
      content_sha256: await hash(sourceJson),
      context: sourceContext(collection, presetId, sourceStage),
    },
    target_context: targetReuseContext(s, op, targetStage),
    values,
  };
  if (key !== reviewKey(s, id) || epoch !== p.documentEpoch) throw new Error('reuse.error.stale');
  if (!writableReuseRecords({ [REUSE_KEY]: { version: 1, records: [record] } }).length)
    throw new Error('reuse.error.source');
  return {
    id,
    epoch,
    key,
    sourceJson,
    token: source.token,
    sourceStage,
    targetStage,
    collectionId: collection.id,
    presetId,
    fields: [...fields],
    before,
    record,
  };
}
export async function confirmPresetReuse(
  p: ProjectState,
  review: ReuseReview,
  read: () => Promise<StoredCollection[]> = listCollections,
) {
  const s = p.data as ReuseTarget;
  if (p.documentEpoch !== review.epoch || reviewKey(s, review.id) !== review.key)
    throw new Error('reuse.error.stale');
  const rows = await read();
  const source = rows.find((r) => {
    try {
      return JSON.parse(r.json).id === review.collectionId;
    } catch {
      return false;
    }
  });
  if (
    !source ||
    source.token !== review.token ||
    JSON.stringify(JSON.parse(source.json)) !== review.sourceJson
  )
    throw new Error('reuse.error.source_changed');
  const checked = await preparePresetReuse(
    p,
    review.id,
    { data: JSON.parse(source.json), token: source.token },
    review.presetId,
    review.sourceStage,
    review.targetStage,
    review.fields,
  );
  if (
    checked.key !== review.key ||
    checked.epoch !== review.epoch ||
    JSON.stringify(checked.record) !== JSON.stringify(review.record)
  )
    throw new Error('reuse.error.stale');
  const op = s.operations.find((o) => o.id === review.id)!;
  const stages = clone(op.stages!);
  Object.assign(
    review.targetStage === 'main' ? stages.main : stages.finishing!.cutting,
    checked.record.values,
  );
  const edit = updateOperationCommand(review.id, { stages, stagePreset: undefined });
  const before = clone(s.serializedExtensions);
  const records = writableReuseRecords(before).filter(
    (r) => r.operation_id !== review.id || r.stage !== review.targetStage,
  );
  const after = {
    ...before,
    [REUSE_KEY]: { version: 1, records: [...records, clone(checked.record)] },
  };
  writableReuseRecords(after);
  const command: Command = {
    label: 'Reuse preset starting values',
    apply(state) {
      const target = state as ReuseTarget;
      if (reviewKey(target, review.id) !== review.key) throw new Error('reuse.error.stale');
      edit.apply(state);
      target.serializedExtensions = clone(after);
    },
    revert(state) {
      edit.revert(state);
      (state as ReuseTarget).serializedExtensions = clone(before);
    },
  };
  p.history.exec(command, p.target());
}
