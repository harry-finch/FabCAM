import { toWireTool } from '../sim/config';
import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { generationContextKey } from './generation-context';
import { qualificationKey, qualificationPhase, verificationInput } from './program-qualification';
import type { ProjectState } from './project.svelte';
import type { GenerateResponse } from '../api/types';
const file = JSON.parse(
  readFileSync(
    new URL(
      '../../../../docs/releases/ux-c-2026-10-19-evidence/saved-preparation.ivac-project.json',
      import.meta.url,
    ),
    'utf8',
  ),
);
function setup() {
  const data = {
    ...explicitProjectFixture(file),
    settings: { cellResolutionMode: 'auto', cellResolutionMm: 0.1, maxSimulationCells: 250000 },
  };
  const p = {
    data,
    gen: {
      generatedVersion: 1,
      generated: {
        gcode: 'G0 X1',
        toolpath: [
          {
            op_id: 1,
            tool_ref: 1,
            kind: 'cut',
            from: { x: 0, y: 0, z: 0 },
            to: { x: 1, y: 0, z: 0 },
            gcode_line: 1,
          },
        ],
        warnings: [],
        tool_provenance: {
          revision: 1,
          authority: 'resolved',
          transitions: [],
          diagnostics: [],
          bindings: [
            {
              reference: 1,
              project_tool_id: data.tools[0].id,
              program_tool_number: data.tools[0].id,
              tool: toWireTool(data.tools[0]),
            },
          ],
        },
      } as unknown as GenerateResponse,
      pipelineState: 'idle',
    },
    documentEpoch: 1,
    loading: false,
    transformedImport: data.imports[0].source,
    stockSizingImport: data.imports[0].source,
  } as unknown as ProjectState;
  p.gen.inputContextKey = generationContextKey(p, 'linuxcnc');
  p.gen.qualification = {
    key: qualificationKey(p, 'linuxcnc')!,
    phase: 'current',
    front: { warnings: [] },
  };
  expect(p.gen.qualification.key).not.toBeNull();
  return p;
}
it.each(['stock', 'tool', 'machine', 'fixtures', 'origin'] as const)(
  'real generation context invalidates qualification after %s edit',
  (field) => {
    const p = setup();
    const key = qualificationKey(p, 'linuxcnc');
    if (field === 'stock') p.data.stock.thickness++;
    if (field === 'tool') p.data.tools[0].diameter++;
    if (field === 'machine') p.data.machine.fastMoveZ++;
    if (field === 'origin') p.data.workOffset.x_mm++;
    if (field === 'fixtures')
      p.data.fixtures = [
        { id: 1, name: 'clamp', x: 0, y: 0, z: 0, width: 5, depth: 5, height: 5 },
      ] as never;
    expect(qualificationKey(p, 'linuxcnc')).not.toBe(key);
    expect(qualificationPhase(p, 'linuxcnc')).toBe('stale');
  },
);
it('cosmetic settings and cursor do not invalidate; numerical simulation settings do', () => {
  const p = setup();
  const key = qualificationKey(p, 'linuxcnc');
  p.playhead = 0;
  p.data.settings.solidOpacity = 0.5;
  expect(qualificationKey(p, 'linuxcnc')).toBe(key);
  p.data.settings.maxSimulationCells++;
  expect(qualificationPhase(p, 'linuxcnc')).toBe('stale');
});
it('request uses immutable tool bindings and refuses missing movement provenance', () => {
  const p = setup();
  const input = verificationInput(p, qualificationKey(p, 'linuxcnc')!);
  expect(input.grid.topZ).toBe(p.data.stock.offsetZ);
  p.gen.generated!.toolpath[0].tool_ref = undefined;
  expect(() => verificationInput(p, 'new')).toThrow('unknown tool provenance');
});

import { summarizeWarnings } from './warnings-summary';
it.each(['pending', 'failed', 'unavailable', 'stale'] as const)(
  'does not label %s verification as clean',
  (verification) => {
    expect(
      summarizeWarnings({
        verification,
        simWarnings: [],
        pipelineWarnings: [],
        hasGenerated: true,
        hasSimDiagnostics: false,
        dirty: false,
      }).severity,
    ).toBe('unverified');
  },
);

import { explicitProjectFixture } from './explicit-project.fixture';
it('authored rate and preset provenance edits invalidate generated export qualification', () => {
  const p = setup();
  const before = generationContextKey(p, 'linuxcnc');
  p.data.operations[0].cutting!.rates[0].feed_mm_min = 1620;
  p.data.operations[0].cutting!.preset_snapshot = {
    collection_id: 'qa',
    collection_name: 'Synthetic QA',
    preset_id: 'qa-cut',
    preset_name: 'Synthetic cut',
    context: {
      machine_id: 'qa',
      machine_label: 'QA',
      material_id: 'qa',
      material_label: 'QA',
      signature: '{}',
    },
    parameters: {
      rates: structuredClone(p.data.operations[0].cutting!.rates),
      schedule: { kind: 'step', step: -1, finish_step: null },
    },
  };
  expect(generationContextKey(p, 'linuxcnc')).not.toBe(before);
  expect(qualificationPhase(p, 'linuxcnc')).toBe('stale');
  p.gen.inputContextKey = generationContextKey(p, 'linuxcnc');
  p.gen.qualification!.key = qualificationKey(p, 'linuxcnc')!;
  expect(qualificationPhase(p, 'linuxcnc')).toBe('current');
  p.data.operations[0].cutting!.preset_snapshot!.preset_name = 'Changed provenance';
  expect(qualificationKey(p, 'linuxcnc')).toBeNull();
  expect(qualificationPhase(p, 'linuxcnc')).toBe('stale');
});
