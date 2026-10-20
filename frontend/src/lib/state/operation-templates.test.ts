import { describe, expect, it, vi } from 'vitest';
import {
  captureTemplate,
  instantiateTemplate,
  validTemplate,
  type TemplateBinding,
} from './operation-templates';
import { buildOpEntry } from './op_defaults';
import { buildProject } from '../api/build-project';
import { History } from './history';
import { addOperationCommand } from './commands';
import { WorkspaceStore, parseWorkspace, DEFAULT_WORKSPACE } from './workspace';
import type { OpEntry } from './op_types';
import type { ToolEntry } from './project-types';
const sourceTool = {
  id: 1,
  name: 'Source flat',
  kind: 'endmill',
  diameter: 6,
  speed: 18000,
  feedRate: 1200,
  plungeRate: 200,
  defaultStep: -0.5,
} as ToolEntry;
const targetTool = {
  ...sourceTool,
  id: 7,
  name: 'Target flat',
  diameter: 8,
  speed: 9000,
  feedRate: 500,
  defaultStep: -2,
};
function source(kind: 'profile' | 'pocket' | 'planar_surface' = 'pocket'): OpEntry {
  return buildOpEntry(kind, {
    nextId: 1,
    tools: [sourceTool],
    selectionIds: [99],
    objectMeta: [],
    reliefSources: [],
  });
}
function binding(): TemplateBinding {
  return {
    nextId: 42,
    toolId: 7,
    tools: [targetTool],
    objectIds: [5],
    availableObjectIds: [5, 6],
    mode: 'mill',
  };
}
describe('UX4 reusable operation templates', () => {
  it.each(['profile', 'pocket', 'planar_surface'] as const)(
    'captures %s and explicitly rebinds without copying source identity',
    (kind) => {
      const op = source(kind);
      op.step = null;
      const template = captureTemplate(op, [sourceTool], 'Shop model', 'template-1');
      const serialized = JSON.stringify(template);
      for (const key of [
        'tool_id',
        'sourceObjects',
        'sourceLayers',
        'appliedCutting',
        'knowledge',
        'proposal',
        'context_key',
        'origin',
        'width_mm',
        'height_mm',
      ])
        expect(serialized).not.toContain(`"${key}"`);
      const next = instantiateTemplate(template, {
        ...binding(),
        stockZone: { minX: 30, minY: 40, maxX: 70, maxY: 80 },
      });
      expect(next).toMatchObject({ id: 42, toolId: 7, step: -0.5 });
      expect(next.appliedCutting?.slots[0]).toMatchObject({
        tool_id: 7,
        rpm: 18000,
        feed_mm_min: 1200,
        plunge_mm_min: 200,
      });
      expect(next.appliedCutting?.proposal).toBeUndefined();
      if (next.kind === 'planar_surface')
        expect(next.surface).toMatchObject({ origin: [30, 40], width_mm: 40, height_mm: 40 });
      else expect(next.sourceObjects).toEqual([5]);
    },
  );
  it('preserves actual manual overrides without exporting private recommendation evidence', () => {
    const op = source();
    op.feedRateOverride = 2222;
    op.plungeRateOverride = 333;
    const template = captureTemplate(op, [sourceTool], 'Manual', 't');
    expect(template.tool.rough).toMatchObject({ feed_mm_min: 2222, plunge_mm_min: 333 });
    const next = instantiateTemplate(template, binding());
    expect(next.appliedCutting?.slots.every((slot) => slot.origins.rpm === 'manual')).toBe(true);
    template.tool.rough.rpm = 1;
    expect(next.appliedCutting?.slots[0].rpm).toBe(18000);
    expect(op.appliedCutting).toBeUndefined();
  });
  it('requires all destination choices, current IDs, milling mode and compatible physical tool', () => {
    const template = captureTemplate(source(), [sourceTool], 'Pocket', 't');
    for (const patch of [
      { objectIds: [] },
      { objectIds: [99] },
      { toolId: 1 },
      { mode: 'laser' },
      { tools: [{ ...targetTool, kind: 'ball_nose' as const }] },
      { tools: [{ ...targetTool, wearOffsetMm: 10 }] },
    ])
      expect(() => instantiateTemplate(template, { ...binding(), ...patch })).toThrow();
    const plane = captureTemplate(source('planar_surface'), [sourceTool], 'Plane', 'p');
    expect(() => instantiateTemplate(plane, binding())).toThrow('templates.choose_stock');
  });
  it('does not collapse a distinct finish tool or use inherited pass depth in a dual-tool model', () => {
    const op = source();
    if (op.kind !== 'pocket') throw new Error();
    op.finishToolId = 2;
    const finish = { ...sourceTool, id: 2, diameter: 3, feedRateFinish: 900 };
    const template = captureTemplate(op, [sourceTool, finish], 'Dual', 't');
    expect(() => instantiateTemplate(template, { ...binding(), finishToolId: 7 })).toThrow(
      'templates.distinct_finish',
    );
    const next = instantiateTemplate(template, {
      ...binding(),
      finishToolId: 8,
      tools: [targetTool, { ...finish, id: 8 }],
    });
    expect(next).toMatchObject({ toolId: 7, finishToolId: 8 });
    expect(
      next.appliedCutting?.slots.find((s) => s.tool_id === 8 && s.pass === 'finish')?.feed_mm_min,
    ).toBe(900);
    op.step = null;
    expect(() => captureTemplate(op, [sourceTool, finish], 'Dual', 't')).toThrow(
      'templates.explicit_step',
    );
  });
  it('refuses geometry-local markers, unsupported kinds and invalid settings', () => {
    const op = source();
    expect(() =>
      captureTemplate({ ...op, approachPoint: [1, 2] } as OpEntry, [sourceTool], 'x', 't'),
    ).toThrow('templates.geometry_bound');
    expect(() =>
      captureTemplate({ ...op, tabMode: { kind: 'manual' } } as OpEntry, [sourceTool], 'x', 't'),
    ).toThrow('templates.geometry_bound');
    expect(() => captureTemplate({ ...op, side: 'back' }, [sourceTool], 'x', 't')).toThrow(
      'templates.front_only',
    );
    expect(() =>
      captureTemplate(
        { ...op, kind: 'gcode_include' } as unknown as OpEntry,
        [sourceTool],
        'x',
        't',
      ),
    ).toThrow('templates.unsupported');
    expect(() => captureTemplate({ ...op, depth: NaN }, [sourceTool], 'x', 't')).toThrow();
    expect(() => captureTemplate(op, [sourceTool], ' ', 't')).toThrow();
  });
  it('loads legacy workspaces, filters malformed/duplicate models and roundtrips independent clones', async () => {
    let blob: string | null = null;
    const transport = {
      read: async () => blob,
      write: async (json: string) => {
        blob = json;
      },
    };
    const store = new WorkspaceStore(transport);
    await store.load();
    expect(store.get().operation_templates).toEqual([]);
    const template = captureTemplate(source(), [sourceTool], 'Pocket', 't');
    store.upsertOperationTemplate(template);
    template.name = 'mutated';
    await store.save();
    const reopened = new WorkspaceStore(transport);
    await reopened.load();
    expect(reopened.get().operation_templates[0].name).toBe('Pocket');
    reopened.removeOperationTemplate('t');
    await reopened.save();
    expect(JSON.parse(blob!).operation_templates).toEqual([]);
    const invalid = { ...template, settings: { ...template.settings, sourceObjects: [99] } };
    expect(validTemplate(invalid)).toBe(false);
    const parsed = parseWorkspace(
      JSON.stringify({ ...DEFAULT_WORKSPACE, operation_templates: [invalid, template, template] }),
    );
    expect(parsed.operation_templates).toHaveLength(1);
  });
  it('adds one independent undoable operation and serializes retained cutting values', () => {
    const template = captureTemplate(source(), [sourceTool], 'Pocket', 't');
    const op = instantiateTemplate(template, binding());
    const state = { operations: [] as OpEntry[], dirty: false };
    const h = new History();
    h.exec(addOperationCommand(op), state);
    expect(state.operations).toHaveLength(1);
    h.undo(state);
    expect(state.operations).toEqual([]);
    h.redo(state);
    const wire = buildProject({
      transformedImport: {
        filename: 'fixture.svg',
        format: 'svg',
        unit_scale: 1,
        warnings: [],
        unit_status: 'unknown',
        unit_source: '',
        segments: [],
        objects: [],
        object_meta: [],
        layers: [],
        bbox: { min_x: 0, min_y: 0, max_x: 40, max_y: 40 },
      } as Parameters<typeof buildProject>[0]['transformedImport'],
      tools: [targetTool],
      operations: state.operations,
      machine: { mode: 'mill', unit: 'mm', fastMoveZ: 5 } as Parameters<
        typeof buildProject
      >[0]['machine'],
    });
    expect(wire?.operations[0].source).toEqual({ kind: 'objects', ids: [5] });
    expect(wire?.operations[0].params.applied_cutting?.slots[0].feed_mm_min).toBe(1200);
    expect(JSON.stringify(wire)).not.toContain('template-1');
  });
  it('serializes overlapping persistence writes and exposes storage failures', async () => {
    const writes: string[] = [];
    let release: () => void = () => {};
    const transport = {
      read: async () => null,
      write: async (json: string) => {
        if (!writes.length) await new Promise<void>((r) => (release = r));
        writes.push(json);
      },
    };
    const store = new WorkspaceStore(transport);
    await store.load();
    store.update({ last_post_processor: 'grbl' });
    const first = store.save();
    await Promise.resolve();
    await Promise.resolve();
    store.update({ last_post_processor: 'cps' });
    const second = store.save();
    release();
    await Promise.all([first, second]);
    expect(JSON.parse(writes[1]).last_post_processor).toBe('cps');
    const failed = new WorkspaceStore({
      read: async () => null,
      write: async () => {
        throw new Error('quota');
      },
    });
    await failed.load();
    await expect(failed.save()).rejects.toThrow('quota');
  });
});

it('handles debounced storage failures while explicit saves still reject', async () => {
  vi.useFakeTimers();
  const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});
  try {
    const store = new WorkspaceStore({
      read: async () => null,
      write: async () => {
        throw new Error('quota');
      },
    });
    await store.load();
    store.upsertOperationTemplate(captureTemplate(source(), [sourceTool], 'Local', 'local'));
    await vi.advanceTimersByTimeAsync(1000);
    expect(warn).toHaveBeenCalledWith('Workspace persistence failed');
    await expect(store.save()).rejects.toThrow('quota');
  } finally {
    warn.mockRestore();
    vi.useRealTimers();
  }
});
