// Real compiled WASM/native transport comparison; software reference only.
import { readFileSync, writeFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import * as wasm from '../../crates/ivac-wasm/pkg/ivac_wasm.js';
const root = new URL('../../docs/shopbot-modernization/geom1/', import.meta.url);
const request = JSON.parse(readFileSync(new URL('square-repaired.request.json', root), 'utf8'));
wasm.initSync({
  module: readFileSync(new URL('../../crates/ivac-wasm/pkg/ivac_wasm_bg.wasm', import.meta.url)),
});
const base = process.argv[2] ?? 'http://127.0.0.1:18804';
async function post(path, value) {
  const r = await fetch(base + path, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(value),
  });
  if (!r.ok) throw new Error(`${path} ${r.status}: ${await r.text()}`);
  return r.json();
}
const normalize = (value) =>
  Array.isArray(value)
    ? value.map((v) => normalize(v ?? null))
    : value && typeof value === 'object'
      ? Object.fromEntries(
          Object.entries(value)
            .filter(([, v]) => v != null)
            .map(([k, v]) => [k, normalize(v)]),
        )
      : value;
const input = { kind: 'evaluate', version: 1, drawing: request.drawing, tolerance_mm: 0.1 };
const native = await post('/drawing', input);
const compiled = wasm.evaluateDrawing(input);
assert.deepEqual(normalize(compiled), normalize(native));
assert.throws(() => wasm.evaluateDrawing({ ...input, version: 99 }));
const rejected = await fetch(base + '/generate', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify(request),
});
assert.ok(!rejected.ok, 'legacy endpoint must reject edited request shape');
request.machining.post_processor = 'linuxcnc';
request.machining.cps_post = null;
const a = wasm.generateDrawing(request),
  b = await post('/generate-drawing', request);
assert.equal(a.gcode, b.gcode);
assert.deepEqual(normalize(a.toolpath), normalize(b.toolpath));
const result = {
  software_only: true,
  contract: 1,
  evaluated_segments: native.evaluated.segments.length,
  wasm_native_evaluation_equal: true,
  generic_program_equal: true,
  resolved_preview_equal: true,
  legacy_endpoint_status: rejected.status,
  moves: a.toolpath.length,
};
writeFileSync(new URL('transport-parity.json', root), JSON.stringify(result, null, 2) + '\n');
console.log(JSON.stringify(result));
