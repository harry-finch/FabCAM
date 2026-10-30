/** Portable advisory data only. Never a CAM input, attestation or restore source. */
import type { OpEntry } from './op_types';
import type { CommandTarget } from './commands';
export const REUSE_KEY = 'fabcamPresetValueReuse';
export const reuseFields = ['rpm', 'feed_mm_min', 'plunge_mm_min', 'depth_per_pass_mm'] as const;
export type ReuseField = (typeof reuseFields)[number];
export type ReuseStage = 'main' | 'finishing';
export type ReuseContext = Record<string, string | number | null>;
export interface ReuseRecord {
  operation_id: number;
  operation_anchor: string;
  stage: ReuseStage;
  source: {
    collection_id: string;
    collection_name: string;
    preset_id: string;
    preset_name: string;
    revision: string;
    content_sha256: string;
    context: ReuseContext;
  };
  target_context: ReuseContext;
  values: Partial<Record<ReuseField, number>>;
}
export type ReuseTarget = CommandTarget & { serializedExtensions: Record<string, unknown> };
export function stageValues(op: OpEntry, stage: ReuseStage) {
  if (!['profile', 'pocket'].includes(op.kind) || !op.stages) throw new Error('reuse.error.target');
  const value =
    stage === 'main' ? op.stages.main : stage === 'finishing' ? op.stages.finishing?.cutting : null;
  if (!value) throw new Error('reuse.error.stage');
  return value;
}
export function validReuseNumber(field: string, n: unknown): n is number {
  return (
    typeof n === 'number' &&
    Number.isFinite(n) &&
    n > 0 &&
    (field === 'depth_per_pass_mm' || (Number.isInteger(n) && n <= 0xffffffff))
  );
}
export function operationAnchor(op: OpEntry) {
  return JSON.stringify([
    op.id,
    op.kind,
    op.name,
    op.sourceObjects,
    op.sourceLayers,
    op.geometryBinding,
  ]);
}
export function targetReuseContext(s: CommandTarget, op: OpEntry, stage: ReuseStage): ReuseContext {
  const v = stageValues(op, stage),
    tool = s.tools.find((t) => t.id === v.tool_id);
  return {
    operation: op.kind,
    stage,
    machine_id: s.machine.physicalId ?? null,
    machine_label: s.machine.name ?? null,
    material_id: s.material?.id ?? null,
    material_label: s.material?.label ?? null,
    material_revision: s.material?.revision ?? null,
    tool_id: v.tool_id,
    tool_physical_id: tool?.physicalId ?? null,
    tool_kind: tool?.kind ?? null,
    diameter_mm: tool?.diameter ?? null,
    strategy:
      op.kind === 'pocket' ? (op.pocketStrategy ?? null) : op.kind === 'profile' ? op.offset : null,
    surface_scope:
      stage === 'finishing' && op.stages?.contract === 'pocket_finishing_v2'
        ? (op.stages.finishing?.scope ?? null)
        : null,
  };
}
function object(v: unknown): v is Record<string, unknown> {
  return !!v && typeof v === 'object' && !Array.isArray(v);
}
const keys = (v: Record<string, unknown>, names: string[]) =>
  Object.keys(v).every((k) => names.includes(k)) && names.every((k) => k in v);
const text = (v: unknown) => typeof v === 'string' && v.length > 0 && v.length <= 2048;
const context = (v: unknown) =>
  object(v) &&
  Object.keys(v).length <= 20 &&
  Object.entries(v).every(
    ([k, x]) =>
      /^[a-z_]+$/.test(k) &&
      (x === null ||
        (typeof x === 'string' && x.length <= 2048) ||
        (typeof x === 'number' && Number.isFinite(x))),
  );
function validRecord(v: unknown): v is ReuseRecord {
  if (
    !object(v) ||
    !keys(v, ['operation_id', 'operation_anchor', 'stage', 'source', 'target_context', 'values']) ||
    !Number.isInteger(v.operation_id) ||
    Number(v.operation_id) <= 0 ||
    !text(v.operation_anchor) ||
    !['main', 'finishing'].includes(String(v.stage)) ||
    !context(v.target_context) ||
    !object(v.source) ||
    !object(v.values)
  )
    return false;
  const a = v.source;
  return (
    keys(a, [
      'collection_id',
      'collection_name',
      'preset_id',
      'preset_name',
      'revision',
      'content_sha256',
      'context',
    ]) &&
    ['collection_id', 'collection_name', 'preset_id', 'preset_name', 'revision'].every((k) =>
      text(a[k]),
    ) &&
    typeof a.content_sha256 === 'string' &&
    /^[a-f0-9]{64}$/.test(a.content_sha256) &&
    context(a.context) &&
    Object.keys(v.values).length > 0 &&
    Object.entries(v.values).every(
      ([k, n]) => (reuseFields as readonly string[]).includes(k) && validReuseNumber(k, n),
    )
  );
}
export function reuseRecords(extensions: Record<string, unknown>): ReuseRecord[] {
  const v = extensions[REUSE_KEY];
  if (
    !object(v) ||
    !keys(v, ['version', 'records']) ||
    v.version !== 1 ||
    !Array.isArray(v.records) ||
    v.records.length > 256 ||
    JSON.stringify(v).length > 262144 ||
    !v.records.every(validRecord)
  )
    return [];
  const ids = v.records.map((r) => `${r.operation_id}:${r.stage}`);
  return new Set(ids).size === ids.length ? (v.records as ReuseRecord[]) : [];
}
export function writableReuseRecords(extensions: Record<string, unknown>) {
  const v = extensions[REUSE_KEY];
  const rows = reuseRecords(extensions);
  if (
    v !== undefined &&
    (!object(v) ||
      !keys(v, ['version', 'records']) ||
      v.version !== 1 ||
      !Array.isArray(v.records) ||
      v.records.length !== rows.length ||
      JSON.stringify(v).length > 262144)
  )
    throw new Error('reuse.error.metadata');
  return rows;
}
/** Normal delete commands remove associations; unknown metadata is preserved opaquely. */
export function withoutOperationReuse(extensions: Record<string, unknown>, id: number) {
  const rows = reuseRecords(extensions);
  return rows.some((r) => r.operation_id === id)
    ? {
        ...extensions,
        [REUSE_KEY]: { version: 1, records: rows.filter((r) => r.operation_id !== id) },
      }
    : extensions;
}
export function reuseState(
  op: OpEntry,
  r: ReuseRecord,
  s: CommandTarget,
): 'unassociated' | 'superseded' | 'modified' | 'copied' {
  if (op.id !== r.operation_id || operationAnchor(op) !== r.operation_anchor) return 'unassociated';
  if (op.stagePreset) return 'superseded';
  try {
    const v = stageValues(op, r.stage);
    return JSON.stringify(targetReuseContext(s, op, r.stage)) !==
      JSON.stringify(r.target_context) ||
      Object.entries(r.values).some(([k, n]) => v[k as ReuseField] !== n)
      ? 'modified'
      : 'copied';
  } catch {
    return 'unassociated';
  }
}
