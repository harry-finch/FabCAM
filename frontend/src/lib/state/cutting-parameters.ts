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
  if (op.kind === 'pocket')
    return [
      { role: 'rough', tool_id: op.toolId },
      { role: 'finish', tool_id: op.finishToolId ?? op.toolId },
    ];
  return [
    { role: 'rough', tool_id: op.toolId },
    ...(['profile', 'engrave', 'chamfer'].includes(op.kind)
      ? [{ role: 'finish' as const, tool_id: op.toolId }]
      : []),
  ];
}
const positive = (v: number | undefined) =>
  Number.isInteger(v) && v! > 0 && v! <= 0xffffffff ? v! : null;
/** Called at creation only. Existing operation values never read live defaults. */
export function seedCutting(op: OpEntry, tools: ToolEntry[]): MillingCutting {
  return {
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
  if (Object.keys(c).some((k) => !['rates', 'preset_snapshot'].includes(k)))
    throw new Error('Unknown cutting field');
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
