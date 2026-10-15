import type {
  BBox,
  CornerReliefRequest,
  CornerReliefResponse,
  ImportResponse,
  Segment,
} from '../api/types';
import type { ImportEntry, ToolEntry } from './project-types';
import type { OpEntry } from './op_types';
import type { Command } from './history';

export interface CornerReliefModifier {
  version: 1;
  sourceKey: string;
  toolId: number;
  diameter: number;
  recipe: Pick<
    CornerReliefRequest,
    'mode' | 'removal_side' | 'selected_objects' | 'selected_corners'
  >;
  evaluated: ImportResponse;
}
export function reliefSourceKey(imports: readonly ImportEntry[]): string {
  return JSON.stringify(
    imports.map(({ id, source, fileTransform }) => ({ id, source, fileTransform })),
  );
}
export function effectiveDiameter(tool: ToolEntry): number {
  return tool.diameter - (tool.wearOffsetMm ?? 0);
}
export function reliefError(
  modifier: CornerReliefModifier | undefined,
  imports: readonly ImportEntry[],
  tools: ToolEntry[],
): string | null {
  if (!modifier) return null;
  if (modifier.sourceKey !== reliefSourceKey(imports))
    return 'Drawing changed — preview and reapply corner relief.';
  const tool = tools.find((t) => t.id === modifier.toolId);
  if (!tool || tool.kind !== 'endmill' || effectiveDiameter(tool) !== modifier.diameter)
    return 'Cutter changed — preview and reapply corner relief.';
  return null;
}
export function reliefOperationError(
  modifier: CornerReliefModifier | undefined,
  operations: OpEntry[],
): string | null {
  if (!modifier) return null;
  const markerProblem = reliefMarkerError(modifier, operations);
  if (markerProblem) return markerProblem;
  for (const op of operations.filter((o) => o.enabled && o.kind !== 'planar_surface')) {
    const touches = reliefTouches(modifier, op);
    if (!touches) continue;
    if (op.pattern) return `${op.name}: patterned relief is not yet qualified.`;
    if (op.toolId !== modifier.toolId)
      return `${op.name}: select the corner-relief cutter or reapply the relief.`;
    if (op.kind !== 'profile' && op.kind !== 'pocket')
      return `${op.name}: corner relief is qualified for Profile/Pocket only.`;
    if (
      op.kind === 'profile' &&
      op.offset !== (modifier.recipe.removal_side === 'interior' ? 'inside' : 'outside')
    )
      return `${op.name}: contour offset must match the relief removal side.`;
    if (
      op.kind === 'pocket' &&
      ((op.pocketStrategy && op.pocketStrategy !== 'cascade') ||
        op.finishToolId ||
        op.finishXyAllowanceMm ||
        op.frameShape)
    )
      return `${op.name}: relief requires a single-cutter Cascade pocket without allowance or synthetic frame.`;
    if (op.tabsActive || (op.tabPlacements?.length ?? 0) > 0 || op.approachPoint)
      return `${op.name}: remove authored tabs/approach point before changing the contour.`;
  }
  return null;
}
export function reliefCommand(
  before: CornerReliefModifier | undefined,
  after: CornerReliefModifier | undefined,
): Command {
  const copy = (v: CornerReliefModifier | undefined) =>
    v ? (JSON.parse(JSON.stringify(v)) as CornerReliefModifier) : undefined;
  const old = copy(before),
    next = copy(after);
  return {
    label: after ? 'Apply corner relief' : 'Remove corner relief',
    apply(s) {
      (s as { cornerRelief?: CornerReliefModifier }).cornerRelief = copy(next);
    },
    revert(s) {
      (s as { cornerRelief?: CornerReliefModifier }).cornerRelief = copy(old);
    },
  };
}
// Exact circular extrema; endpoint-only bounds crop relief lobes and mis-size stock.
export function reliefBounds(segments: readonly Segment[]): BBox {
  const points = segments.flatMap((s) => {
    const p = [s.start, s.end];
    if (s.type === 'LINE' || !s.bulge) return p;
    const dx = s.end.x - s.start.x,
      dy = s.end.y - s.start.y,
      b = s.bulge;
    const cx = (s.start.x + s.end.x) / 2 - (dy * (1 - b * b)) / (4 * b),
      cy = (s.start.y + s.end.y) / 2 + (dx * (1 - b * b)) / (4 * b);
    const r = Math.hypot(s.start.x - cx, s.start.y - cy),
      a = Math.atan2(s.start.y - cy, s.start.x - cx),
      sweep = 4 * Math.atan(b),
      tau = 2 * Math.PI;
    for (const angle of [0, Math.PI / 2, Math.PI, (3 * Math.PI) / 2]) {
      const delta = (((sweep > 0 ? angle - a : a - angle) % tau) + tau) % tau;
      if (delta <= Math.abs(sweep) + 1e-10)
        p.push({ x: cx + r * Math.cos(angle), y: cy + r * Math.sin(angle) });
    }
    return p;
  });
  return {
    min_x: Math.min(...points.map((p) => p.x)),
    min_y: Math.min(...points.map((p) => p.y)),
    max_x: Math.max(...points.map((p) => p.x)),
    max_y: Math.max(...points.map((p) => p.y)),
  };
}
export function evaluatedImport(
  source: ImportResponse,
  response: CornerReliefResponse,
): ImportResponse {
  if (response.error) throw new Error(response.error);
  return {
    ...source,
    segments: response.segments,
    objects: response.objects,
    bbox: reliefBounds(response.segments),
    layers: source.layers.map((l) => ({
      ...l,
      segment_count: response.segments.filter((s) => s.layer === l.name).length,
    })),
    object_meta: source.object_meta?.map((m) => ({
      ...m,
      bbox: reliefBounds(response.segments.filter((_, i) => response.objects[i] === m.id)),
    })),
  };
}

function reliefTouches(modifier: CornerReliefModifier, op: OpEntry): boolean {
  const ids = modifier.recipe.selected_objects;
  const layers = new Set(
    modifier.evaluated.segments
      .filter((_, i) => ids.includes(modifier.evaluated.objects?.[i] ?? 0))
      .map((s) => s.layer),
  );
  const touches = op.sourceObjects?.length
    ? op.sourceObjects.some((id) => ids.includes(id))
    : !op.sourceLayers?.length || op.sourceLayers.some((l) => layers.has(l));
  return !!touches;
}
export function reliefMarkerError(
  modifier: CornerReliefModifier | undefined,
  operations: OpEntry[],
): string | null {
  if (!modifier) return null;
  for (const op of operations) {
    if (
      reliefTouches(modifier, op) &&
      (op.kind === 'profile' || op.kind === 'pocket') &&
      (op.tabsActive || (op.tabPlacements?.length ?? 0) > 0 || op.approachPoint)
    )
      return `${op.name}: remove authored tabs/approach point before changing the contour.`;
  }
  return null;
}
