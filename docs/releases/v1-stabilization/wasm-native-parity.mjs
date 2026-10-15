// Run after the documented native CLI generation and frontend/WASM build.
import { readFileSync, writeFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import * as wasm from '../../../crates/ivac-wasm/pkg/ivac_wasm.js';
wasm.initSync({ module: readFileSync(new URL('../../../crates/ivac-wasm/pkg/ivac_wasm_bg.wasm', import.meta.url)) });
const request = JSON.parse(readFileSync(new URL('deep-pocket.request.json', import.meta.url), 'utf8'));
const result = wasm.generate(request);
assert.equal(result.gcode, readFileSync(new URL('deep-pocket-native.ngc', import.meta.url), 'utf8'));
let inner = false;
for (const s of result.toolpath) {
  if (s.kind !== 'cut' || s.to.z > -0.9) continue;
  if (s.to.x > 24 && s.to.x < 36 && s.to.y > 24 && s.to.y < 36) inner = true;
}
assert.ok(inner, 'compiled WASM must cut the depth-two region');
const evidence = { software_only: true, native_wasm_program_byte_equal: true, wasm_inner_region_cut: true, preview_moves: result.toolpath.length };
writeFileSync(new URL('wasm-native-parity.json', import.meta.url), JSON.stringify(evidence, null, 2) + '\n');
console.log(evidence);
