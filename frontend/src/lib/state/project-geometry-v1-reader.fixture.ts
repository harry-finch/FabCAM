import { drawingInputKey } from './drawing';
import type { ProjectFile } from './project-types';
/** Retained geometry guards, independent of cutting-file versions. */
export function validateProjectGeometry(value: ProjectFile): void {
  if (
    !value.drawing &&
    (value.operations ?? []).some(
      (op) => op.geometryBinding?.kind === 'contours' || op.geometryBinding?.kind === 'unresolved',
    )
  )
    throw new Error(
      'Stable or unresolved geometry bindings require guarded edited-drawing format; reselect geometry before saving',
    );
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
  if (value.cornerRelief) {
    const m = value.cornerRelief;
    if (
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
  }
  const surfaces = (value.operations ?? []).filter((op) => op.kind === 'planar_surface');
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
}
