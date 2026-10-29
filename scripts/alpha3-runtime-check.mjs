/** ALPHA-3 actual native CLI/WASM differential. Public synthetic inputs only. */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import assert from "node:assert/strict";
const root = resolve(import.meta.dirname, "..");
const out = resolve(root, "dist-test/alpha3-qualification/runtime");
mkdirSync(out, { recursive: true });
const binary = resolve(
  process.env.CARGO_TARGET_DIR || resolve(root, "target"),
  "debug/ivac",
);
const pkg = resolve(root, "crates/ivac-wasm/pkg");
const wasm = await import(pathToFileURL(resolve(pkg, "ivac_wasm.js")));
await wasm.default({
  module_or_path: readFileSync(resolve(pkg, "ivac_wasm_bg.wasm")),
});
const base = JSON.parse(
  readFileSync(resolve(root, "tests/fixtures/alpha3-v10/profile.request.json")),
);
base.post_processor = "linuxcnc";
delete base.cps_post;
const cases = [];
for (const post of ["linuxcnc", "grbl"]) {
  for (const kind of ["profile", "pocket"]) {
    for (const allowance of [null, [0.1, 0], [0, 0.1], [0.1, 0.1]]) {
      if (kind === "profile" && allowance?.[1]) continue;
      const req = structuredClone(base);
      req.post_processor = post;
      const op = req.project.operations[0];
      if (kind === "pocket")
        op.kind = {
          type: kind,
          strategy: "cascade",
          contour: op.kind.contour,
          pocket: { xy_overlap: 0.5 },
        };
      if (allowance) {
        op.finish_tool_id = 1;
        op.params.stages.finishing = {
          cutting: {
            ...op.params.stages.main,
            feed_mm_min: 413,
            depth_per_pass_mm: 0.2,
          },
          lateral_allowance_mm: allowance[0],
          floor_allowance_mm: allowance[1],
        };
      }
      cases.push([`${post}-${kind}-${allowance?.join("-") || "main"}`, req]);
    }
  }
}
const dual = structuredClone(
  cases.find(([n]) => n === "linuxcnc-pocket-0.1-0.1")[1],
);
dual.project.tools.push({ ...dual.project.tools[0], id: 2, diameter: 1.5 });
dual.project.operations[0].finish_tool_id = 2;
dual.project.operations[0].params.stages.finishing.cutting.tool_id = 2;
cases.push(["distinct-tool", dual]);
const medium = structuredClone(base);
medium.project.segments = [];
for (let i = 0; i < 24; i++)
  for (const s of base.project.segments) {
    const c = structuredClone(s);
    for (const p of [c.start, c.end]) {
      p.x += (i % 6) * 40;
      p.y += Math.floor(i / 6) * 40;
    }
    medium.project.segments.push(c);
  }
medium.project.stock.width_mm = 300;
medium.project.stock.height_mm = 200;
cases.push(["medium-24-profiles-main", medium]);
const island = structuredClone(
  cases.find(([n]) => n === "linuxcnc-pocket-main")[1],
);
for (const s of base.project.segments) {
  const c = structuredClone(s);
  for (const p of [c.start, c.end]) {
    p.x = 22 + (p.x - 22) * 0.3;
    p.y = 20 + (p.y - 20) * 0.3;
  }
  island.project.segments.push(c);
}
cases.push(["island-main", island]);
// WASM transport maps carry entries; JSON.stringify(Map) alone drops them.
const normalize = (x) =>
  JSON.parse(
    JSON.stringify(x, (_k, v) =>
      v instanceof Map ? Object.fromEntries(v) : v,
    ),
  );
