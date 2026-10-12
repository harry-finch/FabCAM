import type { components } from '../api/generated';
import type { OpEntry } from './op_types';
import type { MachineSettings, ToolEntry } from './project-types';
type S = components['schemas'];
export type DatasetCatalog = {
  machines: CatalogItem[];
  materials: CatalogItem[];
  tools: CatalogItem[];
  description: string;
};
export type CatalogItem = { identity: S['FsIdentity']; label: string; max_rpm?: number | null };
export function parseDatasetCatalog(contents: string): DatasetCatalog {
  const v = JSON.parse(contents);
  if (v.format_version !== 1 || typeof v.description !== 'string')
    throw new Error('Unsupported dataset format');
  for (const key of ['machines', 'materials', 'tools']) {
    if (
      !Array.isArray(v[key]) ||
      v[key].some(
        (x: CatalogItem) => !x?.identity?.id || !x.identity.revision || typeof x.label !== 'string',
      )
    )
      throw new Error('Invalid dataset catalog');
  }
  return v as DatasetCatalog;
}
export function cuttingContextKey(
  op: OpEntry,
  machine: MachineSettings,
  tools: ToolEntry[],
  material?: S['FsIdentity'],
  fsContext?: S['FsContext'],
): string {
  const {
    appliedCutting: _applied,
    appliedContextKey: _key,
    name: _name,
    enabled: _enabled,
    ...geometry
  } = op;
  void _applied;
  void _key;
  void _name;
  void _enabled;
  const finishId = 'finishToolId' in op ? op.finishToolId : undefined;
  return JSON.stringify([
    geometry,
    machine,
    tools.filter((t) => t.id === op.toolId || t.id === finishId),
    material ?? null,
    fsContext ?? null,
  ]);
}
export function preserveManualCutting(op: OpEntry, patch: Partial<OpEntry>): Partial<OpEntry> {
  if (!op.appliedCutting || 'appliedCutting' in patch) return patch;
  const applied = JSON.parse(JSON.stringify(op.appliedCutting)) as S['AppliedCutting'];
  let changed = false;
  for (const [key, field, origin] of [
    ['feedRateOverride', 'feed_mm_min', 'feed'],
    ['plungeRateOverride', 'plunge_mm_min', 'plunge'],
  ] as const) {
    const value = patch[key];
    if (value !== undefined && Number.isInteger(value) && value > 0) {
      for (const slot of applied.slots.filter((s) => s.tool_id === op.toolId)) {
        slot[field] = value;
        slot.origins[origin] = 'manual';
      }
      changed = true;
    }
  }
  if ('step' in patch || 'depthList' in patch) {
    applied.doc_origin = 'manual';
    changed = true;
  }
  if ('xyOverlap' in patch) {
    applied.stepover_origin = 'manual';
    changed = true;
  }
  const result = { ...patch };
  if ('feedRateOverride' in patch) result.feedRateOverride = undefined;
  if ('plungeRateOverride' in patch) result.plungeRateOverride = undefined;
  return changed ? { ...result, appliedCutting: applied } : result;
}

export function sourceLink(reference: string): string | null {
  try {
    const url = new URL(reference);
    return ['https:', 'http:'].includes(url.protocol) ? url.href : null;
  } catch {
    return null;
  }
}

/** Preserve every known hard limit when binding a dataset cutter. */
export function minimumKnownRpm(...values: (number | null | undefined)[]): number | null {
  const known = values.filter((v): v is number => v != null);
  return known.length ? Math.min(...known) : null;
}

export function operationRates(op: OpEntry, tool: ToolEntry, pass: 'rough' | 'finish') {
  const saved = op.appliedCutting?.slots.find((s) => s.tool_id === tool.id && s.pass === pass);
  if (op.appliedCutting) return saved ?? null;
  return {
    tool_id: tool.id,
    pass,
    rpm: pass === 'finish' ? (tool.speedFinish ?? tool.speed) : tool.speed,
    feed_mm_min:
      op.feedRateOverride ??
      (pass === 'finish' ? (tool.feedRateFinish ?? tool.feedRate) : tool.feedRate),
    plunge_mm_min:
      op.plungeRateOverride ??
      (pass === 'finish' ? (tool.plungeRateFinish ?? tool.plungeRate) : tool.plungeRate),
    origins: {
      rpm: 'legacy_snapshot',
      feed: 'legacy_snapshot',
      plunge: 'legacy_snapshot',
    } as S['RateOrigins'],
  };
}
export function editOperationRate(
  op: OpEntry,
  tools: ToolEntry[],
  toolId: number,
  pass: 'rough' | 'finish',
  field: 'rpm' | 'feed_mm_min' | 'plunge_mm_min',
  value: number,
): S['AppliedCutting'] {
  if (!Number.isInteger(value) || value <= 0 || value > 0xffffffff)
    throw new Error('Rates must be positive integers');
  const block: S['AppliedCutting'] = op.appliedCutting
    ? JSON.parse(JSON.stringify(op.appliedCutting))
    : { slots: [], context_key: '', doc_origin: 'legacy_snapshot' };
  const ids = [
    ...new Set([op.toolId, ...('finishToolId' in op && op.finishToolId ? [op.finishToolId] : [])]),
  ];
  for (const id of ids) {
    const tool = tools.find((t) => t.id === id);
    if (!tool) throw new Error('Tool missing');
    for (const role of ['rough', 'finish'] as const) {
      if (!block.slots.some((s) => s.tool_id === id && s.pass === role)) {
        const defaults = operationRates({ ...op, appliedCutting: undefined }, tool, role)!;
        block.slots.push(defaults);
      }
    }
  }
  const slot = block.slots.find((s) => s.tool_id === toolId && s.pass === pass);
  if (!slot) throw new Error('Cutting role missing');
  slot[field] = value;
  slot.origins[field === 'rpm' ? 'rpm' : field === 'feed_mm_min' ? 'feed' : 'plunge'] = 'manual';
  return block;
}
export function referenceContext(
  machine: MachineSettings,
  tools: ToolEntry[],
  old?: S['FsContext'],
): S['FsContext'] {
  return {
    machine_identity: machine.fsIdentity !== undefined ? machine.fsIdentity : old?.machine_identity,
    tools: tools.flatMap((tool) => {
      const binding = old?.tools.find((b) => b.tool_id === tool.id);
      const identity = tool.fsIdentity !== undefined ? tool.fsIdentity : binding?.identity;
      return identity ? [{ tool_id: tool.id, identity, max_rpm: binding?.max_rpm }] : [];
    }),
  };
}
