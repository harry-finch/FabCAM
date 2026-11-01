import { defaultWorkOffset } from '../src/lib/state/project-types';
import { toWireTool } from '../src/lib/sim/config';
import type { ProjectState } from '../src/lib/state/project.svelte';
import type { GenerateResponse } from '../src/lib/api/types';
import { explicitProjectFixture } from '../src/lib/state/explicit-project.fixture';
import { generationContextKey } from '../src/lib/state/generation-context';
import { qualificationKey } from '../src/lib/state/program-qualification';
import savedFixture from '../../docs/releases/ux-c-2026-10-19-evidence/saved-preparation.ivac-project.json';
const file = savedFixture;
export function setup() {
  const source = explicitProjectFixture(file);
  const data = {
    ...source,
    reliefSources: source.reliefSources ?? [],
    textLayers: source.textLayers ?? [],
    fixtures: source.fixtures ?? [],
    workOffset: source.workOffset ?? defaultWorkOffset(),
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
  if (!p.gen.qualification.key) throw new Error('Synthetic qualification key missing');
  return p;
}
