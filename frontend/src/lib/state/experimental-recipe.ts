import type { components } from '../api/generated';
import type { PresetRow } from './preset-library';
import type { OpEntry } from './op_types';
import { isLabPackId } from './lab-presets';
type Recipe = components['schemas']['ExperimentalRecipe'];
/** Presentation eligibility only; Rust always validates the source and target. */
export function trialEligible(row: PresetRow): boolean {
  return !!(
    row.enabled &&
    row.milling &&
    isLabPackId(row.pack.manifest.identity.id) &&
    row.pack.manifest.distribution === 'local_only' &&
    row.record?.lab?.state === 'draft' &&
    !row.record.lab.deleted &&
    !row.record.lab.enabled
  );
}
export function validateExperimentalRecipe(recipe: Recipe): void {
  const exact = (v: unknown, keys: string[]) => {
    if (
      !v ||
      typeof v !== 'object' ||
      Array.isArray(v) ||
      Object.keys(v).some((k) => !keys.includes(k)) ||
      keys.some((k) => !(k in v))
    )
      throw new Error('Invalid experimental recipe snapshot');
  };
  const identity = (id: { id: string; revision: string }) => {
    exact(id, ['id', 'revision']);
    for (const s of [id.id, id.revision])
      if (typeof s !== 'string' || !s || s.length > 256 || /^\//.test(s) || /[\\:\s]/.test(s))
        throw new Error('Invalid portable trial identity');
  };
  exact(recipe, ['kind', 'source', 'target', 'applied_values']);
  exact(recipe.source, ['pack', 'record_id', 'profile']);
  exact(recipe.target, [
    'machine',
    'material',
    'tool',
    'tool_id',
    'operation',
    'diameter_mm',
    'flute_count',
  ]);
  exact(recipe.applied_values, ['rpm', 'feed_mm_min', 'plunge_mm_min', 'doc_mm', 'stepover_ratio']);
  if (recipe.kind !== 'theoretical_untested' || !isLabPackId(recipe.source.pack.id))
    throw new Error('Only theoretical local trial provenance is supported');
  [
    recipe.source.pack,
    recipe.source.profile,
    recipe.target.machine,
    recipe.target.material,
    recipe.target.tool,
    { id: recipe.source.record_id, revision: '1' },
  ].forEach(identity);
  const v = recipe.applied_values;
  if (
    ![
      v.rpm,
      v.feed_mm_min,
      v.plunge_mm_min,
      recipe.target.tool_id,
      recipe.target.flute_count,
    ].every((n) => Number.isInteger(n) && n > 0 && n <= 0xffffffff) ||
    ![v.doc_mm, recipe.target.diameter_mm].every((n) => Number.isFinite(n) && n > 0) ||
    (v.stepover_ratio != null &&
      (!Number.isFinite(v.stepover_ratio) || v.stepover_ratio <= 0 || v.stepover_ratio >= 1)) ||
    !['profile', 'full_slot', 'pocket'].includes(recipe.target.operation)
  )
    throw new Error('Invalid experimental values or target');
}
export function experimentalModified(op: OpEntry): boolean {
  const a = op.appliedCutting,
    r = a?.experimental_recipe;
  if (!a || !r) return false;
  const v = r.applied_values;
  return (
    a.slots.some(
      (s) =>
        s.tool_id !== r.target.tool_id ||
        s.rpm !== v.rpm ||
        s.feed_mm_min !== v.feed_mm_min ||
        s.plunge_mm_min !== v.plunge_mm_min,
    ) ||
    Math.abs(op.step ?? 0) !== v.doc_mm ||
    !!op.depthList?.length ||
    (op.kind === 'pocket' && Math.abs(1 - (op.xyOverlap ?? 0) - (v.stepover_ratio ?? 0)) > 1e-12)
  );
}
