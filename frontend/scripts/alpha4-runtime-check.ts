/** Actual frontend copy service and WASM/native/HTTP consumers; synthetic only. */
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import init, * as wasm from '../../crates/ivac-wasm/pkg/ivac_wasm.js';
import { preparePresetReuse, confirmPresetReuse } from '../src/lib/state/preset-reuse';
import { buildGenerationProject } from '../src/lib/state/generation-context';
import { combineImports } from '../src/lib/state/file-transform';
import { History } from '../src/lib/state/history';
import type { ProjectState } from '../src/lib/state/project.svelte';
import type { PresetCollection } from '../src/lib/state/preset-collections';
import type { ReuseField } from '../src/lib/state/preset-reuse-metadata';
const root = resolve(process.cwd(), '..'),
  out = resolve(root, 'dist-test/alpha4/runtime');
mkdirSync(out, { recursive: true });
await init({
  module_or_path: readFileSync(resolve(root, 'crates/ivac-wasm/pkg/ivac_wasm_bg.wasm')),
});
const norm = (x: unknown) =>
  JSON.parse(JSON.stringify(x, (_k, v) => (v instanceof Map ? Object.fromEntries(v) : v)));
function fixture() {
  const file = JSON.parse(
    readFileSync(resolve(root, 'tests/fixtures/alpha32/browser-pocket.ivac-project.json'), 'utf8'),
  );
  file.machine.physicalId = 'synthetic-target-machine';
  file.material = { id: 'synthetic-target-material', revision: '1' };
  file.tools.forEach(
    (t: { id: number; physicalId: string }) => (t.physicalId = `synthetic-tool-${t.id}`),
  );
  file.operations[0].stages.finishing = null;
  delete file.operations[0].finishToolId;
  const data = {
    ...file,
    serializedExtensions: {},
    groupOpsByTool: false,
    reliefSources: [],
    workOffset: { wcs: 'G54', x_mm: 0, y_mm: 0, z_mm: 0 },
  };
  return {
    data,
    history: new History(),
    documentEpoch: 1,
    target: () => data,
    get transformedImport() {
      return combineImports(data.imports);
    },
    get geometryView() {
      return combineImports(data.imports);
    },
    get stockSizingImport() {
      return combineImports(data.imports);
    },
  } as unknown as ProjectState;
}
const sourceReq = JSON.parse(
  readFileSync(resolve(root, 'tests/fixtures/alpha3-v10/profile.request.json'), 'utf8'),
);
sourceReq.project.machine.physical_id = 'synthetic-source-machine';
sourceReq.project.machine.name = 'Source router';
sourceReq.project.material = { id: 'synthetic-source-material', revision: '1' };
sourceReq.project.tools.forEach(
  (t: { id: number; physical_id: string }) => (t.physical_id = `synthetic-source-tool-${t.id}`),
);
const inspected = JSON.parse(
  wasm.presetCommand(
    JSON.stringify({
      action: 'inspect',
      project: sourceReq.project,
      op_id: sourceReq.project.operations[0].id,
    }),
  ),
);
assert.ok(inspected.context);
inspected.parameters.stages.main = {
  ...inspected.parameters.stages.main,
  rpm: 12000,
  feed_mm_min: 600,
  plunge_mm_min: 120,
  depth_per_pass_mm: 0.5,
};
const collection: PresetCollection = {
  format: 'fabcam-preset-collection',
  version: 2,
  id: 'synthetic-alpha4',
  name: 'Synthetic ALPHA-4',
  presets: [
    {
      id: 'profile-source',
      name: 'Profile source values',
      context: inspected.context,
      parameters: inspected.parameters,
      tested: true,
      test_date: '2026-01-01',
      notes: 'PRIVATE TRIAL NOT PORTABLE',
    },
  ],
};
wasm.presetCommand(
  JSON.stringify({ action: 'validate_collection', json: JSON.stringify(collection) }),
);
writeFileSync(resolve(out, 'synthetic-collection.json'), JSON.stringify(collection, null, 2));
const matching = JSON.parse(
  wasm.presetCommand(
    JSON.stringify({
      action: 'load',
      project: sourceReq.project,
      op_id: sourceReq.project.operations[0].id,
      collection,
      preset_id: 'profile-source',
    }),
  ),
);
assert.equal(matching.params.stages.main.rpm, 12000);
function request(p: ProjectState, post = 'linuxcnc') {
  return {
    project: buildGenerationProject(p, true),
    post_processor: post,
    ...(post === 'cps' ? { cps_post: { source: { kind: 'bundled', id: 'shopbot' } } } : {}),
  };
}
const rows: unknown[] = [];
async function run(name: string, req: ReturnType<typeof request>, refusal = false) {
  const path = resolve(out, `${name}.request.json`);
  writeFileSync(path, JSON.stringify(req));
  const native = spawnSync(resolve(root, 'target/debug/ivac'), ['generate', '--request', path], {
    encoding: 'utf8',
    timeout: 30000,
    maxBuffer: 64 * 1024 * 1024,
  });
  const http = await fetch('http://127.0.0.1:18987/generate', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(req),
    }),
    body = await http.json();
  if (refusal) {
    assert.notEqual(native.status, 0, name);
    assert.equal(http.ok, false, name);
    const reason = name.includes('shopbot') ? /unsupported record spindleSpeed/ : /machine maximum/;
    assert.match(native.stderr, reason, name);
    assert.match(JSON.stringify(body), reason, name);
    if (req.post_processor !== 'cps') assert.throws(() => wasm.generate(req));
    rows.push({ name, expected_refusal: true, native: native.stderr.trim(), http: body });
    return;
  }
  assert.equal(native.status, 0, `${name}: ${native.error ?? native.stderr}`);
  assert.ok(http.ok, `${name}: ${JSON.stringify(body)}`);
  const n = JSON.parse(native.stdout),
    w = norm(wasm.generate(req));
  for (const key of ['gcode', 'toolpath', 'cutting_stages', 'warnings']) {
    assert.deepEqual(w[key], n[key], `${name} WASM ${key}`);
    assert.deepEqual(body[key], n[key], `${name} HTTP ${key}`);
  }
  const minZ = Math.min(
    ...n.toolpath.map((s: { from: { z: number }; to: { z: number } }) =>
      Math.min(s.from.z, s.to.z),
    ),
  );
  assert.ok(minZ >= req.project!.operations[0].params.depth - 1e-9, name);
  rows.push({
    name,
    native_wasm_http_equal: true,
    minZ,
    gcode_sha256: createHash('sha256').update(n.gcode).digest('hex'),
    moves: n.toolpath.length,
  });
}
const base = request(fixture());
await run('without-reuse', base);
assert.throws(() =>
  wasm.presetCommand(
    JSON.stringify({
      action: 'load',
      project: base.project,
      op_id: 1,
      collection,
      preset_id: 'profile-source',
    }),
  ),
);
for (const field of ['rpm', 'feed_mm_min', 'plunge_mm_min', 'depth_per_pass_mm'] as ReuseField[]) {
  const p = fixture(),
    before = JSON.parse(JSON.stringify(p.data.operations[0]));
  await confirmPresetReuse(
    p,
    await preparePresetReuse(
      p,
      1,
      { data: collection, token: 'synthetic-rev-1' },
      'profile-source',
      'main',
      'main',
      [field],
    ),
    async () => [{ json: JSON.stringify(collection), token: 'synthetic-rev-1' }],
  );
  const only = { ...before, stages: structuredClone(before.stages) };
  only.stages.main[field] = collection.presets[0].parameters.stages!.main[field];
  assert.deepEqual(JSON.parse(JSON.stringify(p.data.operations[0])), only, field);
  const req = request(p);
  assert.equal(JSON.stringify(req).includes('fabcamPresetValueReuse'), false);
  await run(`copy-${field}`, req);
}
const limit = fixture();
limit.data.machine.maxFeedMmMin = 500;
await confirmPresetReuse(
  limit,
  await preparePresetReuse(
    limit,
    1,
    { data: collection, token: 'synthetic-rev-1' },
    'profile-source',
    'main',
    'main',
    ['feed_mm_min'],
  ),
  async () => [{ json: JSON.stringify(collection), token: 'synthetic-rev-1' }],
);
await run('target-machine-limit', request(limit), true);
const shop = fixture();
shop.data.operations[0].stages!.finishing = {
  cutting: { ...shop.data.operations[0].stages!.main },
  scope: 'walls_only',
  lateral_allowance_mm: 0.1,
  floor_allowance_mm: 0,
};
shop.data.operations[0].finishToolId = shop.data.operations[0].toolId;
await confirmPresetReuse(
  shop,
  await preparePresetReuse(
    shop,
    1,
    { data: collection, token: 'synthetic-rev-1' },
    'profile-source',
    'main',
    'main',
    ['rpm'],
  ),
  async () => [{ json: JSON.stringify(collection), token: 'synthetic-rev-1' }],
);
await run('shopbot-intra-section-rpm', request(shop, 'cps'), true);
writeFileSync(
  resolve(root, 'docs/releases/evidence/alpha4/runtime.json'),
  JSON.stringify(
    {
      base: '38fc911',
      synthetic_only: true,
      cases: rows,
      strict_matching_pass: true,
      strict_cross_context_refused: true,
      wasm_sha256: createHash('sha256')
        .update(readFileSync(resolve(root, 'crates/ivac-wasm/pkg/ivac_wasm_bg.wasm')))
        .digest('hex'),
    },
    null,
    2,
  ) + '\n',
);
console.log(`${rows.length} actual frontend/native/HTTP/WASM outcomes verified`);
