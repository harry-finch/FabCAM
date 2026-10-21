import { notificationText } from '../api/error-notification';
import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import reference from '../../../../data/feeds-speeds/packs/sienci-longmill.v1.json';
import { buildProject } from '../api/build-project';
import { defaultProjectMachine, defaultProjectTools } from './project-defaults';
import type { SourcePack } from './feeds-speeds-sources';
import type { FsCamRequest, FsCamResponse, ImportResponse } from '../api/types';
it('real WASM trial preserves exact values, refuses stale sources and leaves Suggest eligibility unchanged', async () => {
  const url = new URL('../../../../crates/ivac-wasm/pkg/ivac_wasm.js', import.meta.url);
  const wasm = await import(/* @vite-ignore */ url.href);
  await wasm.default({ module_or_path: readFileSync(new URL('./ivac_wasm_bg.wasm', url)) });
  const pack = structuredClone(reference) as unknown as SourcePack;
  Object.assign(pack.manifest, {
    identity: { id: 'sorbonne-fablab', revision: '0001791792000000' },
    schema_version: 2,
    visibility: 'private',
    distribution: 'local_only',
  });
  pack.records = pack.records.slice(0, 1);
  pack.dataset.profile_set.profiles = pack.dataset.profile_set.profiles.slice(0, 1);
  const profile = pack.dataset.profile_set.profiles[0];
  profile.category = 'theoretical';
  profile.evidence = 'theoretical';
  profile.scope.doc_mm = null;
  profile.source.notes = 'PRIVATE TEST SENTINEL';
  profile.data = {
    kind: 'exact_values',
    rpm: 18000,
    feed_mm_min: 1200,
    plunge_mm_min: 300,
    doc_mm: 2,
    stepover_ratio: null,
  };
  pack.records[0].lab = {
    name: 'SOFTWARE ONLY Draft',
    state: 'draft',
    created_at: '2026-10-21',
    updated_at: '2026-10-21',
    tested_at: null,
    physically_tested: false,
    no_critical_failure: false,
    operator: '',
    job_reference: '',
    observations: '',
    detailed_notes: 'PRIVATE TEST SENTINEL',
    finishing_context: '',
    enabled: false,
    deleted: false,
    review: null,
  };
  const machine = {
    ...defaultProjectMachine(),
    spindleRpmMin: 5000,
    spindleRpmMax: 24000,
    maxFeedMmMin: 6000,
    fsIdentity: pack.dataset.machines[0].identity,
  };
  const tool = {
    ...defaultProjectTools()[0],
    diameter: 6.35,
    flutes: 1,
    fluteLengthMm: 20,
    fsIdentity: pack.dataset.tools[0].identity,
  };
  const project = buildProject({
    machine,
    tools: [tool],
    material: pack.dataset.materials[0].identity,
    fsContext: {
      machine_identity: machine.fsIdentity,
      tools: [{ tool_id: tool.id, identity: tool.fsIdentity, max_rpm: 22000 }],
    },
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
    operations: [
      {
        id: 1,
        name: 'Trial',
        enabled: true,
        kind: 'profile',
        toolId: tool.id,
        sourceLayers: null,
        depth: -4,
        startDepth: 0,
        step: -2,
        offset: 'outside',
        fsFullWidth: true,
      },
    ],
  })!;
  const request: FsCamRequest = {
    project,
    op_id: 1,
    dataset_json: JSON.stringify({ source_packs: [pack] }),
    aggressiveness: 'prudent',
    apply: true,
    experimental_trial: true,
    trial_acknowledged: true,
    selected_record: {
      pack: pack.manifest.identity,
      profile: profile.identity,
      record_id: pack.records[0].source_record_id,
    },
  };
  const before = JSON.stringify(project);
  const result = wasm.feedsSpeeds(request) as FsCamResponse;
  expect(
    result.applied_op?.params.applied_cutting?.slots.every(
      (s) => s.rpm === 18000 && s.feed_mm_min === 1200 && s.plunge_mm_min === 300,
    ),
  ).toBe(true);
  expect(JSON.stringify(result.applied_op)).not.toContain('PRIVATE TEST SENTINEL');
  expect(JSON.stringify(project)).toBe(before);
  expect(() => wasm.feedsSpeeds({ ...request, experimental_trial: false })).toThrow();
  try {
    wasm.feedsSpeeds({ ...request, trial_acknowledged: false });
    throw new Error('Expected acknowledgment refusal');
  } catch (e) {
    expect(notificationText(e)).toContain('Acknowledge');
    expect(notificationText(e)).not.toContain('[object Object]');
  }
  expect(() =>
    wasm.feedsSpeeds({
      ...request,
      selected_record: {
        ...request.selected_record!,
        profile: { ...profile.identity, revision: 'stale' },
      },
    }),
  ).toThrow();
});
