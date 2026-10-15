// Direct V8/WASM binding benchmark, not a browser/worker benchmark.
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { performance } from 'node:perf_hooks';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import init, { generate } from '../../crates/ivac-wasm/pkg/ivac_wasm.js';

const [requestPath, outputPath, repeatsRaw = '7'] = process.argv.slice(2);
if (!requestPath || !outputPath)
  throw new Error('usage: node perf1-wasm.mjs REQUEST RESULT [REPEATS]');
const payload = readFileSync(requestPath);
const wasmPath = fileURLToPath(
  new URL('../../crates/ivac-wasm/pkg/ivac_wasm_bg.wasm', import.meta.url),
);
const wasmBytes = readFileSync(wasmPath);
const initStart = performance.now();
await init({ module_or_path: wasmBytes });
const initMs = performance.now() - initStart;
const parseStart = performance.now();
const request = JSON.parse(payload);
const parseMs = performance.now() - parseStart;
const runs = [];
let first;
const jsonValue = (v) => {
  if (v instanceof Map)
    return Object.fromEntries([...v].map(([key, value]) => [key, jsonValue(value)]));
  if (Array.isArray(v)) return v.map(jsonValue);
  if (v && typeof v === 'object')
    return Object.fromEntries(Object.entries(v).map(([key, value]) => [key, jsonValue(value)]));
  return v;
};
for (let i = 0; i < Number(repeatsRaw); i++) {
  const input = structuredClone(request);
  const started = performance.now();
  try {
    const result = generate(input);
    const generateMs = performance.now() - started;
    const serializeStart = performance.now();
    const json = JSON.stringify(jsonValue(result));
    const serializeMs = performance.now() - serializeStart;
    if (first === undefined) {
      first = json;
      writeFileSync(resolve(outputPath) + '.response.json', json);
    }
    runs.push({
      run: i,
      generate_ms: generateMs,
      serialize_ms: serializeMs,
      success: true,
      moves: result.toolpath.length,
      identical_to_first: json === first,
    });
  } catch (error) {
    runs.push({
      run: i,
      generate_ms: performance.now() - started,
      success: false,
      error: typeof error === 'object' ? jsonValue(error) : String(error),
    });
  }
}
writeFileSync(
  outputPath,
  JSON.stringify(
    {
      request: requestPath,
      response_sha256:
        first === undefined ? null : createHash('sha256').update(first).digest('hex'),
      request_sha256: createHash('sha256').update(payload).digest('hex'),
      wasm_sha256: createHash('sha256').update(wasmBytes).digest('hex'),
      wasm_bytes: wasmBytes.length,
      engine: process.version + ' / ' + process.versions.v8,
      init_ms: initMs,
      parse_ms: parseMs,
      boundary:
        'generate: includes JS→Rust request and Rust→JS response conversion; excludes init, JSON parsing and explicit serialization',
      runs,
    },
    null,
    2,
  ) + '\n',
);
