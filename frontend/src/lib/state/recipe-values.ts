/** Editable recipe values; no calculation, clamping or application to CAM. */
import type { PresetRow } from './preset-library';
import type { MachineSettings } from './project-types';
export type RecipeData = PresetRow['profile']['data'];
export type RecipeInputs = {
  rpm: number | undefined;
  rate: number | undefined;
  plunge: number | undefined;
  depth: number | undefined;
  stepover: number | undefined;
};
export function recipeInputs(data: RecipeData): RecipeInputs {
  return {
    rpm: data.rpm,
    rate: data.kind === 'exact_values' ? data.feed_mm_min : data.chipload_mm_tooth,
    plunge: data.plunge_mm_min,
    depth: data.doc_mm,
    stepover: data.stepover_ratio == null ? undefined : data.stepover_ratio * 100,
  };
}
export function recipeData(source: RecipeData, inputs: RecipeInputs): RecipeData | null {
  if (
    ![inputs.rpm, inputs.rate, inputs.plunge, inputs.depth].every(
      (v) => typeof v === 'number' && Number.isFinite(v) && v > 0,
    )
  )
    return null;
  if (
    inputs.stepover != null &&
    (!Number.isFinite(inputs.stepover) || inputs.stepover <= 0 || inputs.stepover > 100)
  )
    return null;
  const common = {
    rpm: inputs.rpm!,
    plunge_mm_min: inputs.plunge!,
    doc_mm: inputs.depth!,
    stepover_ratio: inputs.stepover == null ? null : inputs.stepover / 100,
  };
  return source.kind === 'exact_values'
    ? { kind: 'exact_values', ...common, feed_mm_min: inputs.rate! }
    : { kind: 'chipload_target', ...common, chipload_mm_tooth: inputs.rate! };
}
export function recipeProblems(
  data: RecipeData,
  operation: string,
  machine: MachineSettings,
): ('values' | 'limits')[] {
  if (
    !recipeData(data, recipeInputs(data)) ||
    (operation === 'pocket' && data.stepover_ratio == null) ||
    (['profile', 'full_slot'].includes(operation) && data.stepover_ratio != null)
  )
    return ['values'];
  if (
    (machine.spindleRpmMin != null && data.rpm < machine.spindleRpmMin) ||
    (machine.spindleRpmMax != null && data.rpm > machine.spindleRpmMax) ||
    (machine.maxFeedMmMin != null &&
      (data.plunge_mm_min > machine.maxFeedMmMin ||
        (data.kind === 'exact_values' && data.feed_mm_min > machine.maxFeedMmMin)))
  )
    return ['limits'];
  return [];
}
