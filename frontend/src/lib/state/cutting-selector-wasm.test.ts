import { readFileSync } from 'node:fs';
import { adaptationFixture } from './preset-adaptation.fixture';
import { duplicatePresetAsDraft } from './preset-adaptation';
import { expect, it } from 'vitest';
import reference from '../../../../data/feeds-speeds/packs/sienci-longmill.v1.json';
import { buildProject } from '../api/build-project';
import type { FsCamRequest, FsCamResponse, ImportResponse } from '../api/types';
import type { SourcePack } from './feeds-speeds-sources';
it('real WASM accepts legacy and selected requests with identical authoritative application, and refuses revision substitution', async () => {
  const url = new URL('../../../../crates/ivac-wasm/pkg/ivac_wasm.js', import.meta.url);
  const wasm = await import(/* @vite-ignore */ url.href);
  await wasm.default({ module_or_path: readFileSync(new URL('./ivac_wasm_bg.wasm', url)) });
  const pack = reference as unknown as SourcePack;
  const project = buildProject({
    transformedImport: {
      bbox: { min_x: 0, min_y: 0, max_x: 10, max_y: 10 },
      filename: 'software.svg',
      format: 'svg',
      layers: [],
      object_meta: [],
      objects: [],
      segments: [],
      unit_scale: 1,
      warnings: [],
      unit_status: 'unknown',
      unit_source: '',
    } as ImportResponse,
    machine: {
      unit: 'mm',
      mode: 'mill',
      comments: true,
      arcs: true,
      toolchangeStrategy: 'manual_m0_pause',
      fastMoveZ: 5,
      fsIdentity: pack.dataset.machines[0].identity,
    },
    tools: [
      {
        id: 1,
        name: 'SOFTWARE ONLY',
        kind: 'endmill',
        diameter: 6.35,
        flutes: 1,
        speed: 17000,
        feedRate: 1000,
        plungeRate: 500,
        coolant: 'off',
        fsIdentity: pack.dataset.tools[0].identity,
      },
    ],
    operations: [
      {
        id: 1,
        name: 'Software profile',
        enabled: true,
        kind: 'profile',
        toolId: 1,
        sourceLayers: null,
        depth: -2.8,
        startDepth: 0,
        step: -2.8,
        offset: 'outside',
        fsFullWidth: true,
      },
    ],
    material: pack.dataset.materials[0].identity,
  })!;
  const request: FsCamRequest = {
    project,
    op_id: 1,
    dataset_json: JSON.stringify({ source_packs: [pack] }),
    aggressiveness: 'prudent',
    apply: true,
  };
  const legacy = wasm.feedsSpeeds(request) as FsCamResponse;
  expect(legacy.applied_op).toBeTruthy();
  request.selected_record = {
    pack: pack.manifest.identity,
    record_id: pack.records[0].source_record_id,
    profile: pack.records[0].profile,
  };
  expect(wasm.feedsSpeeds(request)).toEqual(legacy);
  const before = JSON.stringify(project);
  request.selected_record = {
    ...request.selected_record,
    profile: { ...request.selected_record.profile, revision: 'missing' },
  };
  expect(() => wasm.feedsSpeeds(request)).toThrow();
  expect(JSON.stringify(project)).toBe(before);
  const recipe = adaptationFixture();
  recipe.target.machine.fsIdentity = null;
  recipe.target.material = { id: 'material/wood.plywood', revision: '1', label: 'Plywood' };
  const draft = duplicatePresetAsDraft(
    recipe.destination,
    recipe.row,
    recipe.target,
    {
      name: 'SOFTWARE ONLY editable recipe',
      reviewed: true,
      ownershipConfirmed: false,
      notes: '',
      data: { ...recipe.row.profile.data, rpm: 12000, doc_mm: 0.5 },
    },
    '2026-10-21T16:00:00.000Z',
    'manual-recipe',
  );
  request.dataset_json = JSON.stringify({ source_packs: [draft] });
  request.selected_record = {
    pack: draft.manifest.identity,
    record_id: draft.records[0].source_record_id,
    profile: draft.records[0].profile,
  };
  expect(() => wasm.feedsSpeeds(request)).toThrow(/Draft, disabled/);
  expect(JSON.stringify(project)).toBe(before);
});
