import type { components } from '../api/generated';
import type { OpEntry } from './op_types';
import type { ToolEntry } from './project-types';
export type MainFinishing = components['schemas']['MainFinishing'];
export type PocketScope = components['schemas']['PocketFinishingScope'];
export type CuttingStage = components['schemas']['CuttingStage'];
export function usesStages(op: OpEntry): boolean {
  return op.kind === 'profile' || (op.kind === 'pocket' && op.pocketStrategy !== 'halfpipe');
}
/** Creation only. Missing authored fields stay missing, never substituted at generation. */
export function seedStages(op: OpEntry, tools: readonly ToolEntry[]): MainFinishing {
  const tool = tools.find((t) => t.id === op.toolId);
  const rate = (n: number | undefined) => (Number.isInteger(n) && n! > 0 ? n! : null);
  return {
    contract: op.kind === 'pocket' ? 'pocket_finishing_v2' : 'main_finishing_v1',
    main: {
      tool_id: op.toolId,
      rpm: rate(tool?.speed),
      feed_mm_min: rate(tool?.feedRate),
      plunge_mm_min: rate(tool?.plungeRate),
      depth_per_pass_mm: op.step == null ? null : Math.abs(op.step),
    },
    finishing: null,
  };
}
function object(
  value: unknown,
  keys: string[],
  required = keys,
): asserts value is Record<string, unknown> {
  if (
    !value ||
    typeof value !== 'object' ||
    Array.isArray(value) ||
    Object.keys(value).some((k) => !keys.includes(k)) ||
    required.some((k) => !(k in value))
  )
    throw new Error('Malformed Main/Finishing contract');
}
export function validateStages(value: unknown): asserts value is MainFinishing {
  object(value, ['contract', 'main', 'finishing']);
  if (!['main_finishing_v1', 'pocket_finishing_v2'].includes(String(value.contract)))
    throw new Error('Unsupported cutting stage contract');
  const stage = (s: unknown) => {
    object(s, ['tool_id', 'rpm', 'feed_mm_min', 'plunge_mm_min', 'depth_per_pass_mm']);
    if (!Number.isInteger(s.tool_id) || Number(s.tool_id) <= 0 || Number(s.tool_id) > 0xffffffff)
      throw new Error('Invalid stage tool identity');
    for (const field of ['rpm', 'feed_mm_min', 'plunge_mm_min']) {
      const n = s[field];
      if (n !== null && (typeof n !== 'number' || !Number.isInteger(n) || n < 0 || n > 0xffffffff))
        throw new Error(`Invalid stage ${field}`);
    }
    const depth = s.depth_per_pass_mm;
    if (depth !== null && (typeof depth !== 'number' || !Number.isFinite(depth) || depth <= 0))
      throw new Error('Invalid stage depth per pass');
  };
  stage(value.main);
  if (value.finishing !== null) {
    object(value.finishing, [
      'cutting',
      'lateral_allowance_mm',
      'floor_allowance_mm',
      ...(value.contract === 'pocket_finishing_v2' ? ['scope'] : []),
    ]);
    if (value.contract === 'pocket_finishing_v2') {
      const f = value.finishing;
      if (!['walls_only', 'floor_only', 'walls_and_floor'].includes(String(f.scope)))
        throw new Error('Unsupported Pocket finishing scope');
      if (
        (f.scope === 'walls_only' && f.floor_allowance_mm !== 0) ||
        (f.scope === 'floor_only' && f.lateral_allowance_mm !== 0)
      )
        throw new Error('Finishing scope contradicts its allowances');
    }
    stage(value.finishing.cutting);
    for (const field of ['lateral_allowance_mm', 'floor_allowance_mm']) {
      const n = value.finishing[field];
      if (typeof n !== 'number' || !Number.isFinite(n) || n < 0)
        throw new Error(`Invalid finishing ${field}`);
    }
  }
}
export function validateStageOperation(op: OpEntry): void {
  validateStages(op.stages);
  if (
    op.stages.contract === 'pocket_finishing_v2' &&
    (op.kind !== 'pocket' || op.pocketStrategy === 'halfpipe')
  )
    throw new Error('Pocket scope requires an ordinary Pocket operation');
  if (
    op.cutting != null ||
    op.step != null ||
    op.depthList?.length ||
    ('finishStep' in op && op.finishStep != null) ||
    ('finishXyAllowanceMm' in op && op.finishXyAllowanceMm != null)
  )
    throw new Error(`${op.name}: mixed legacy and stage cutting fields`);
  if (
    op.toolId !== op.stages.main.tool_id ||
    (op.finishToolId ?? null) !== (op.stages.finishing?.cutting.tool_id ?? null)
  )
    throw new Error(`${op.name}: stage and operation tools disagree`);
  if (
    op.retainedStockMm != null &&
    (!Number.isFinite(op.retainedStockMm) || op.retainedStockMm < 0)
  )
    throw new Error('Invalid retained-for-later stock');
}
/** Tool reassignment is explicit and atomic; rates never switch to library defaults. */
export function rebindStages(op: OpEntry, patch: Partial<OpEntry>): Partial<OpEntry> {
  if (patch.stages !== undefined) return patch;
  const next = { ...op, ...patch } as OpEntry;
  if (!op.stages) {
    if (!usesStages(op) && usesStages(next))
      return {
        ...patch,
        cutting: undefined,
        step: null,
        finishToolId: undefined,
        stages: {
          contract: next.kind === 'pocket' ? 'pocket_finishing_v2' : 'main_finishing_v1',
          main: {
            tool_id: next.toolId,
            rpm: null,
            feed_mm_min: null,
            plunge_mm_min: null,
            depth_per_pass_mm: null,
          },
          finishing: null,
        },
      } as Partial<OpEntry>;
    return patch;
  }
  if (!usesStages(next))
    return {
      ...patch,
      stages: undefined,
      stagePreset: undefined,
      cutting: {
        rates: [
          {
            role: 'rough',
            tool_id: next.toolId,
            rpm: null,
            feed_mm_min: null,
            plunge_mm_min: null,
          },
        ],
      },
      step: null,
      finishToolId: undefined,
    } as Partial<OpEntry>;
  if (op.stages.contract === 'pocket_finishing_v2' && next.kind !== 'pocket')
    throw new Error(
      'Create a separate Profile operation; Pocket finishing scope cannot be transferred automatically',
    );
  const stages = JSON.parse(JSON.stringify(op.stages)) as MainFinishing;
  if (patch.toolId !== undefined) stages.main.tool_id = patch.toolId;
  if (patch.finishToolId !== undefined && stages.finishing)
    stages.finishing.cutting.tool_id = patch.finishToolId;
  return { ...patch, stages };
}

/** Previewed scope change: caller shows discarded allowance before committing. */
export function withPocketScope(stages: MainFinishing, scope: PocketScope): MainFinishing {
  if (!stages.finishing) throw new Error('Enable finishing before choosing its scope');
  return {
    contract: 'pocket_finishing_v2',
    main: { ...stages.main },
    finishing: {
      ...stages.finishing,
      cutting: { ...stages.finishing.cutting },
      scope,
      lateral_allowance_mm: scope === 'floor_only' ? 0 : stages.finishing.lateral_allowance_mm,
      floor_allowance_mm: scope === 'walls_only' ? 0 : stages.finishing.floor_allowance_mm,
    },
  };
}
