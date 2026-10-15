import type { OperationTemplate, TemplateBinding } from './operation-templates';
import { instantiateTemplate } from './operation-templates';
import type { MsgKey } from '../i18n';

/** Local substring search only; never changes library order or template data. */
export function searchTemplates(entries: readonly OperationTemplate[], query: string) {
  const words = query.trim().toLocaleLowerCase().split(/\s+/).filter(Boolean);
  return entries.filter((entry) => {
    const text = [
      entry.name,
      entry.settings.kind.replaceAll('_', ' '),
      entry.tool.name,
      entry.finishTool?.name ?? '',
      String(entry.tool.diameter),
    ]
      .join(' ')
      .toLocaleLowerCase();
    return words.every((word) => text.includes(word));
  });
}

/** Same authoritative validation used at Create time, with no project mutation. */
export function templateBindingProblem(
  template: OperationTemplate | undefined,
  binding: TemplateBinding,
): MsgKey | null {
  if (!template) return 'templates.select_first';
  try {
    instantiateTemplate(template, binding);
    return null;
  } catch (e) {
    const key = e instanceof Error ? e.message : '';
    return key.startsWith('templates.') ? (key as MsgKey) : 'templates.invalid';
  }
}

export function templateToolDifferences(template: OperationTemplate, binding: TemplateBinding) {
  const pairs = [
    {
      role: 'rough' as const,
      source: template.tool,
      target: binding.tools.find((tool) => tool.id === binding.toolId),
    },
    ...(template.finishTool
      ? [
          {
            role: 'finish' as const,
            source: template.finishTool,
            target: binding.tools.find((tool) => tool.id === binding.finishToolId),
          },
        ]
      : []),
  ];
  return pairs
    .filter(({ source, target }) => target && source.diameter !== target.diameter)
    .map(({ role, source, target }) => ({
      role,
      sourceDiameter: source.diameter,
      targetDiameter: target!.diameter,
    }));
}

export interface TemplateParameterRow {
  label: MsgKey;
  value: string;
}
/** Display stored intent only. No default values or destination calculations. */
export function templateParameterRows(template: OperationTemplate): TemplateParameterRow[] {
  const s = template.settings;
  const rows: TemplateParameterRow[] = [];
  const add = (label: MsgKey, value: unknown, unit = '') => {
    if (value !== undefined && value !== null)
      rows.push({ label, value: `${value}${unit ? ` ${unit}` : ''}` });
  };
  if (s.kind === 'profile') add('templates.preview.offset', s.offset);
  if (s.kind === 'pocket') add('templates.preview.strategy', s.pocketStrategy);
  add('opprops.cut.final_depth', s.depth, 'mm');
  add('opprops.cut.start_depth', s.startDepth, 'mm');
  add('opprops.cut.step', s.step, 'mm');
  if (s.depthList && (s.depthList as number[]).length)
    add('opprops.cut.depth_list', (s.depthList as number[]).join(', '), 'mm');
  add('opprops.cut.through_depth', s.throughDepth, 'mm');
  const surface = s.surface as Record<string, unknown> | undefined;
  if (surface) {
    add('surface.stepover_mm', surface.stepover_mm, 'mm');
    add('surface.direction', surface.direction === 'along_x' ? 'X' : 'Y');
    add('surface.margin_mm', surface.margin_mm, 'mm');
    add('surface.overtravel_mm', surface.overtravel_mm, 'mm');
  }
  if (s.xyOverlap !== undefined) add('templates.preview.overlap', Number(s.xyOverlap) * 100, '%');
  const plunge = s.plunge as Record<string, unknown> | undefined;
  if (plunge) {
    add('templates.preview.entry', plunge.kind);
    add('templates.preview.entry_angle', plunge.angleDeg, '°');
    add('templates.preview.entry_radius', plunge.radiusMm, 'mm');
  }
  add('templates.preview.direction', s.cutDirection);
  add('opprops.cut.finish_step', s.finishStep, 'mm');
  add('opprops.cut.xy_finish_stock', s.finishXyAllowanceMm, 'mm');
  add('templates.preview.lead_in', s.leadInKind === 'off' ? undefined : s.leadIn, 'mm');
  add('templates.preview.lead_out', s.leadOutKind === 'off' ? undefined : s.leadOut, 'mm');
  const halfpipe = s.halfpipeProfile as Record<string, unknown> | undefined;
  if (halfpipe && s.pocketStrategy === 'halfpipe') {
    add('templates.preview.floor', halfpipe.kind);
    add('templates.preview.floor_radius', halfpipe.radiusMm, 'mm');
    add('templates.preview.floor_angle', halfpipe.includedAngleDeg, '°');
  }
  return rows;
}
