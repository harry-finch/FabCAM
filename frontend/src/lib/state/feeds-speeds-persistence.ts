import type { OpEntry } from './op_types';
import type { ProjectFile } from './project-types';
export function hasFeedsSpeedsData(value: {
  material?: unknown;
  fsContext?: unknown;
  operations?: OpEntry[];
  machine?: { fsIdentity?: unknown };
  tools?: { fsIdentity?: unknown }[];
}): boolean {
  return (
    !!value.material ||
    !!value.fsContext ||
    value.machine?.fsIdentity !== undefined ||
    !!value.tools?.some((tool) => tool.fsIdentity !== undefined) ||
    !!value.operations?.some((op) => op.appliedCutting || op.fsFullWidth)
  );
}
export function validateProjectFile(value: ProjectFile): void {
  if (
    !value ||
    !(
      (value.kind === 'ivac-project' && value.version === 1) ||
      (value.kind === 'ivac-project-fs' && value.version === 2) ||
      (value.kind === 'ivac-project-cutting' && value.version === 3) ||
      (value.kind === 'ivac-project-knowledge' && value.version === 4)
    )
  )
    throw new Error('Unsupported project format/version');
  if (value.version === 1 && hasFeedsSpeedsData(value))
    throw new Error('Cutting parameters require a guarded project format');
  for (const op of value.operations ?? []) {
    const applied = op.appliedCutting;
    if (!applied) continue;
    if (!Array.isArray(applied.slots) || typeof applied.context_key !== 'string')
      throw new Error('Incomplete applied cutting parameters');
    if (
      (value.version === 2 && !applied.proposal) ||
      (applied.proposal &&
        (!applied.proposal.provenance?.profile?.source ||
          !applied.proposal.provenance.dataset ||
          !applied.proposal.parameters))
    )
      throw new Error('Incomplete recommendation snapshot');
    if (applied.knowledge) {
      if (value.version !== 4)
        throw new Error('Source-pack provenance requires guarded project version 4');
      const k = applied.knowledge;
      if (
        !k.selected_pack?.id ||
        !k.selected_pack.revision ||
        !k.selected_record ||
        !k.policy ||
        !k.engine_version ||
        !['theoretical', 'reference_data', 'tested', 'validated'].includes(k.evidence) ||
        typeof k.local_only !== 'boolean' ||
        !Number.isInteger(k.compatible_sources) ||
        k.compatible_sources < 1
      )
        throw new Error('Invalid source-pack provenance');
      if (k.local_only && applied.proposal)
        throw new Error('Private source records cannot be embedded in projects');
    }
    const keys = new Set<string>();
    for (const slot of applied.slots) {
      const key = `${slot.tool_id}/${slot.pass}`;
      if (
        keys.has(key) ||
        !['rough', 'finish'].includes(slot.pass) ||
        !slot.origins ||
        ![slot.origins.rpm, slot.origins.feed, slot.origins.plunge].every((origin) =>
          ['recommendation', 'manual', 'legacy_snapshot'].includes(origin),
        ) ||
        ![slot.tool_id, slot.rpm, slot.feed_mm_min, slot.plunge_mm_min].every(
          (v) => Number.isInteger(v) && v > 0 && v <= 0xffffffff,
        )
      )
        throw new Error('Invalid applied cutting rate role');
      keys.add(key);
    }
  }
}
