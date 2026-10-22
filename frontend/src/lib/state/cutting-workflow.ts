import type { components } from '../api/generated';
import { presetCommand } from '../api/wasm';
import { buildGenerationProject } from './generation-context';
import type { ProjectState } from './project.svelte';
import type { OpEntry } from './op_types';
import { updateOperationCommand, type CommandTarget } from './commands';
import type { Command } from './history';
import type { PresetCollection, TestedPreset } from './preset-collections';
type S = components['schemas'];
export interface CuttingInspection {
  check: S['CuttingCheck'];
  parameters: S['PresetParameters'];
  context: S['PresetContext'] | null;
  context_error: string | null;
}
export function cuttingTargetKey(
  target: Pick<
    CommandTarget,
    'operations' | 'machine' | 'tools' | 'material' | 'stock' | 'workOffset'
  >,
  id: number,
): string {
  return JSON.stringify({
    op: target.operations.find((o) => o.id === id),
    machine: target.machine,
    tools: target.tools,
    material: target.material,
    stock: target.stock,
    workOffset: target.workOffset,
  });
}
export async function inspectCutting(p: ProjectState, id: number): Promise<CuttingInspection> {
  const project = buildGenerationProject(p, true);
  if (!project) throw new Error('Create an operation first');
  return (await presetCommand({ action: 'inspect', project, op_id: id })) as CuttingInspection;
}
function patchFromWire(current: OpEntry, next: S['Op']): Partial<OpEntry> {
  const params = next.params;
  return {
    cutting: params.cutting ?? undefined,
    step: params.step ?? null,
    finishStep: params.finish_step ?? undefined,
    depthList: params.depth_list ?? [],
    ...(current.kind === 'pocket' && next.kind.type === 'pocket'
      ? { xyOverlap: next.kind.pocket?.xy_overlap }
      : {}),
    ...(current.kind === 'drill' && next.kind.type === 'drill'
      ? {
          drillCycle:
            next.kind.cycle.kind === 'simple'
              ? { kind: 'simple', dwellSec: next.kind.cycle.dwell_sec }
              : {
                  kind: next.kind.cycle.kind,
                  peckStepMm: next.kind.cycle.peck_step_mm,
                  dwellSec: next.kind.cycle.dwell_sec,
                },
          ...(current.spotFirst && next.kind.spot_first
            ? {
                spotFirst: {
                  ...current.spotFirst,
                  spotDepthMm: next.kind.spot_first.spot_depth_mm,
                },
              }
            : {}),
          chamferAfterWidthMm: next.kind.chamfer_after_width_mm ?? undefined,
        }
      : {}),
    ...(current.kind === 'relief_mill' && next.kind.type === 'relief_mill'
      ? {
          scallopHeightMm: next.kind.scallop_height_mm,
          stepoverMm: next.kind.stepover_mm ?? null,
          alongStepMm: next.kind.along_step_mm,
        }
      : {}),
    ...(current.kind === 'waterline_rough' && next.kind.type === 'waterline_rough'
      ? {
          zStepMm: next.kind.z_step_mm,
          stepoverMm: next.kind.stepover_mm,
        }
      : {}),
    ...(current.kind === 'planar_surface' && next.kind.type === 'planar_surface'
      ? { surface: { ...current.surface, stepover_mm: next.kind.surface.stepover_mm } }
      : {}),
  } as Partial<OpEntry>;
}
export function reviewedCuttingCommand(id: number, key: string, patch: Partial<OpEntry>): Command {
  let edit: Command | undefined;
  return {
    label: 'Load tested preset',
    apply(state) {
      const target = state as CommandTarget;
      if (cuttingTargetKey(target, id) !== key)
        throw new Error('Operation or setup changed since review; preview the preset again');
      edit = updateOperationCommand(id, patch);
      edit.apply(target);
    },
    revert(state) {
      edit?.revert(state);
    },
  };
}
export async function preparePresetLoad(
  p: ProjectState,
  id: number,
  collection: PresetCollection,
  presetId: string,
) {
  const key = cuttingTargetKey(p.data, id);
  const project = buildGenerationProject(p, true);
  const op = p.data.operations.find((o) => o.id === id);
  if (!project || !op) throw new Error('Operation no longer available');
  const next = (await presetCommand({
    action: 'load',
    project,
    op_id: id,
    collection,
    preset_id: presetId,
  })) as S['Op'];
  if (key !== cuttingTargetKey(p.data, id))
    throw new Error('Operation changed while loading; review again');
  const patch = patchFromWire(op, next);
  return {
    key,
    patch,
    before: op.cutting?.rates ?? [],
    after: next.params.cutting!.rates,
    parameters: next.params.cutting!.preset_snapshot!.parameters,
    command: reviewedCuttingCommand(id, key, patch),
  };
}
export async function captureTestedPreset(
  p: ProjectState,
  id: number,
  name: string,
  attestedKey: string,
): Promise<TestedPreset> {
  if (!name.trim()) throw new Error('Enter a preset name');
  if (!attestedKey || attestedKey !== cuttingTargetKey(p.data, id))
    throw new Error('Confirm testing of the current machine, tool, material and values');
  const result = await inspectCutting(p, id);
  if (attestedKey !== cuttingTargetKey(p.data, id))
    throw new Error('Values changed while saving; confirm testing again');
  if (result.check.errors.length) throw new Error(result.check.errors.join('; '));
  if (!result.context)
    throw new Error(result.context_error ?? 'Complete machine, tool and material identities');
  return {
    id: crypto.randomUUID(),
    name: name.trim(),
    context: result.context,
    parameters: result.parameters,
    tested: true,
    test_date: null,
    notes: null,
  };
}
export function presetStatus(op: OpEntry, inspection: CuttingInspection | null): string {
  const baseline = op.cutting?.preset_snapshot;
  if (!baseline) return 'Manual parameters';
  const modified =
    !inspection ||
    inspection.context?.signature !== baseline.context.signature ||
    JSON.stringify(inspection.parameters) !== JSON.stringify(baseline.parameters);
  return `${modified ? 'Modified from preset' : 'Loaded preset'}: ${baseline.collection_name} — ${baseline.preset_name}`;
}
