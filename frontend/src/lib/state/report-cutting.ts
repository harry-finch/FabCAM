import { cuttingRoles } from './cutting-parameters';
import { isProgramOnlyOp, type OpEntry } from './op_types';
import type { ToolEntry } from './project-types';

/** Authored v10 stages or unchanged actual roles of other operation families. */
export function reportCuttingRoles(op: OpEntry) {
  return cuttingRoles(op).filter(
    (r) => !!op.stages || op.kind !== 'profile' || r.role === 'finish',
  );
}
export function reportToolIds(op: OpEntry): number[] {
  if (isProgramOnlyOp(op.kind)) return [];
  const roles = reportCuttingRoles(op);
  return [...new Set(roles.length ? roles.map((r) => r.tool_id) : [op.toolId])];
}
export function reportToolNames(op: OpEntry, tools: ToolEntry[]): string {
  return (
    reportToolIds(op)
      .map((id) => `#${id} ${tools.find((t) => t.id === id)?.name ?? '—'}`)
      .join(' · ') || '—'
  );
}
/** Saved operation inputs, not measured speeds or a claim of generation validity. */
export function reportCuttingText(
  op: OpEntry,
  labels: Partial<Record<string, string>> = {},
): string {
  const number = (v: number | null | undefined) =>
    typeof v === 'number' && Number.isInteger(v) && v > 0 && v <= 0xffffffff ? String(v) : '—';
  if (op.stages) {
    const s = op.stages;
    const row = (name: string, v: typeof s.main) =>
      `${name} · #${v.tool_id}: ${number(v.rpm)} rpm · ${number(v.feed_mm_min)} / ${number(v.plunge_mm_min)} mm/min · ${v.depth_per_pass_mm ?? '—'} mm/pass`;
    return [
      row('Main', s.main),
      ...(s.finishing
        ? [
            row('Finishing', s.finishing.cutting),
            `Allowances: lateral ${s.finishing.lateral_allowance_mm} mm / floor ${s.finishing.floor_allowance_mm} mm`,
          ]
        : ['Finishing off']),
      `Retained for later: ${op.retainedStockMm ?? 0} mm`,
    ].join(' · ');
  }
  return (
    reportCuttingRoles(op)
      .map(({ role, tool_id }) => {
        const matches =
          op.cutting?.rates.filter((r) => r.role === role && r.tool_id === tool_id) ?? [];
        const r = matches.length === 1 ? matches[0] : undefined;
        return `${labels[role] ?? role} · #${tool_id}: ${number(r?.rpm)} rpm · ${number(r?.feed_mm_min)} / ${number(r?.plunge_mm_min)} mm/min`;
      })
      .join(' · ') || '—'
  );
}

/** Only call with current generated evidence. No range is inferred from tool ID. */
export function reportEmittedStages(
  opId: number,
  bindings: NonNullable<import('../api/types').GenerateResponse['cutting_stages']>,
  labels: { main: string; finishing: string; emitted: string; empty: string },
): string {
  return bindings
    .filter((b) => b.operation_id === opId)
    .map(
      (b) =>
        `${b.authority === 'resolved' ? labels.emitted : 'Recorded intent'}: ${b.stage === 'main' ? labels.main : labels.finishing} · #${b.parameters.tool_id} · ${b.first_segment === b.end_segment_exclusive ? labels.empty : `L${b.output_start_line}–${b.output_end_line} · ${b.end_segment_exclusive - b.first_segment} moves`}`,
    )
    .join(' · ');
}
export function stageAtSegment(
  bindings: NonNullable<import('../api/types').GenerateResponse['cutting_stages']>,
  segment: number,
) {
  return bindings.find((b) => segment >= b.first_segment && segment < b.end_segment_exclusive);
}
