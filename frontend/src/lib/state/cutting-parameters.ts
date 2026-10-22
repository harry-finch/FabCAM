import type { components } from '../api/generated';
import { isProgramOnlyOp, type OpEntry } from './op_types';
import type { ToolEntry } from './project-types';
export type MillingCutting = components['schemas']['MillingCutting'];
export type RoleRates = components['schemas']['RoleRates'];
export type CuttingRole = RoleRates['role'];
export function cuttingRoles(op: OpEntry): { role: CuttingRole; tool_id: number }[] {
  if (isProgramOnlyOp(op.kind) || op.kind === 'drag_knife' || op.kind === 'raster_engrave')
    return [];
  if (op.kind === 'drill')
    return [
      { role: 'drill', tool_id: op.toolId },
      ...(op.spotFirst ? [{ role: 'spot' as const, tool_id: op.spotFirst.spotToolId }] : []),
      ...(op.chamferAfterWidthMm !== undefined
        ? [{ role: 'chamfer' as const, tool_id: op.finishToolId ?? op.toolId }]
        : []),
    ];
  if (op.kind === 'pocket' && op.pocketStrategy === 'halfpipe')
    return [{ role: 'rough', tool_id: op.toolId }];
  if (op.kind === 'pocket')
    return [
      { role: 'rough', tool_id: op.toolId },
      { role: 'finish', tool_id: op.finishToolId ?? op.toolId },
    ];
  return [
    { role: 'rough', tool_id: op.toolId },
    ...(op.kind === 'profile' || (op.kind === 'chamfer' && op.chamferFinishPass)
      ? [{ role: 'finish' as const, tool_id: op.toolId }]
      : []),
  ];
}
export function usesEntryAngle(op: OpEntry): boolean {
  return (
    op.kind === 'vcarve' ||
    (op.kind === 'pocket' && op.pocketStrategy === 'halfpipe') ||
    (op.kind === 'drill' && op.chamferAfterWidthMm !== undefined)
  );
}
const positive = (v: number | undefined) =>
  Number.isInteger(v) && v! > 0 && v! <= 0xffffffff ? v! : null;
/** Called at creation only. Existing operation values never read live defaults. */
export function seedCutting(op: OpEntry, tools: ToolEntry[]): MillingCutting {
  return {
    ...(usesEntryAngle(op) ? { entry_angle_deg: 10 } : {}),
    rates: cuttingRoles(op).map(({ role, tool_id }) => {
      const t = tools.find((t) => t.id === tool_id);
      const drill = ['drill', 'spot', 'chamfer'].includes(role);
      return {
        role,
        tool_id,
        rpm: positive(
          role === 'finish'
            ? (t?.speedFinish ?? t?.speed)
            : drill
              ? (t?.speedDrill ?? t?.speed)
              : t?.speed,
        ),
        feed_mm_min: positive(
          role === 'finish'
            ? (t?.feedRateFinish ?? t?.feedRate)
            : drill
              ? (t?.feedRateDrill ?? t?.feedRate)
              : t?.feedRate,
        ),
        plunge_mm_min: positive(
          role === 'finish'
            ? (t?.plungeRateFinish ?? t?.plungeRate)
            : drill
              ? (t?.plungeRateDrill ?? t?.plungeRate)
              : t?.plungeRate,
        ),
      };
    }),
  };
}
export function validateCuttingStructure(op: OpEntry): void {
  const c = op.cutting;
  if (!c || !Array.isArray(c.rates) || c.rates.length > 5)
    throw new Error(`${op.name}: missing cutting values`);
  if (Object.keys(c).some((k) => !['rates', 'preset_snapshot', 'entry_angle_deg'].includes(k)))
    throw new Error('Unknown cutting field');
  if (c.preset_snapshot != null) validateLoadedSnapshot(c.preset_snapshot);
  if (
    c.entry_angle_deg != null &&
    (typeof c.entry_angle_deg !== 'number' || !Number.isFinite(c.entry_angle_deg))
  )
    throw new Error('Invalid entry angle');
  const expected = cuttingRoles(op)
    .map((r) => `${r.role}/${r.tool_id}`)
    .sort();
  const actual = c.rates.map((r) => `${r.role}/${r.tool_id}`).sort();
  if (JSON.stringify(expected) !== JSON.stringify(actual))
    throw new Error(`${op.name}: invalid cutting roles or assigned tools`);
  for (const r of c.rates) {
    if (
      Object.keys(r).some(
        (k) => !['role', 'tool_id', 'rpm', 'feed_mm_min', 'plunge_mm_min'].includes(k),
      )
    )
      throw new Error('Unknown cutting rate field');
    for (const field of ['rpm', 'feed_mm_min', 'plunge_mm_min'] as const) {
      const v = r[field];
      if (v !== null && (typeof v !== 'number' || !Number.isInteger(v) || v < 0 || v > 0xffffffff))
        throw new Error(`${op.name}: invalid ${r.role} ${field}`);
    }
  }
}

/** Existing roles keep their numbers; newly enabled roles are visibly incomplete. */
export function rebindCutting(op: OpEntry, patch: Partial<OpEntry>): Partial<OpEntry> {
  if (!op.cutting || patch.cutting !== undefined) return patch;
  const next = { ...op, ...patch } as OpEntry;
  const roles = cuttingRoles(next);
  if (
    JSON.stringify(roles) ===
    JSON.stringify(op.cutting.rates.map(({ role, tool_id }) => ({ role, tool_id })))
  )
    return patch;
  const rates = roles.map((r) => {
    const old = op.cutting!.rates.find((v) => v.role === r.role);
    return old
      ? { ...old, tool_id: r.tool_id }
      : { ...r, rpm: null, feed_mm_min: null, plunge_mm_min: null };
  });
  return { ...patch, cutting: { ...op.cutting, rates } };
}

