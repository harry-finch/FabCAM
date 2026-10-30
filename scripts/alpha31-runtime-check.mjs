/** ALPHA-3.1 source-bound native/WASM/output differential, synthetic inputs only.
 * The archived baseline executable is local evidence, never distributed.
 * Run after building native CLI and the fresh canonical web WASM package.
 */
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
const root = resolve(import.meta.dirname, '..');
const out = resolve(root, 'dist-test/alpha31/runtime');
mkdirSync(out, { recursive: true });
const binary = resolve(process.env.CARGO_TARGET_DIR || resolve(root, 'target'), 'debug/ivac');
const baseline = resolve(root, 'dist-test/alpha31/baseline/ivac');
const hash = x => createHash('sha256').update(x).digest('hex');
const pkg = resolve(root, 'crates/ivac-wasm/pkg');
const wasm = await import(pathToFileURL(resolve(pkg, 'ivac_wasm.js')));
await wasm.default({ module_or_path: readFileSync(resolve(pkg, 'ivac_wasm_bg.wasm')) });
const shapes = JSON.parse(readFileSync(resolve(root, 'tests/fixtures/alpha31/polygons.json')));
const base = JSON.parse(readFileSync(resolve(root, 'tests/fixtures/alpha3-v10/profile.request.json')));
base.post_processor = 'linuxcnc'; delete base.cps_post;
function request(shape, kind, finish) {
  const v = structuredClone(base), points = shapes[shape];
  v.project.segments = points.map((a, i) => {
    const b = points[(i + 1) % points.length];
    return { type: 'LINE', start: { x: a[0], y: a[1] }, end: { x: b[0], y: b[1] }, layer: '0', color: 7 };
  });
  const op = v.project.operations[0];
  const profile = ['inside', 'outside'].includes(kind);
  if (profile) op.kind.offset = kind;
  else op.kind = { type: 'pocket', strategy: kind, contour: op.kind.contour, pocket: { xy_overlap: 0.5 } };
  if (finish) {
    op.finish_tool_id = 1;
    op.params.stages.finishing = {
      cutting: { tool_id: 1, rpm: 18000, feed_mm_min: 413, plunge_mm_min: 113, depth_per_pass_mm: 0.2 },
      lateral_allowance_mm: 0.1, floor_allowance_mm: profile ? 0 : 0.1,
    };
  }
  return v;
}
const cases = [];
for (const shape of Object.keys(shapes))
  for (const kind of ['inside', 'outside', 'cascade', 'zigzag', 'spiral'])
    for (const finish of [false, true])
      cases.push([`${shape}-${kind}-${finish ? 'finish' : 'main'}`, request(shape, kind, finish)]);
for (const kind of ['cascade', 'zigzag', 'spiral'])
  for (const [wall, floor] of [[0, 0], [0.05, 0], [0, 0.05]]) {
    const v = request('rectangle', kind, true);
    Object.assign(v.project.operations[0].params.stages.finishing, { lateral_allowance_mm: wall, floor_allowance_mm: floor });
    cases.push([`rectangle-${kind}-${wall}-${floor}`, v]);
  }
for (const post of ['grbl', 'cps'])
  for (const kind of ['inside', 'outside', 'cascade']) {
    const v = request('concave', kind, true);
    v.post_processor = post;
    if (post === 'cps') v.cps_post = { source: { kind: 'bundled', id: 'shopbot' } };
    cases.push([`${post}-concave-${kind}`, v]);
  }
