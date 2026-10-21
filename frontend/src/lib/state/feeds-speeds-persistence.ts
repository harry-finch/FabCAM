import { drawingInputKey } from './drawing';
import type { OpEntry } from './op_types';
import type { ProjectFile } from './project-types';
export function hasFeedsSpeedsData(value: {
  material?: unknown;
  fsContext?: unknown;
  operations?: OpEntry[];
  machine?: { fsIdentity?: unknown; fsLocalIdentity?: unknown };
  tools?: { fsIdentity?: unknown; fsLocalIdentity?: unknown }[];
}): boolean {
  return (
    !!value.material ||
    !!value.fsContext ||
    value.machine?.fsIdentity !== undefined ||
    value.machine?.fsLocalIdentity !== undefined ||
    !!value.tools?.some(
      (tool) => tool.fsIdentity !== undefined || tool.fsLocalIdentity !== undefined,
    ) ||
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
      (value.kind === 'ivac-project-knowledge' && value.version === 4) ||
      (value.kind === 'ivac-project-cad' && value.version === 5) ||
      (value.kind === 'ivac-project-surfacing' && value.version === 6) ||
      (value.kind === 'ivac-project-geometry' && value.version === 7)
    )
  )
    throw new Error('Unsupported project format/version');
  if (
    !value.drawing &&
    (value.operations ?? []).some(
      (op) => op.geometryBinding?.kind === 'contours' || op.geometryBinding?.kind === 'unresolved',
    )
  )
    throw new Error(
      'Stable or unresolved geometry bindings require guarded edited-drawing format; reselect geometry before saving',
    );
  if ((value.version === 7) !== !!value.drawing)
    throw new Error('Edited drawing requires guarded version 7');
  if (
    value.drawing &&
    (!Array.isArray(value.drawing.originalBindings) ||
      value.drawing.version !== 1 ||
      value.drawing.response?.version !== 1 ||
      value.drawing.response.drawing?.version !== 1 ||
      value.drawing.inputKey !== drawingInputKey(value.imports))
  )
    throw new Error('Invalid or incompatible edited drawing snapshot');
  if (value.drawing) {
    const ids = new Set<number>();
    for (const binding of value.drawing.originalBindings) {
      if (
        !binding ||
        !Number.isSafeInteger(binding.id) ||
        binding.id <= 0 ||
        ids.has(binding.id) ||
        (binding.sourceObjects != null &&
          (!Array.isArray(binding.sourceObjects) ||
            !binding.sourceObjects.every((id) => Number.isSafeInteger(id) && id > 0))) ||
        (binding.sourceLayers != null &&
          (!Array.isArray(binding.sourceLayers) ||
            !binding.sourceLayers.every((name) => typeof name === 'string'))) ||
        binding.geometryBinding?.kind === 'contours' ||
        binding.geometryBinding?.kind === 'unresolved'
      )
        throw new Error('Invalid original operation bindings');
      ids.add(binding.id);
    }
    for (const op of value.operations ?? []) {
      const b = op.geometryBinding;
      if (!b) continue;
      const identity = (id: { import: number; entity: number }) =>
        id &&
        Number.isInteger(id.import) &&
        id.import > 0 &&
        Number.isInteger(id.entity) &&
        id.entity > 0;
      if (
        !(
          b.kind === 'all' ||
          (b.kind === 'layers' &&
            Array.isArray(b.names) &&
            b.names.every((n) => typeof n === 'string')) ||
          (b.kind === 'contours' &&
            Array.isArray(b.ids) &&
            b.ids.length > 0 &&
            b.ids.every(identity)) ||
          (b.kind === 'unresolved' &&
            Array.isArray(b.previous) &&
            b.previous.every(identity) &&
            typeof b.reason === 'string')
        )
      )
        throw new Error('Invalid edited operation binding');
    }
  }
  if (value.version === 1 && hasFeedsSpeedsData(value))
    throw new Error('Cutting parameters require a guarded project format');
  if (value.cornerRelief) {
    const m = value.cornerRelief;
    if (
      (value.version !== 5 && value.version !== 6 && value.version !== 7) ||
      m.version !== 1 ||
      typeof m.sourceKey !== 'string' ||
      !Number.isFinite(m.diameter) ||
      m.diameter <= 0 ||
      !Number.isInteger(m.toolId) ||
      !m.recipe?.selected_corners?.length ||
      !Array.isArray(m.evaluated?.segments) ||
      m.evaluated.segments.length !== m.evaluated.objects?.length
    )
      throw new Error('Invalid corner-relief project snapshot');
  } else if (value.version === 5) throw new Error('Missing corner-relief snapshot');
  const surfaces = (value.operations ?? []).filter((op) => op.kind === 'planar_surface');
  if (value.version !== 7 && surfaces.length > 0 !== (value.version === 6))
    throw new Error('Planar surfacing requires guarded project version 6');
  for (const op of surfaces) {
    const p = op.surface;
    if (
      !p ||
      !Array.isArray(p.origin) ||
      p.origin.length !== 2 ||
      ![...p.origin, p.width_mm, p.height_mm, p.margin_mm, p.overtravel_mm, p.stepover_mm].every(
        Number.isFinite,
      ) ||
      p.width_mm <= 0 ||
      p.height_mm <= 0 ||
      p.margin_mm < 0 ||
      p.overtravel_mm < 0 ||
      p.stepover_mm <= 0 ||
      !['along_x', 'along_y'].includes(p.direction)
    )
      throw new Error('Invalid planar surfacing parameters');
  }
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
      if (value.version !== 4 && value.version !== 5 && value.version !== 6 && value.version !== 7)
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
