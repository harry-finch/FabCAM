import { describe, expect, it } from 'vitest';
import {
  captureTemplate,
  instantiateTemplate,
  type OperationTemplate,
  type TemplateBinding,
} from './operation-templates';
import {
  searchTemplates,
  templateBindingProblem,
  templateParameterRows,
  templateToolDifferences,
} from './operation-template-ui';
import { buildOpEntry } from './op_defaults';
import type { ToolEntry } from './project-types';
const tool: ToolEntry = {
  id: 1,
  name: 'Flat workshop',
  kind: 'endmill',
  diameter: 12,
  speed: 10000,
  feedRate: 1000,
  plungeRate: 500,
} as ToolEntry;
function make(
  kind: 'pocket' | 'profile' | 'planar_surface',
  name = 'Plywood finish',
): OperationTemplate {
  return captureTemplate(
    buildOpEntry(kind, {
      nextId: 1,
      tools: [tool],
      selectionIds: [99],
      objectMeta: [],
      reliefSources: [],
    }),
    [tool],
    name,
    `id-${kind}`,
  );
}
function target(): TemplateBinding {
  return {
    nextId: 42,
    tools: [{ ...tool, id: 7, diameter: 6, speed: 9000, feedRate: 200 }],
    toolId: 7,
    objectIds: [5],
    availableObjectIds: [5],
    mode: 'mill',
  };
}
describe('UX4.1 presentation without changing template semantics', () => {
  it('searches names/type/source tool with case-insensitive local terms and preserves order/data', () => {
    const entries = [
      make('pocket'),
      make('profile', 'Outside contour'),
      make('planar_surface', 'Spoilboard'),
    ];
    const before = JSON.stringify(entries);
    expect(searchTemplates(entries, '  ')).toEqual(entries);
    expect(searchTemplates(entries, 'PLYWOOD flat 12')).toEqual([entries[0]]);
    expect(searchTemplates(entries, 'PROFILE')).toEqual([entries[1]]);
    expect(searchTemplates(entries, 'planar surface')).toEqual([entries[2]]);
    expect(searchTemplates(entries, 'unknown')).toEqual([]);
    expect(searchTemplates([], 'pocket')).toEqual([]);
    expect(JSON.stringify(entries)).toBe(before);
  });
  it('explains absent template, incompatible mode, tool and geometry through authoritative validation', () => {
    const entry = make('pocket');
    expect(templateBindingProblem(undefined, target())).toBe('templates.select_first');
    expect(templateBindingProblem(entry, { ...target(), toolId: 0 })).toBe('templates.choose_tool');
    expect(templateBindingProblem(entry, { ...target(), objectIds: [] })).toBe(
      'templates.choose_geometry',
    );
    expect(templateBindingProblem(entry, { ...target(), objectIds: [99] })).toBe(
      'templates.choose_geometry',
    );
    expect(templateBindingProblem(entry, { ...target(), mode: 'plot' })).toBe(
      'templates.milling_only',
    );
    expect(
      templateBindingProblem(entry, { ...target(), tools: [{ ...tool, id: 7, diameter: -1 }] }),
    ).toBe('templates.choose_tool');
    expect(templateBindingProblem(entry, target())).toBeNull();
  });
  it('validates stock and distinct finish role before enabling Create', () => {
    const surface = make('planar_surface');
    expect(templateBindingProblem(surface, target())).toBe('templates.choose_stock');
    expect(
      templateBindingProblem(surface, {
        ...target(),
        stockZone: { minX: 20, minY: 30, maxX: 40, maxY: 50 },
      }),
    ).toBeNull();
    const pocket = make('pocket');
    pocket.finishTool = structuredClone(pocket.tool);
    expect(templateBindingProblem(pocket, target())).toBe('templates.choose_tool');
    expect(templateBindingProblem(pocket, { ...target(), finishToolId: 7 })).toBe(
      'templates.distinct_finish',
    );
  });
  it('reports only known cutter diameter differences without rewriting or certifying values', () => {
    const entry = make('pocket');
    const before = JSON.stringify(entry);
    expect(templateToolDifferences(entry, target())).toEqual([
      { role: 'rough', sourceDiameter: 12, targetDiameter: 6 },
    ]);
    expect(templateToolDifferences(entry, { ...target(), tools: [{ ...tool, id: 7 }] })).toEqual(
      [],
    );
    expect(templateToolDifferences(entry, { ...target(), toolId: 0 })).toEqual([]);
    expect(instantiateTemplate(entry, target()).appliedCutting?.slots[0]).toMatchObject({
      tool_id: 7,
      rpm: 10000,
      feed_mm_min: 1000,
      plunge_mm_min: 500,
    });
    expect(JSON.stringify(entry)).toBe(before);
  });
  it('previews stored depths, pocket overlap and entry intent without inventing defaults', () => {
    const entry = make('pocket');
    entry.settings.plunge = { kind: 'helix', angleDeg: 3, radiusMm: null };
    entry.settings.depthList = [-0.5, -1, -2];
    const rows = templateParameterRows(entry);
    expect(rows).toContainEqual({ label: 'opprops.cut.final_depth', value: '-2 mm' });
    expect(rows).toContainEqual({ label: 'opprops.cut.depth_list', value: '-0.5, -1, -2 mm' });
    expect(rows).toContainEqual({ label: 'templates.preview.overlap', value: '50 %' });
    expect(rows).toContainEqual({ label: 'templates.preview.entry', value: 'helix' });
    expect(rows.some((row) => row.label === 'templates.preview.entry_radius')).toBe(false);
    expect(rows.some((row) => row.label === 'surface.stepover_mm')).toBe(false);
  });
  it('limits surface/profile previews to their actual fields', () => {
    const surface = templateParameterRows(make('planar_surface'));
    expect(surface.some((row) => row.label === 'surface.stepover_mm')).toBe(true);
    expect(surface.some((row) => row.label === 'templates.preview.strategy')).toBe(false);
    const profile = templateParameterRows(make('profile'));
    expect(profile).toContainEqual({ label: 'templates.preview.offset', value: 'outside' });
    expect(profile.some((row) => row.label === 'templates.preview.overlap')).toBe(false);
  });
});
