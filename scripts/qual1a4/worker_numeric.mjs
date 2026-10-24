// Actual production worker handler + WASM in a real Node worker thread.
// Host adapters provide self/postMessage and local WASM loading, not a browser
// acceptance claim. No request fields, results or validators are rewritten.
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { Worker } from "node:worker_threads";
import { createRequire } from "node:module";
const require = createRequire(import.meta.url);
const esbuild = require("../../frontend/node_modules/.pnpm/esbuild@0.21.5/node_modules/esbuild");
const root = path.resolve(new URL("../..", import.meta.url).pathname);
const raw = path.resolve(process.argv[2]);
esbuild.buildSync({
  absWorkingDir: root,
  entryPoints: ["frontend/src/lib/api/wasm.worker.ts"],
  bundle: true,
  format: "esm",
  platform: "browser",
  outfile: raw + "/production-worker.mjs",
  alias: { "ivac-wasm": root + "/crates/ivac-wasm/pkg/ivac_wasm.js" },
});
esbuild.buildSync({
  absWorkingDir: root,
  stdin: {
    contents:
      'export {toCloneable} from "./frontend/src/lib/api/wasm-worker-client"',
    resolveDir: root,
  },
  bundle: true,
  format: "esm",
  platform: "browser",
  outfile: raw + "/production-cloneable.mjs",
});
const { toCloneable } = await import(
  pathToFileURL(raw + "/production-cloneable.mjs")
);
fs.writeFileSync(
  raw + "/node-worker-host.mjs",
  `
import {parentPort,workerData} from 'node:worker_threads';
import fs from 'node:fs';
globalThis.self={onmessage:null,postMessage:m=>parentPort.postMessage(m)};
globalThis.fetch=async url=>{if(!String(url).endsWith('ivac_wasm_bg.wasm'))throw new Error('unexpected fetch');return new Response(fs.readFileSync(workerData.wasm),{headers:{'Content-Type':'application/wasm'}})};
await import(workerData.handler);
parentPort.on('message',m=>self.onmessage({data:m}));
parentPort.postMessage({ready:true});
`,
);
const worker = new Worker(pathToFileURL(raw + "/node-worker-host.mjs"), {
  workerData: {
    wasm: root + "/crates/ivac-wasm/pkg/ivac_wasm_bg.wasm",
    handler: pathToFileURL(raw + "/production-worker.mjs").href,
  },
});
const next = () =>
  new Promise((resolve, reject) => {
    worker.once("message", resolve);
    worker.once("error", reject);
  });
await next();
const results = [];
for (const name of ["dense-circle.json", "square-medial-linuxcnc.json"]) {
  const location =
    name === "dense-circle.json"
      ? path.resolve(root, "../ivacam/dist-test/qual1a/adversarial", name)
      : path.resolve(root, "tests/fixtures/perf-v9", name);
  const request = JSON.parse(fs.readFileSync(location));
  const promise = next();
  worker.postMessage({
    id: 1,
    method: "generate",
    args: [toCloneable(request)],
  });
  const response = await promise;
  if (response.type !== "result") throw new Error(JSON.stringify(response));
  const expected = JSON.parse(
    fs.readFileSync(raw + "/replay/candidate-wasm/" + name + ".response.json"),
  );
  const jsonValue = (v) =>
    v instanceof Map
      ? Object.fromEntries([...v].map(([k, x]) => [k, jsonValue(x)]))
      : Array.isArray(v)
        ? v.map(jsonValue)
        : v && typeof v === "object"
          ? Object.fromEntries(
              Object.entries(v).map(([k, x]) => [k, jsonValue(x)]),
            )
          : v;
  const value = jsonValue(response.value);
  const equal = JSON.stringify(value) === JSON.stringify(expected);
  if (!equal) throw new Error("worker response differs " + name);
  results.push({ case: name, full_response_exact: true });
}
await worker.terminate();
const zero = toCloneable({ negativeZero: -0 });
results.push({
  worker_json_negative_zero_becomes_positive_zero: Object.is(
    zero.negativeZero,
    0,
  ),
  notice:
    "Existing JSON.stringify de-proxy behavior. No signed-zero preservation claim for worker JSON normalization.",
});
fs.writeFileSync(raw + "/WORKER.json", JSON.stringify(results, null, 2) + "\n");
console.log(results);
