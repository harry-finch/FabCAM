import { describe, expect, it } from 'vitest';
import { buildOpEntry } from './op_defaults';
import type { OpEntry } from './op_types';
import { identityFileTransform, type ImportEntry } from './project-types';
import { History } from './history';
import { updateOperationCommand } from './commands';
import { buildProject } from '../api/build-project';
import { combineImports } from './file-transform';
import { opIncludesObject } from './op_source';
import {
  effectiveSourceMode,
  sourceModePatch,
  sourceModeLabels,
  combineChoices,
  canInterpretContours,
  contourHelp,
  layerProvenance,
} from './geometry-source-ui';

const fixture = (kind: OpEntry['kind'] = 'pocket') =>
  buildOpEntry(kind, {
    nextId: 1,
    tools: [],
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
const entry = (filename: string, format: string, layer: string, id: number): ImportEntry => ({
  id,
  fileTransform: identityFileTransform(),
  source: {
    filename,
    format,
    segments: [
      { type: 'LINE', bulge: 0, color: 7, layer, start: { x: 0, y: 0 }, end: { x: 10, y: 0 } },
    ],
    objects: [1],
    object_meta: [],
    layers: [{ name: layer, color: 7, segment_count: 1 }],
    bbox: { min_x: 0, min_y: 0, max_x: 10, max_y: 0 },
    unit_scale: 1,
    warnings: [],
    unit_status: 'unknown',
    unit_source: '',
  },
});
function wire(op: OpEntry) {
  return buildProject({
    transformedImport: entry('drawing.svg', 'svg', 'svg', 1).source,
    operations: [op],
    tools: [],
    machine: {
      unit: 'mm',
      mode: 'mill',
      comments: true,
      arcs: true,
      fastMoveZ: 5,
      toolchangeStrategy: 'manual_m0_pause',
    },
  })!.operations[0].source;
}

describe('geometry source presentation and persisted selection', () => {
  it('keeps all three labels and all six existing enum values', () => {
    expect(sourceModeLabels).toEqual({
      all: 'opprops.source.mode.all',
      layers: 'opprops.source.mode.layer',
      objects: 'opprops.source.mode.objects',
    });
    expect(combineChoices).toEqual(['auto', 'union', 'difference', 'intersection', 'xor', 'none']);
  });
  it('reports the actual empty-array fallback and object precedence', () => {
    for (const op of [fixture(), { ...fixture(), sourceLayers: [], sourceObjects: [] }]) {
      expect(effectiveSourceMode(op)).toBe('all');
      expect(wire(op)).toEqual({ kind: 'all' });
    }
    const op = { ...fixture(), sourceLayers: ['Cuts'], sourceObjects: [3, 1] };
    expect(effectiveSourceMode(op)).toBe('objects');
    expect(wire(op)).toEqual({ kind: 'objects', ids: [3, 1] });
  });
  it('stages unbound modes without clearing authored selection or changing the wire', () => {
    const op = { ...fixture(), sourceLayers: ['Cuts'], sourceCombine: 'difference' as const };
    const before = wire(op);
    expect(sourceModePatch('objects', {})).toBeNull();
    expect(sourceModePatch('layers', {})).toBeNull();
    expect(wire(op)).toEqual(before);
    expect(op.sourceLayers).toEqual(['Cuts']);
  });
  it('restores remembered multiple layers and ordered IDs without changing interpretation', () => {
    const memory = { layers: ['Cuts', 'Engraving'], objects: [3, 1] };
    const op = { ...fixture(), sourceCombine: 'xor' as const };
    expect(wire({ ...op, ...sourceModePatch('objects', memory)! })).toEqual({
      kind: 'objects',
      ids: [3, 1],
      combine: 'xor',
    });
    expect(wire({ ...op, ...sourceModePatch('layers', memory)! })).toEqual({
      kind: 'layers',
      layers: ['Cuts', 'Engraving'],
      combine: 'xor',
    });
    const all = { ...op, ...sourceModePatch('all', memory)! };
    expect(all.sourceCombine).toBe('xor');
    expect(wire(all)).toEqual({ kind: 'all' });
    expect(memory).toEqual({ layers: ['Cuts', 'Engraving'], objects: [3, 1] });
  });
  it('keeps each combine value through wire conversion, save/reopen and duplication', () => {
    for (const combine of combineChoices) {
      const op = { ...fixture(), sourceLayers: ['svg', 'Cuts'], sourceCombine: combine };
      const reopened = JSON.parse(JSON.stringify(op));
      const duplicate = { ...structuredClone(reopened), id: 2 };
      expect(wire(duplicate)).toEqual({
        kind: 'layers',
        layers: ['svg', 'Cuts'],
        ...(combine === 'auto' ? {} : { combine }),
      });
      expect(reopened).toEqual(op);
    }
  });
  it('changes sources as one undoable command, preserving unrelated machining fields', () => {
    const op = {
      ...fixture(),
      sourceLayers: ['Cuts', 'Inner'],
      sourceCombine: 'difference' as const,
    };
    const state = { operations: [op], dirty: false };
    const history = new History();
    history.exec(
      updateOperationCommand(op.id, sourceModePatch('objects', { objects: [2, 1] })!),
      state,
    );
    expect(state.operations[0].sourceObjects).toEqual([2, 1]);
    expect(state.operations[0].depth).toBe(op.depth);
    expect(state.operations[0].sourceCombine).toBe('difference');
    history.undo(state);
    expect(state.operations[0]).toEqual(op);
    history.redo(state);
    expect(wire(state.operations[0])).toEqual({
      kind: 'objects',
      ids: [2, 1],
      combine: 'difference',
    });
  });
  it('keeps SVG synthetic layers and named DXF layers across merged drawings', () => {
    const imports = [
      entry('logo.svg', 'svg', 'svg', 1),
      entry('letters.svg', 'svg', 'svg', 2),
      entry('plate.dxf', 'dxf', 'Cuts', 3),
    ];
    const merged = combineImports(imports)!;
    expect(layerProvenance('svg', imports)).toEqual({
      name: 'svg',
      syntheticSvg: true,
      drawings: [
        { filename: 'logo.svg', format: 'svg' },
        { filename: 'letters.svg', format: 'svg' },
      ],
    });
    expect(layerProvenance('Cuts', imports).syntheticSvg).toBe(false);
    expect(merged.layers.find((l) => l.name === 'svg')!.segment_count).toBe(2);
    expect(merged.objects).toEqual([1, 2, 3]);
    expect([1, 2, 3].map((id) => opIncludesObject({ sourceLayers: ['svg'] }, id, merged))).toEqual([
      true,
      true,
      false,
    ]);
    expect([1, 2, 3].map((id) => opIncludesObject({ sourceLayers: ['Cuts'] }, id, merged))).toEqual(
      [false, false, true],
    );
    expect(
      [1, 2, 3].map((id) => opIncludesObject({ sourceLayers: ['Cuts', 'svg'] }, id, merged)),
    ).toEqual([true, true, true]);
  });
  it('does not label a DXF layer named svg as an SVG-only source or scope equal names to files', () => {
    const imports = [entry('a.svg', 'svg', 'svg', 1), entry('b.dxf', 'dxf', 'svg', 2)];
    expect(layerProvenance('svg', imports).syntheticSvg).toBe(false);
    expect(combineImports(imports)!.layers).toHaveLength(1);
  });
  it('retains missing IDs/layers for existing generation diagnostics', () => {
    const op = {
      ...fixture(),
      sourceLayers: ['gone'],
      sourceObjects: [999],
      sourceCombine: 'intersection' as const,
    };
    expect(wire(op)).toEqual({ kind: 'objects', ids: [999], combine: 'intersection' });
    expect(op.sourceLayers).toEqual(['gone']);
  });
  it('shows region interpretation only for consumers and preserves hidden saved values', () => {
    for (const kind of [
      'profile',
      'engrave',
      'drill',
      'chamfer',
      'thread',
      'drag_knife',
      'planar_surface',
      'relief_mill',
      'waterline_rough',
      'raster_engrave',
    ] as const) {
      const op = { ...fixture(kind), sourceLayers: ['Cuts'], sourceCombine: 'union' as const };
      expect(canInterpretContours(op)).toBe(false);
      expect(op.sourceCombine).toBe('union');
    }
    for (const kind of ['pocket', 'vcarve'] as const) {
      expect(canInterpretContours({ ...fixture(kind), sourceLayers: ['Cuts'] })).toBe(true);
      expect(canInterpretContours({ ...fixture(kind), sourceLayers: null })).toBe(false);
    }
    expect(
      canInterpretContours({
        ...fixture('pocket'),
        kind: 'pocket',
        sourceLayers: ['Cuts'],
        frameShape: 'rectangle',
        pocketStrategy: 'cascade',
      }),
    ).toBe(false);
  });
  it('uses consumer-specific Auto and Difference help without promising layer operand ordering', () => {
    expect(contourHelp({ ...fixture('pocket'), sourceLayers: ['svg'] })).toBe(
      'opprops.geometry.pocket_auto.help',
    );
    expect(contourHelp({ ...fixture('vcarve'), sourceLayers: ['svg'] })).toBe(
      'opprops.combine.auto.help',
    );
    expect(
      contourHelp({
        ...fixture('pocket'),
        kind: 'pocket',
        pocketStrategy: 'halfpipe',
        sourceLayers: ['svg'],
      }),
    ).toBe('opprops.combine.auto.help');
    const diff = {
      ...fixture('pocket'),
      sourceCombine: 'difference' as const,
      sourceLayers: ['Cuts'],
    };
    expect(contourHelp(diff)).toBe('opprops.geometry.difference_layers.help');
    expect(contourHelp({ ...diff, sourceObjects: [2, 1] })).toBe(
      'opprops.geometry.difference_objects.help',
    );
  });
});
