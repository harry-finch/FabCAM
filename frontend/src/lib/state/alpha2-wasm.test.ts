import { validateDrawingSnapshot } from './drawing-load';
import { readFileSync } from 'node:fs';
import { expect, it, vi } from 'vitest';
vi.mock('./text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('./workspace.svelte', () => ({ workspace: {} }));
import { validateProjectGeometry as oldReader } from './project-geometry-v1-reader.fixture';
import { validateProjectV10 } from './project-v10';
import { explicitProjectFixture } from './explicit-project.fixture';
import { drawingCommand, drawingInputKey, type DrawingState } from './drawing';
import { History } from './history';
import type { ScopedResponse, ScopedRequest } from '../api/types';
const line = (a: number[], b: number[]) => ({
  type: 'LINE',
  start: { x: a[0], y: a[1] },
  end: { x: b[0], y: b[1] },
  layer: 'synthetic',
  color: 7,
  bulge: 0,
});
const original = [
  line([0, 0], [10, 0]),
  line([10, 0], [10, 10]),
  line([10, 10], [0, 10]),
  line([0, 10], [0, 0.00005]),
];
const start = {
  kind: 'start',
  version: 1,
  legacy_offset: { x: 0, y: 0 },
  imports: [
    {
      id: 1,
      original,
      legacy_objects: [],
      pivot: { x: 5, y: 5 },
      next_entity: 1,
      transform: {
        translate: { x: 0, y: 0 },
        rotation_deg: 0,
        scale: 1,
        mirror_x: false,
        mirror_y: false,
      },
    },
  ],
};
it('real WASM v1/v2 boundaries, explicit repair, stale apply and guarded portable v9', async () => {
  const url = new URL('../../../../crates/ivac-wasm/pkg/ivac_wasm.js', import.meta.url);
  const wasm = await import(/* @vite-ignore */ url.href);
  await wasm.default({ module_or_path: readFileSync(new URL('./ivac_wasm_bg.wasm', url)) });
  const old = wasm.evaluateDrawing(start);
  expect(old.version).toBe(1);
  expect(() => wasm.evaluateDrawing({ ...start, version: 2 })).toThrow();
  const analyzed = wasm.evaluateDrawingV2({
    kind: 'analyze',
    version: 2,
    input: start,
  }) as ScopedResponse;
  expect(analyzed.version).toBe(2);
  expect(analyzed.drawing.version).toBe(1);
  expect(analyzed.continuity[0].exact).toBe(false);
  expect(analyzed.continuity[0].max_gap_mm).toBeCloseTo(0.00005, 10);
  const machineProject = JSON.parse(
    readFileSync(
      new URL('../../../../tests/fixtures/test1/profile.request.json', import.meta.url),
      'utf8',
    ),
  ).project;
  machineProject.operations = [];
  machineProject.segments = original;
  machineProject.text_layers = [];
  const context = {
    drawing: analyzed.drawing,
    project: machineProject,
    bindings: [],
    document: { workOffset: [0, 0, 0], operations: [], text: [], auxiliary: [], revision: 1 },
  };
  const gap = analyzed.findings.find((f) => f.repair?.kind === 'gap')!.repair!;
  const previewRequest = {
    kind: 'preview',
    version: 2,
    input: { context, repairs: [{ ...gap, connector: true, tolerance_mm: 0.001 }] },
  } as ScopedRequest & { kind: 'preview' };
  const result = wasm.evaluateDrawingV2(previewRequest) as ScopedResponse;
  expect(result.local_edit_accepted).toBe(true);
  expect(result.continuity[0].exact).toBe(true);
  expect(
    wasm.evaluateDrawingV2({
      kind: 'apply',
      version: 2,
      preview: previewRequest.input,
      current: context,
    }),
  ).toEqual(result);
  expect(() =>
    wasm.evaluateDrawingV2({
      kind: 'apply',
      version: 2,
      preview: previewRequest.input,
      current: { ...context, document: { ...context.document, revision: 2 } },
    }),
  ).toThrow(/stale/);
  const imports = [
    {
      id: 1,
      source: {
        filename: 'synthetic.dxf',
        format: 'dxf',
        segments: original,
        layers: [],
        bbox: { min_x: 0, min_y: 0, max_x: 10, max_y: 10 },
        unit_scale: 1,
        warnings: [],
        objects: [1, 1, 1, 1],
        object_meta: [],
      },
      fileTransform: {
        translate: { x: 0, y: 0 },
        rotateDeg: 0,
        scale: 1,
        mirrorX: false,
        mirrorY: false,
      },
    },
  ] as unknown as import('./project-types').ImportEntry[];
  const state: DrawingState = {
    version: 2,
    inputKey: drawingInputKey(imports),
    originalBindings: [],
    response: result,
  };
  const file = explicitProjectFixture({ imports, operations: [], tools: [], drawing: state });
  // Actual baseline reader source, unchanged: old readers reject nested revision.
  expect(() => oldReader(file)).toThrow(/incompatible/);
  const saved = JSON.parse(JSON.stringify(file));
  validateProjectV10(saved);
  const loaded = await validateDrawingSnapshot(saved, {
    evaluateDrawingV2: async (request) => wasm.evaluateDrawingV2(request),
  } as import('../api/client').WiacClient);
  expect(loaded.drawing?.response.drawing).toEqual(result.drawing);
  const reopened = wasm.evaluateDrawingV2({
    kind: 'analyze',
    version: 2,
    input: { kind: 'evaluate', version: 1, drawing: saved.drawing!.response.drawing },
  }) as ScopedResponse;
  expect(reopened.drawing).toEqual(result.drawing);
  expect(reopened.local_edit_accepted).toBe(false);
  const malformed = structuredClone(saved);
  malformed.drawing!.response.version = 1;
  expect(() => validateProjectV10(malformed)).toThrow();
  const model = { drawing: undefined as DrawingState | undefined, operations: [], dirty: false };
  const history = new History();
  history.exec(drawingCommand(undefined, state, [], []), model);
  expect(model.drawing?.version).toBe(2);
  history.undo(model);
  expect(model.drawing).toBeUndefined();
  history.redo(model);
  expect(model.drawing).toEqual(state);
});