// serde_wasm_bindgen represents the optional prior cutter as undefined; native
// JSON uses null. Canonicalize only this established nullable evidence field.
const evidence = (x) => ({
  ...x,
  transitions: x.transitions.map((t) => ({
    ...t,
    before_ref: t.before_ref ?? null,
  })),
});
const hash = (x) => createHash("sha256").update(x).digest("hex");
const receipts = [];
for (const [name, req] of cases) {
  const file = resolve(out, name + ".request.json");
  writeFileSync(file, JSON.stringify(req, null, 2));
  let t = performance.now();
  const n = spawnSync(binary, ["generate", "--request", file], {
    encoding: "utf8",
    maxBuffer: 64 * 1024 * 1024,
  });
  const nativeMs = performance.now() - t;
  assert.equal(n.status, 0, n.stderr);
  const native = JSON.parse(n.stdout);
  t = performance.now();
  const actual = normalize(wasm.generate(req));
  const wasmMs = performance.now() - t;
  for (const field of ["gcode", "cutting_stages", "toolpath", "warnings"])
    assert.deepEqual(actual[field], native[field], `${name}: ${field}`);
  assert.deepEqual(
    evidence(actual.tool_provenance),
    evidence(native.tool_provenance),
    `${name}: tool_provenance`,
  );
  const events = [];
  const stream = normalize(wasm.generateStreaming(req, (e) => events.push(e)));
  for (const field of ["gcode", "cutting_stages", "toolpath"])
    assert.deepEqual(
      stream[field],
      native[field],
      `${name}: streaming ${field}`,
    );
  assert.deepEqual(
    evidence(stream.tool_provenance),
    evidence(native.tool_provenance),
    `${name}: streaming tool_provenance`,
  );
  const two = normalize(wasm.generateTwoSided(req));
  assert.equal(two.front.gcode, native.gcode);
  writeFileSync(
    resolve(out, name + ".native.json"),
    JSON.stringify(native, null, 2),
  );
  receipts.push({
    name,
    nativeMs,
    wasmMs,
    moves: actual.toolpath.length,
    stages: actual.cutting_stages.length,
    programSha256: hash(actual.gcode),
    streamEvents: events.length,
    peakNodeRssKiB: process.resourceUsage().maxRSS,
  });
}
// Execute archived old WASM, not a new validator pretending to be the old reader.
const oldpkg = resolve(
  root,
  "dist-test/alpha3-preflight/accepted-baseline/wasm-reader-v9",
);
const old = await import(pathToFileURL(resolve(oldpkg, "ivac_wasm.js")));
await old.default({
  module_or_path: readFileSync(resolve(oldpkg, "ivac_wasm_bg.wasm")),
});
const legacy = JSON.parse(
  readFileSync(resolve(root, "tests/fixtures/cli-v9/profile.request.json")),
);
legacy.post_processor = "linuxcnc";
delete legacy.cps_post;
assert.ok(old.generate(legacy).gcode.length > 0);
for (const call of [
  () => old.generate(base),
  () => old.generateStreaming(base, () => {}),
  () => old.generateTwoSided(base),
])
  assert.throws(call, /v9|version|cutting/i);
const col = {
  format: "fabcam-preset-collection",
  version: 2,
  id: "alpha3",
  name: "QA",
  presets: [],
};
assert.throws(() =>
  old.presetCommand(
    JSON.stringify({
      action: "validate_collection",
      json: JSON.stringify(col),
    }),
  ),
);
for (const request of [
  legacy,
  { ...base, project: { ...base.project, cutting_version: 11 } },
])
  assert.throws(() => wasm.generate(request));
const bad = structuredClone(dual);
bad.project.operations[0].params.stages.finishing.floor_allowance_mm = 0.000001;
assert.throws(
  () => wasm.generate(bad),
  (e) => /output precision/.test(e.message),
);
const receipt = {
  sourceHead: spawnSync("git", ["rev-parse", "HEAD"], {
    cwd: root,
    encoding: "utf8",
  }).stdout.trim(),
  dirty: true,
  wasmSha256: hash(readFileSync(resolve(pkg, "ivac_wasm_bg.wasm"))),
  oldWasmSha256: hash(readFileSync(resolve(oldpkg, "ivac_wasm_bg.wasm"))),
  oldReaderRefusal: true,
  cases: receipts,
};
writeFileSync(
  resolve(out, "receipt.json"),
  JSON.stringify(receipt, null, 2) + "\n",
);
console.log(
  `PASS ${cases.length} native/WASM/program/path/stage/provenance/streaming comparisons; actual archived v9 reader and collection-v2 refusal`,
);
