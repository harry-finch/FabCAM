import { toWireTool } from '../sim/config';
import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { generationContextKey } from './generation-context';
import {
  programExportStatus,
  qualificationKey,
  qualificationPhase,
  verificationInput,
} from './program-qualification';
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
it.each(['stock', 'stockXY', 'tool', 'machine', 'fixtures', 'origin'] as const)(
  'real generation context invalidates qualification after %s edit',
  (field) => {
    const p = setup();
    const key = qualificationKey(p, 'linuxcnc');
    if (field === 'stock') p.data.stock.thickness++;
    if (field === 'stockXY') {
      p.data.stock.mode = 'manual';
      p.data.stock.customX = 320;
      p.data.stock.customY = 200;
      p.data.stock.offsetX = (p.data.stock.offsetX ?? 0) + 15;
    }
    if (field === 'tool') p.data.tools[0].diameter++;
    if (field === 'machine') p.data.machine.fastMoveZ++;
    if (field === 'origin') p.data.workOffset.x_mm++;
    if (field === 'fixtures')
      p.data.fixtures = [
        { id: 1, name: 'clamp', x: 0, y: 0, z: 0, width: 5, depth: 5, height: 5 },
      ] as never;
    expect(qualificationKey(p, 'linuxcnc')).not.toBe(key);
    expect(qualificationPhase(p, 'linuxcnc')).toBe('stale');
    expect(programExportStatus(p, 'linuxcnc').stale).toBe(true);
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
  p.data.operations[0].stages!.main.feed_mm_min = 1620;
  p.data.operations[0].stagePreset = {
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
      stages: structuredClone(p.data.operations[0].stages!),
    },
  };
  expect(generationContextKey(p, 'linuxcnc')).not.toBe(before);
  expect(qualificationPhase(p, 'linuxcnc')).toBe('stale');
  p.gen.inputContextKey = generationContextKey(p, 'linuxcnc');
  p.gen.qualification!.key = qualificationKey(p, 'linuxcnc')!;
  expect(qualificationPhase(p, 'linuxcnc')).toBe('current');
  p.data.operations[0].stagePreset!.preset_name = 'Changed provenance';
  expect(qualificationKey(p, 'linuxcnc')).toBeNull();
  expect(qualificationPhase(p, 'linuxcnc')).toBe('stale');
});

it('every Main/Finishing field invalidates the generated export context', () => {
  for (const field of [
    'main-tool',
    'main-rpm',
    'main-feed',
    'main-plunge',
    'main-depth',
    'toggle',
    'finish-tool',
    'finish-rpm',
    'finish-feed',
    'finish-plunge',
    'finish-depth',
    'lateral',
    'floor',
    'retained',
  ] as const) {
    const p = setup();
    const op = p.data.operations.find((o) => o.stages)!;
    expect(op).toBeDefined();
    const s = op.stages!;
    s.finishing = { cutting: { ...s.main }, lateral_allowance_mm: 0.2, floor_allowance_mm: 0.1 };
    op.finishToolId = s.main.tool_id;
    const before = generationContextKey(p, 'linuxcnc');
    p.gen.inputContextKey = before;
    if (field === 'toggle') {
      s.finishing = null;
      op.finishToolId = undefined;
    } else if (field === 'main-tool') {
      op.toolId++;
      s.main.tool_id = op.toolId;
    } else if (field === 'finish-tool') {
      op.finishToolId!++;
      s.finishing!.cutting.tool_id = op.finishToolId!;
    } else if (field === 'lateral') s.finishing!.lateral_allowance_mm += 0.1;
    else if (field === 'floor') s.finishing!.floor_allowance_mm += 0.1;
    else if (field === 'retained') op.retainedStockMm = (op.retainedStockMm ?? 0) + 0.1;
    else {
      const target = field.startsWith('main-') ? s.main : s.finishing!.cutting;
      const key = field.endsWith('rpm')
        ? 'rpm'
        : field.endsWith('feed')
          ? 'feed_mm_min'
          : field.endsWith('plunge')
            ? 'plunge_mm_min'
            : 'depth_per_pass_mm';
      target[key] = (target[key] ?? 0) + 1;
    }
    expect(generationContextKey(p, 'linuxcnc'), field).not.toBe(before);
    expect(programExportStatus(p, 'linuxcnc').stale, field).toBe(true);
  }
});
