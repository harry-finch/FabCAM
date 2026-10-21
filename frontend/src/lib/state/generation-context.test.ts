import { describe, expect, it } from 'vitest';
import { generationContextKey } from './generation-context';
import type { ProjectState } from './project.svelte';
import { buildOpEntry } from './op_defaults';

function fixture() {
  const op = buildOpEntry('profile', {
    nextId: 1,
    tools: [],
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
  const imported = {
    bbox: { min_x: 0, min_y: 0, max_x: 10, max_y: 10 },
    filename: 'software.svg',
    format: 'svg',
    layers: [],
    object_meta: [],
    objects: [],
    segments: [],
    unit_scale: 1,
    warnings: [],
  };
  return {
    data: {
      machine: {
        unit: 'mm',
        mode: 'mill',
        fastMoveZ: 5,
        cpsPost: { source: 'bundled', bundledId: 'shopbot', properties: {} },
      },
      tools: [],
      operations: [op],
      fixtures: [],
      textLayers: [],
      reliefSources: [],
      groupOpsByTool: false,
      dirty: false,
    },
    transformedImport: imported,
    stockSizingImport: imported,
  } as unknown as ProjectState;
}

describe('generated program identity is independent of file saving', () => {
  it('saving after an entry edit cannot revive an old program; undo can restore inputs', () => {
    const p = fixture();
    const before = generationContextKey(p, 'cps');
    const op = p.data.operations[0];
    const old = op.plunge;
    op.plunge = { kind: 'helix', angleDeg: 3, radiusMm: 5 };
    p.data.dirty = true;
    expect(generationContextKey(p, 'cps')).not.toBe(before);
    p.data.dirty = false; // saveProject resets file dirtiness
    expect(generationContextKey(p, 'cps')).not.toBe(before);
    op.plunge = old;
    expect(generationContextKey(p, 'cps')).toBe(before);
  });
  it('tracks post properties and authored origin, but excludes view and save state', () => {
    const p = fixture();
    const before = generationContextKey(p, 'cps');
    p.playhead = 0.2;
    p.savedToProject = true;
    expect(generationContextKey(p, 'cps')).toBe(before);
    p.data.workOffset = { x_mm: 5, y_mm: 0, z_mm: 0, wcs: 'G54' };
    expect(generationContextKey(p, 'cps')).not.toBe(before);
    p.data.workOffset = undefined as unknown as typeof p.data.workOffset;
    p.data.machine.cpsPost!.properties = { tolerance: 0.05 };
    expect(generationContextKey(p, 'cps')).not.toBe(before);
    expect(generationContextKey(p, 'grbl')).not.toBe(generationContextKey(p, 'cps'));
  });
});

it('V-Carve tool geometry invalidates output across save, undo and redo without changing cutting rates', async () => {
  const { History } = await import('./history');
  const { replaceToolsCommand } = await import('./commands');
  const p = fixture();
  p.data.operations = [
    buildOpEntry('vcarve', {
      nextId: 1,
      tools: [],
      reliefSources: [],
      selectionIds: [],
      objectMeta: [],
    }),
  ];
  p.data.operations[0].toolId = 1;
  p.data.tools = [
    {
      id: 1,
      name: 'Software V-bit',
      kind: 'v_bit',
      diameter: 6,
      tipDiameter: 0.5,
      tipAngleDeg: 60,
      flutes: 1,
      speed: 10000,
      plungeRate: 200,
      feedRate: 600,
      coolant: 'off',
    },
  ];
  const key = generationContextKey(p, 'cps');
  const history = new History();
  history.exec(replaceToolsCommand([{ ...p.data.tools[0], tipAngleDeg: 90 }]), p.data);
  const changed = generationContextKey(p, 'cps');
  expect(changed).not.toBe(key);
  p.data.dirty = false;
  expect(generationContextKey(p, 'cps')).toBe(changed);
  history.undo(p.data);
  expect(generationContextKey(p, 'cps')).toBe(key);
  history.redo(p.data);
  expect(generationContextKey(p, 'cps')).toBe(changed);
  expect(p.data.tools[0]).toMatchObject({ feedRate: 600, plungeRate: 200, speed: 10000 });
  p.data.tools[0].tipDiameter = 1;
  expect(generationContextKey(p, 'cps')).not.toBe(changed);
});

it('shared preset application invalidates generation identity and Undo/Redo restores it without setup substitution', async () => {
  const { applyCuttingCommand } = await import('./commands');
  const p = fixture();
  p.data.tools = [
    {
      id: 1,
      name: 'Software cutter',
      kind: 'endmill',
      diameter: 6.35,
      flutes: 1,
      speed: 17000,
      feedRate: 1000,
      plungeRate: 500,
      coolant: 'off',
    },
  ];
  p.data.operations[0].toolId = 1;
  const old = generationContextKey(p, 'cps');
  const tools = JSON.stringify(p.data.tools),
    machine = JSON.stringify(p.data.machine);
  const command = applyCuttingCommand(
    p.data.operations[0].id,
    {
      step: -2.8,
      appliedCutting: {
        slots: [
          {
            tool_id: 1,
            pass: 'finish',
            rpm: 17000,
            feed_mm_min: 1620,
            plunge_mm_min: 810,
            origins: { rpm: 'recommendation', feed: 'recommendation', plunge: 'recommendation' },
          },
        ],
        context_key: 'software',
        doc_origin: 'recommendation',
      },
    },
    p.data.material,
    p.data.fsContext,
  );
  command.apply(p.data);
  const applied = generationContextKey(p, 'cps');
  expect(applied).not.toBe(old);
  p.data.dirty = false;
  expect(generationContextKey(p, 'cps')).toBe(applied);
  command.revert(p.data);
  expect(generationContextKey(p, 'cps')).toBe(old);
  command.apply(p.data);
  expect(generationContextKey(p, 'cps')).toBe(applied);
  expect(JSON.stringify(p.data.tools)).toBe(tools);
  expect(JSON.stringify(p.data.machine)).toBe(machine);
});

it('MAT-1 browsing is read-only; explicit material choice changes freshness and remains undoable', async () => {
  const { materialOptions, filterMaterials } = await import('./material-presentation');
  const { setMaterialCommand } = await import('./commands');
  const p = fixture();
  p.data.material = { id: 'legacy-private-stock', revision: '7', label: 'Original name' };
  const original = JSON.stringify(p.data);
  const key = generationContextKey(p, 'cps');
  const options = materialOptions([], p.data.material);
  filterMaterials(options, 'acrylic', 'plastic');
  expect(JSON.stringify(p.data)).toBe(original);
  expect(generationContextKey(p, 'cps')).toBe(key);
  const choice = options.find((m) => m.identity.id === 'material/plastic.pmma.cast')!;
  const command = setMaterialCommand({ ...choice.identity, label: choice.labels[0] });
  command.apply(p.data);
  const changed = generationContextKey(p, 'cps');
  expect(changed).not.toBe(key);
  command.revert(p.data);
  expect(p.data.material).toEqual(JSON.parse(original).material);
  expect(generationContextKey(p, 'cps')).toBe(key);
  command.apply(p.data);
  expect(generationContextKey(p, 'cps')).toBe(changed);
});
