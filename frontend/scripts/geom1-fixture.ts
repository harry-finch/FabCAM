// Software-only reference exported through production FE adapters; no workspace state.
import { readFileSync, writeFileSync } from 'node:fs';
import { buildProject } from '../src/lib/api/build-project';
import { drawingImport } from '../src/lib/state/drawing';
import { toCpsPostRequest } from '../src/lib/components/cps-post-form';
import type { ProjectFile } from '../src/lib/state/project-types';
const base = new URL('../../docs/shopbot-modernization/geom1/', import.meta.url);
const file = JSON.parse(
  readFileSync(new URL('square-repaired.ivac-project.json', base), 'utf8'),
) as Required<ProjectFile>;
const project = buildProject({
  ...file,
  transformedImport: drawingImport(file.drawing, file.imports),
});
if (!project) throw new Error('Missing machining project');
writeFileSync(
  new URL('square-repaired.request.json', base),
  JSON.stringify(
    {
      version: 1,
      drawing: file.drawing.response.drawing,
      bindings: file.operations.map((op) => ({ operation: op.id, binding: op.geometryBinding })),
      machining: {
        project,
        post_processor: 'cps',
        cps_post: toCpsPostRequest(file.machine.cpsPost),
      },
    },
    null,
    2,
  ) + '\n',
);