const dual = request('convex', 'cascade', true);
dual.project.tools.push({ ...dual.project.tools[0], id: 2, diameter: 1.5 });
dual.project.operations[0].finish_tool_id = 2;
dual.project.operations[0].params.stages.finishing.cutting.tool_id = 2;
cases.push(['distinct-tool', dual]);
for (const kind of ['outside', 'cascade']) {
  const v = request('convex', kind, true);
  const points = Array.from({ length: 32 }, (_, i) => ({ x: 45 + 20 * Math.cos(i * 2 * Math.PI / 32), y: 45 + 20 * Math.sin(i * 2 * Math.PI / 32) }));
  v.project.segments = points.map((a, i) => ({ type: 'LINE', start: a, end: points[(i + 1) % points.length], layer: '0', color: 7 }));
  cases.push([`medium-32-${kind}`, v]);
}
const multi = request('rectangle', 'inside', false);
const original = structuredClone(multi.project.segments);
for (let i = 1; i < 8; i++) multi.project.segments.push(...original.map(s => ({ ...s, start: { x: s.start.x + 40 * i, y: s.start.y }, end: { x: s.end.x + 40 * i, y: s.end.y } })));
multi.project.stock.width_mm = 400;
cases.push(['multi-eight-main-only', multi]);
const island = request('convex', 'cascade', true);
island.project.segments.push(...request('rectangle', 'inside', false).project.segments.map(s => ({ ...s, start: { x: 22 + s.start.x * 0.1, y: 22 + s.start.y * 0.1 }, end: { x: 22 + s.end.x * 0.1, y: 22 + s.end.y * 0.1 } })));
cases.push(['island-refusal', island]);
// Adversarial source-bound runtime cases: narrow neck, sub-minimum edge,
// approximate closure, crossing, and unqualified source curve.
for (const [name, points] of [
  ['narrow-neck', [[10,10],[30,10],[30,19.5],[40,19.5],[40,10],[60,10],[60,30],[40,30],[40,20.5],[30,20.5],[30,30],[10,30]]],
  ['tiny-edge', [[10,10],[10.0002,10.0002],[40,10],[40,30],[10,30]]],
  ['crossing', [[10,10],[40,30],[10,30],[40,10]]],
  ['near-collinear', [[10,10],[25,10],[40,10],[40,30],[10,30]]],
]) {
  const v = request('convex', 'cascade', true);
  v.project.segments = points.map((a,i)=>({type:'LINE',start:{x:a[0],y:a[1]},end:{x:points[(i+1)%points.length][0],y:points[(i+1)%points.length][1]},layer:'0',color:7}));
  cases.push([`refusal-${name}`,v]);
}
const gap = request('convex','inside',true);
gap.project.segments.at(-1).end.y += 0.000001;
cases.push(['refusal-micro-gap',gap]);
const curve = request('convex','outside',true);
curve.project.segments[0].type='ARC'; curve.project.segments[0].bulge=0.1;
cases.push(['refusal-source-arc',curve]);
const normalize = x => JSON.parse(JSON.stringify(x, (_k, v) => v instanceof Map ? Object.fromEntries(v) : v));
const provenance = x => ({ ...x, transitions: x.transitions.map(t => ({ ...t, before_ref: t.before_ref ?? null })) });
// JS/WASM binary64 and native JSON parsing can differ by one ULP in decoded
// analytic-arc coordinates. Program text and all non-coordinate fields remain
// exact. This comparison tolerance is 20 billion times below the 0.02 mm
// output proof budget; it never changes production values or safety predicates.
function compareTransport(actual, expected, label, row, path = '') {
  if (typeof actual === 'number' && typeof expected === 'number' &&
      /\.(from|to|center)\.(x|y|z)$/.test(path)) {
    const delta = Math.abs(actual - expected);
    assert.ok(Number.isFinite(delta) && delta <= 1e-12, `${label}${path}: ${delta} mm`);
    row.maxCoordinateDeltaMm = Math.max(row.maxCoordinateDeltaMm || 0, delta);
    return;
  }
  if (actual && expected && typeof actual === 'object' && typeof expected === 'object') {
    assert.deepEqual(Object.keys(actual).sort(), Object.keys(expected).sort(), `${label}${path}: keys`);
    for (const key of Object.keys(expected)) compareTransport(actual[key], expected[key], label, row, `${path}.${key}`);
  } else assert.deepEqual(actual, expected, `${label}${path}`);
}
const rows = [];
for (const [name, req] of cases) {
  const file = resolve(out, name + '.request.json');
  writeFileSync(file, JSON.stringify(req, null, 2) + '\n');
  const start = performance.now();
  const n = spawnSync(binary, ['generate', '--request', file], { encoding: 'utf8', maxBuffer: 128 * 1024 * 1024 });
  const nativeMs = performance.now() - start;
  if (name.startsWith('refusal-')) assert.notEqual(n.status, 0, `${name}: unproven geometry must refuse`);
  const b = spawnSync(baseline, ['generate', '--request', file], { encoding: 'utf8', maxBuffer: 128 * 1024 * 1024 });
  assert.notEqual(b.status, null, 'baseline executable unavailable');
  const row = { name, requestSha256: hash(readFileSync(file)), nativeMs, accepted: n.status === 0, baselineAccepted: b.status === 0 };
  if (n.status !== 0) {
    assert.notEqual(b.status, 0, `${name}: previously accepted request now refused: ${n.stderr}`);
    row.refusal = n.stderr.trim().split('\n').at(-1);
    if (req.post_processor !== 'cps') {
      for (const call of [() => wasm.generate(req), () => wasm.generateStreaming(req, () => {}), () => wasm.generateTwoSided(req)]) assert.throws(call, e => typeof e.message === "string" && n.stderr.includes(e.message));
      row.wasmRefusal = true;
    }
    rows.push(row); continue;
  }
  const native = JSON.parse(n.stdout);
  row.programSha256 = hash(native.gcode);
  row.moves = native.toolpath.length;
  row.stages = native.cutting_stages.length;
  row.remaining = native.remaining_material.map(r => ({ status: r.status, reason: r.reason, area_mm2: r.area_mm2 }));
  if (b.status === 0) {
    const old = JSON.parse(b.stdout);
    assert.equal(native.gcode, old.gcode, `${name}: unexplained baseline program change`);
    assert.deepEqual(native.toolpath, old.toolpath, `${name}: unexplained decoded change`);
    row.baselineByteExact = true;
  }
  writeFileSync(resolve(out, name + '.native.json'), JSON.stringify(native));
  if (req.post_processor !== 'cps') {
    let t = performance.now();
    const actual = normalize(wasm.generate(req));
    row.wasmMs = performance.now() - t;
    for (const key of ['gcode', 'toolpath', 'cutting_stages', 'warnings']) compareTransport(actual[key], native[key], `${name}: ${key}`, row);
    assert.deepEqual(provenance(actual.tool_provenance), provenance(native.tool_provenance), name);
    const stream = normalize(wasm.generateStreaming(req, () => {}));
    for (const key of ['gcode', 'toolpath', 'cutting_stages']) compareTransport(stream[key], native[key], `${name}: WASM streaming ${key}`, row);
    const two = normalize(wasm.generateTwoSided(req));
    assert.equal(two.front.gcode, native.gcode, `${name}: two-sided front`);
    const fileOut = resolve(out, name + '.stream.ngc');
    const s = spawnSync(binary, ['stream-gcode', file, '--output', fileOut], { encoding: 'utf8' });
    assert.equal(s.status, 0, `${name}: ${s.stderr}`);
    assert.equal(readFileSync(fileOut, 'utf8'), native.gcode, `${name}: CLI stream`);
    row.runtimeProgramExact = true;
  } else row.wasmCps = 'UNSUPPORTED by canonical browser WASM; native/core qualified separately';
  row.peakNodeRssKiB = process.resourceUsage().maxRSS;
  rows.push(row);
  writeFileSync(resolve(out, 'partial-receipt.json'), JSON.stringify(rows, null, 2));
}
const receipt = {
  baseSha: spawnSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).stdout.trim(),
  uncommitted: true,
  baselineExecutableSha256: hash(readFileSync(baseline)),
  nativeExecutableSha256: hash(readFileSync(binary)),
  wasmSha256: hash(readFileSync(resolve(pkg, 'ivac_wasm_bg.wasm'))),
  cases: rows,
};
writeFileSync(resolve(out, 'receipt.json'), JSON.stringify(receipt, null, 2) + '\n');
console.log(`PASS ${rows.length} cases; ${rows.filter(r => r.baselineByteExact).length} exact baseline programs; ${rows.filter(r => r.runtimeProgramExact).length} native/WASM/stream/two-sided comparisons; ${rows.filter(r => !r.accepted).length} bounded refusals.`);
