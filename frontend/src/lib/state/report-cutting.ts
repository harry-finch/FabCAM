import { cuttingRoles } from './cutting-parameters';
import { isProgramOnlyOp, type OpEntry } from './op_types';
import type { ToolEntry } from './project-types';

/** Actual CAM roles. Profile emits Finish; its stored Rough slot is not used. */
export function reportCuttingRoles(op: OpEntry) {
  return cuttingRoles(op).filter((r) => op.kind !== 'profile' || r.role === 'finish');
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
