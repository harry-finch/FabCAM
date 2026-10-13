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
