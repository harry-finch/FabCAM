import type { OpEntry, ToolKind } from './op_types';
import type { ToolEntry } from './project-types';
import { operationRates } from './feeds-speeds';

export type TemplateKind = 'profile' | 'pocket' | 'planar_surface';
type Rates = { rpm: number; feed_mm_min: number; plunge_mm_min: number };
export interface TemplateTool {
  kind: ToolKind;
  diameter: number;
  name: string;
  rough: Rates;
  finish: Rates;
}
export interface OperationTemplate {
  version: 1;
  id: string;
  name: string;
  settings: Record<string, unknown> & { kind: TemplateKind };
  tool: TemplateTool;
  finishTool?: TemplateTool;
}
const COMMON = [
  'depth',
  'startDepth',
  'step',
  'sourceCombine',
  'plunge',
  'throughDepth',
  'depthList',
  'cornerFeedReduction',
];
const CONTOUR = [
  'cutDirection',
  'finishCutDirection',
  'leadInKind',
  'leadOutKind',
  'leadIn',
  'leadOut',
  'finishStep',
  'tabType',
  'tabRampAngleDeg',
  'tabWidth',
  'tabHeight',
  'tabsActive',
  'tabMode',
];
const POCKET = [
  'pocketStrategy',
  'xyOverlap',
  'engagementAngleDeg',
  'loopRadiusFactor',
  'pocketZigzagAngleDeg',
  'halfpipeProfile',
  'finishXyAllowanceMm',
  'frameShape',
  'framePaddingMm',
  'frameCornerRadiusMm',
];
const copy = <T>(value: T): T => JSON.parse(JSON.stringify(value));
export function templateKind(kind: string): kind is TemplateKind {
  return ['profile', 'pocket', 'planar_surface'].includes(kind);
}
function keys(kind: TemplateKind) {
  return [
    'kind',
    ...COMMON,
    ...(kind === 'planar_surface' ? ['surface'] : CONTOUR),
    ...(kind === 'profile' ? ['offset'] : kind === 'pocket' ? POCKET : []),
  ];
}
function rates(op: OpEntry, tool: ToolEntry, pass: 'rough' | 'finish'): Rates {
  const value = operationRates(op, tool, pass);
  if (
    !value ||
    ![value.rpm, value.feed_mm_min, value.plunge_mm_min].every(
      (v) => Number.isInteger(v) && v > 0 && v <= 0xffffffff,
    )
  )
    throw new Error('templates.invalid_rates');
  return { rpm: value.rpm, feed_mm_min: value.feed_mm_min, plunge_mm_min: value.plunge_mm_min };
}
export function captureTemplate(
  op: OpEntry,
  tools: readonly ToolEntry[],
  name: string,
  id: string,
): OperationTemplate {
  if (!templateKind(op.kind)) throw new Error('templates.unsupported');
  if (op.side === 'back') throw new Error('templates.front_only');
  if ('finishToolId' in op && op.finishToolId && op.finishToolId !== op.toolId && op.step === null)
    throw new Error('templates.explicit_step');
  if (
    op.pattern ||
    ('tabMode' in op && ['manual', 'mixed'].includes(op.tabMode?.kind ?? '')) ||
    ('tabPlacements' in op && op.tabPlacements?.length) ||
    ('approachPoint' in op && op.approachPoint)
  )
    throw new Error('templates.geometry_bound');
  const settings = Object.fromEntries(
    keys(op.kind)
      .filter((key) => key in op && (op as unknown as Record<string, unknown>)[key] !== undefined)
      .map((key) => [key, copy((op as unknown as Record<string, unknown>)[key])]),
  ) as OperationTemplate['settings'];
  if (op.kind === 'planar_surface') {
    // Zone is rebound explicitly to the destination stock, not reused by coordinate.
    settings.surface = {
      margin_mm: op.surface.margin_mm,
      overtravel_mm: op.surface.overtravel_mm,
      stepover_mm: op.surface.stepover_mm,
      direction: op.surface.direction,
    };
  }
  const requirement = (toolId: number): TemplateTool => {
    const tool = tools.find((tool) => tool.id === toolId);
    if (
      !tool ||
      tool.kind !== 'endmill' ||
      tool.whirl ||
      !Number.isFinite(tool.diameter) ||
      tool.diameter <= 0
    )
      throw new Error('templates.flat_tool');
    return {
      kind: tool.kind,
      diameter: tool.diameter,
      name: tool.name,
      rough: rates(op, tool, 'rough'),
      finish: rates(op, tool, 'finish'),
    };
  };
  settings.step = op.step ?? tools.find((tool) => tool.id === op.toolId)?.defaultStep ?? null;
  const result: OperationTemplate = {
    version: 1,
    id,
    name: name.trim(),
    settings,
    tool: requirement(op.toolId),
    ...('finishToolId' in op && op.finishToolId && op.finishToolId !== op.toolId
      ? { finishTool: requirement(op.finishToolId) }
      : {}),
  };
  if (!validTemplate(result)) throw new Error('templates.invalid');
  return result;
}
/** Strict local format: no source/tool IDs, geometry coordinates or knowledge blobs. */
export function validTemplate(value: unknown): value is OperationTemplate {
  if (!value || typeof value !== 'object') return false;
  const t = value as OperationTemplate;
  if (
    t.version !== 1 ||
    typeof t.id !== 'string' ||
    !t.id ||
    typeof t.name !== 'string' ||
    !t.name.trim() ||
    t.name.length > 100 ||
    !t.settings ||
    !templateKind(t.settings.kind)
  )
    return false;
  if (
    Object.keys(t).some(
      (k) => !['version', 'id', 'name', 'settings', 'tool', 'finishTool'].includes(k),
    )
  )
    return false;
  if (Object.keys(t.settings).some((key) => !keys(t.settings.kind).includes(key))) return false;
  const enums: Record<string, string[]> = {
    kind: ['profile', 'pocket', 'planar_surface'],
    offset: ['inside', 'outside', 'on'],
    sourceCombine: ['auto', 'union', 'difference', 'intersection', 'xor', 'none'],
    pocketStrategy: ['cascade', 'zigzag', 'spiral', 'trochoidal', 'halfpipe'],
    cutDirection: ['climb', 'conventional'],
    finishCutDirection: ['climb', 'conventional'],
    leadInKind: ['off', 'straight', 'arc'],
    leadOutKind: ['off', 'straight', 'arc'],
    tabType: ['rectangle', 'ramp'],
    frameShape: ['rectangle', 'rounded_rectangle'],
  };
  const nested: Record<string, string[]> = {
    plunge: ['kind', 'angleDeg', 'radiusMm'],
    tabMode: ['kind', 'count', 'autoCount'],
    halfpipeProfile: ['kind', 'radiusMm', 'includedAngleDeg'],
    surface: ['margin_mm', 'overtravel_mm', 'stepover_mm', 'direction'],
  };
  for (const [key, value] of Object.entries(t.settings)) {
    if (enums[key]) {
      if (!(key === 'pocketStrategy' && value === null) && !enums[key].includes(String(value)))
        return false;
    } else if (key === 'tabsActive') {
      if (typeof value !== 'boolean') return false;
    } else if (key === 'depthList') {
      if (
        !Array.isArray(value) ||
        !value.every((v) => typeof v === 'number' && Number.isFinite(v) && v < 0)
      )
        return false;
    } else if (nested[key]) {
      if (
        !value ||
        typeof value !== 'object' ||
        Array.isArray(value) ||
        Object.keys(value).some((k) => !nested[key].includes(k))
      )
        return false;
      const obj = value as Record<string, unknown>;
      if (
        Object.entries(obj).some(
          ([k, v]) =>
            k !== 'kind' &&
            k !== 'direction' &&
            !(k === 'radiusMm' && v === null) &&
            (typeof v !== 'number' || !Number.isFinite(v)),
        )
      )
        return false;
      if (key === 'plunge' && !['direct', 'ramp', 'helix'].includes(String(obj.kind))) return false;
      if (key === 'tabMode' && !['off', 'auto'].includes(String(obj.kind))) return false;
      if (key === 'halfpipeProfile' && !['circular_arc', 'v_bottom'].includes(String(obj.kind)))
        return false;
    } else if (typeof value !== 'number' || !Number.isFinite(value)) return false;
  }
  const finiteTree = (v: unknown): boolean =>
    typeof v === 'number'
      ? Number.isFinite(v)
      : Array.isArray(v)
        ? v.every(finiteTree)
        : v !== null && typeof v === 'object'
          ? Object.values(v).every(finiteTree)
          : ['string', 'boolean'].includes(typeof v) || v === null;
  if (
    !finiteTree(t.settings) ||
    typeof t.settings.depth !== 'number' ||
    t.settings.depth >= 0 ||
    typeof t.settings.startDepth !== 'number' ||
    t.settings.startDepth < t.settings.depth ||
    typeof t.settings.step !== 'number' ||
    t.settings.step >= 0
  )
    return false;
  if (
    t.settings.kind === 'profile' &&
    !['inside', 'outside', 'on'].includes(String(t.settings.offset))
  )
    return false;
  const safeTool = (tool: TemplateTool) =>
    tool &&
    tool.kind === 'endmill' &&
    Number.isFinite(tool.diameter) &&
    tool.diameter > 0 &&
    typeof tool.name === 'string' &&
    Object.keys(tool).every((k) => ['kind', 'diameter', 'name', 'rough', 'finish'].includes(k)) &&
    ['rough', 'finish'].every((pass) => {
      const r = tool[pass as 'rough' | 'finish'];
      return (
        r &&
        Object.keys(r).length === 3 &&
        ['rpm', 'feed_mm_min', 'plunge_mm_min'].every((key) => {
          const v = r[key as keyof Rates];
          return Number.isInteger(v) && v > 0 && v <= 0xffffffff;
        })
      );
    });
  if (
    !safeTool(t.tool) ||
    (t.finishTool && (!safeTool(t.finishTool) || t.settings.kind !== 'pocket'))
  )
    return false;
  if (t.settings.kind === 'planar_surface') {
    const p = t.settings.surface as Record<string, unknown>;
    if (
      !p ||
      Object.keys(p).some(
        (k) => !['margin_mm', 'overtravel_mm', 'stepover_mm', 'direction'].includes(k),
      ) ||
      !['along_x', 'along_y'].includes(String(p.direction)) ||
      !(Number(p.stepover_mm) > 0) ||
      !(Number(p.margin_mm) >= 0) ||
      !(Number(p.overtravel_mm) >= 0)
    )
      return false;
  }
  return true;
}
export interface TemplateBinding {
  nextId: number;
  toolId: number;
  finishToolId?: number;
  tools: readonly ToolEntry[];
  objectIds: number[];
  availableObjectIds: number[];
  mode: string;
  stockZone?: { minX: number; minY: number; maxX: number; maxY: number };
}
export function instantiateTemplate(
  template: OperationTemplate,
  binding: TemplateBinding,
): OpEntry {
  if (!validTemplate(template)) throw new Error('templates.invalid');
  if (binding.mode !== 'mill') throw new Error('templates.milling_only');
  const resolve = (id: number | undefined, required: TemplateTool) => {
    const tool = binding.tools.find((tool) => tool.id === id);
    if (
      !tool ||
      tool.kind !== required.kind ||
      tool.whirl ||
      !Number.isFinite(tool.diameter) ||
      tool.diameter <= 0 ||
      !Number.isFinite(tool.wearOffsetMm ?? 0) ||
      tool.diameter - (tool.wearOffsetMm ?? 0) <= 0
    )
      throw new Error('templates.choose_tool');
    return tool;
  };
  const tool = resolve(binding.toolId, template.tool);
  const finish = template.finishTool ? resolve(binding.finishToolId, template.finishTool) : null;
  if (finish && finish.id === tool.id) throw new Error('templates.distinct_finish');
  const ids = [...new Set(binding.objectIds)];
  const settings = copy(template.settings);
  if (settings.kind === 'planar_surface') {
    const b = binding.stockZone;
    if (!b || !Object.values(b).every(Number.isFinite) || b.maxX <= b.minX || b.maxY <= b.minY)
      throw new Error('templates.choose_stock');
    settings.surface = {
      ...(settings.surface as object),
      origin: [b.minX, b.minY],
      width_mm: b.maxX - b.minX,
      height_mm: b.maxY - b.minY,
    };
  } else if (!ids.length || ids.some((id) => !binding.availableObjectIds.includes(id)))
    throw new Error('templates.choose_geometry');
  const slots = [[tool, template.tool], ...(finish ? [[finish, template.finishTool!]] : [])] as [
    ToolEntry,
    TemplateTool,
  ][];
  return {
    ...settings,
    id: binding.nextId,
    name: template.name,
    enabled: true,
    toolId: tool.id,
    ...(finish ? { finishToolId: finish.id } : {}),
    sourceLayers: null,
    ...(settings.kind === 'planar_surface' ? {} : { sourceObjects: ids }),
    side: 'front',
    appliedCutting: {
      context_key: '',
      doc_origin: 'manual',
      slots: slots.flatMap(([tool, r]) =>
        (['rough', 'finish'] as const).map((pass) => ({
          tool_id: tool.id,
          pass,
          ...copy(r[pass]),
          origins: { rpm: 'manual', feed: 'manual', plunge: 'manual' },
        })),
      ),
    },
  } as OpEntry;
}