export function validateLoadedSnapshot(
  value: NonNullable<MillingCutting['preset_snapshot']>,
): void {
  const object = (v: unknown, keys: string[]) => {
    if (
      !v ||
      typeof v !== 'object' ||
      Array.isArray(v) ||
      Object.keys(v).some((k) => !keys.includes(k))
    )
      throw new Error('Invalid loaded preset snapshot');
  };
  const text = (v: unknown, max: number) =>
    typeof v === 'string' && v.trim().length > 0 && new TextEncoder().encode(v).length <= max;
  object(value, [
    'collection_id',
    'collection_name',
    'preset_id',
    'preset_name',
    'context',
    'parameters',
  ]);
  if (
    ![value.collection_id, value.preset_id].every(
      (v) => typeof v === 'string' && /^[a-zA-Z0-9_-]{1,96}$/.test(v),
    ) ||
    ![value.collection_name, value.preset_name].every((v) => text(v, 160))
  )
    throw new Error('Invalid loaded preset identity');
  const c = value.context;
  object(c, ['machine_id', 'machine_label', 'material_id', 'material_label', 'signature']);
  if (
    ![c.machine_id, c.material_id].every((v) => text(v, 256)) ||
    ![c.machine_label, c.material_label].every((v) => text(v, 160)) ||
    !text(c.signature, 65536)
  )
    throw new Error('Invalid loaded preset context');
  JSON.parse(c.signature);
  const p = value.parameters;
  object(p, [
    'rates',
    'schedule',
    'pocket_overlap',
    'planar_stepover_mm',
    'entry_angle_deg',
    'specialized',
  ]);
  if (
    p.entry_angle_deg != null &&
    (!Number.isFinite(p.entry_angle_deg) || p.entry_angle_deg < 1 || p.entry_angle_deg > 89)
  )
    throw new Error('Invalid entry angle');
  if (p.specialized != null) {
    const s = p.specialized;
    const positive = (v: unknown) => typeof v === 'number' && Number.isFinite(v) && v > 0;
    if (s.kind === 'drill') {
      object(s, ['kind', 'cycle', 'spot_depth_mm', 'chamfer_width_mm']);
      object(
        s.cycle,
        s.cycle?.kind === 'simple' ? ['kind', 'dwell_sec'] : ['kind', 'dwell_sec', 'peck_step_mm'],
      );
      if (
        !['simple', 'peck', 'chip_break'].includes(s.cycle.kind) ||
        !Number.isFinite(s.cycle.dwell_sec) ||
        s.cycle.dwell_sec < 0 ||
        (s.cycle.kind !== 'simple' && !positive(s.cycle.peck_step_mm)) ||
        (s.spot_depth_mm != null && (!Number.isFinite(s.spot_depth_mm) || s.spot_depth_mm >= 0)) ||
        (s.chamfer_width_mm != null && !positive(s.chamfer_width_mm))
      )
        throw new Error('Invalid drilling preset');
    } else if (s.kind === 'relief') {
      object(s, ['kind', 'scallop_height_mm', 'stepover_mm', 'along_step_mm']);
      if (
        !Number.isFinite(s.scallop_height_mm) ||
        s.scallop_height_mm < 0 ||
        !positive(s.along_step_mm) ||
        (s.stepover_mm == null ? !positive(s.scallop_height_mm) : !positive(s.stepover_mm))
      )
        throw new Error('Invalid relief preset');
    } else if (s.kind === 'waterline') {
      object(s, ['kind', 'z_step_mm', 'stepover_mm']);
      if (!positive(s.z_step_mm) || !positive(s.stepover_mm))
        throw new Error('Invalid waterline preset');
    } else throw new Error('Unknown specialized preset');
  }
  if (!Array.isArray(p.rates) || !p.rates.length || p.rates.length > 5)
    throw new Error('Invalid loaded preset roles');
  const roles = new Set<string>();
  for (const r of p.rates) {
    object(r, ['role', 'tool_id', 'rpm', 'feed_mm_min', 'plunge_mm_min']);
    if (
      !['rough', 'finish', 'drill', 'spot', 'chamfer'].includes(r.role) ||
      roles.has(r.role) ||
      ![r.tool_id, r.rpm, r.feed_mm_min, r.plunge_mm_min].every(
        (v) => typeof v === 'number' && Number.isInteger(v) && v > 0 && v <= 0xffffffff,
      )
    )
      throw new Error('Invalid loaded preset values');
    roles.add(r.role);
  }
  const schedule = p.schedule;
  object(
    schedule,
    schedule?.kind === 'step' ? ['kind', 'step', 'finish_step'] : ['kind', 'depths'],
  );
  if (schedule.kind === 'step') {
    for (const v of [schedule.step, schedule.finish_step])
      if (v !== null && (typeof v !== 'number' || !Number.isFinite(v) || v >= -1e-9))
        throw new Error('Invalid preset depth per pass');
  } else if (schedule.kind === 'explicit') {
    const d = schedule.depths;
    if (
      !Array.isArray(d) ||
      !d.length ||
      d.length > 4096 ||
      d.some((v, i) => !Number.isFinite(v) || (i > 0 && v >= d[i - 1]))
    )
      throw new Error('Invalid preset depth schedule');
  } else throw new Error('Unknown preset depth schedule');
  if (
    p.pocket_overlap != null &&
    (!Number.isFinite(p.pocket_overlap) || p.pocket_overlap <= 0 || p.pocket_overlap >= 1)
  )
    throw new Error('Invalid preset overlap');
  if (
    p.planar_stepover_mm != null &&
    (!Number.isFinite(p.planar_stepover_mm) || p.planar_stepover_mm <= 0)
  )
    throw new Error('Invalid preset stepover');
}
