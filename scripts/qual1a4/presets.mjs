// Disposable synthetic files and real public preset commands; no user stores.
import fs from "node:fs";
import path from "node:path";
import crypto from "node:crypto";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";
import assert from "node:assert/strict";
const root = path.resolve(new URL("../..", import.meta.url).pathname);
const raw = path.resolve(root, "../ivacam/dist-test/qual1a4");
const clone = (x) => structuredClone(x),
  hash = (x) => crypto.createHash("sha256").update(x).digest("hex");
const runtimes = {};
for (const [label, dir] of [
  ["old-wasm", "../FabCAM-qual-1a"],
  ["corrected-wasm", "."],
]) {
  const base = path.resolve(root, dir, "crates/ivac-wasm/pkg");
  const w = await import(pathToFileURL(base + "/ivac_wasm.js"));
  await w.default({
    module_or_path: fs.readFileSync(base + "/ivac_wasm_bg.wasm"),
  });
  runtimes[label] = (x) => JSON.parse(w.presetCommand(JSON.stringify(x)));
}
runtimes["corrected-native"] = (x) => {
  const r = spawnSync(
    path.resolve(root, "../ivacam/dist-test/qual1a4/preset-command"),
    [],
    { input: JSON.stringify(x), encoding: "utf8" },
  );
  if (r.status !== 0) throw new Error(r.stderr);
  return JSON.parse(r.stdout);
};
runtimes["old-native"] = (x) => {
  const r = spawnSync(path.resolve(root, "../ivacam/dist-test/qual1a3/old-native-preset-command"), [], {
    input: JSON.stringify(x),
    encoding: "utf8",
  });
  if (r.status !== 0) throw new Error(r.stderr);
  return JSON.parse(r.stdout);
};
const project = JSON.parse(
  fs.readFileSync(
    root + "/tests/fixtures/perf-v9/profile-contours-linuxcnc.json",
  ),
).project;
project.machine.physical_id = "synthetic-lab-machine";
project.material = { id: "synthetic-material", revision: "1" };
project.tools.forEach((t) => {
  t.physical_id = "synthetic-bit-" + t.id;
  t.diameter = 15.122706142614561;
});
const opid = project.operations[0].id;
const inspect = (call, p) =>
  call({ action: "inspect", project: p, op_id: opid });
const make = (call, p) => {
  const i = inspect(call, p);
  return {
    format: "fabcam-preset-collection",
    version: 1,
    id: "synthetic-qa",
    name: "SOFTWARE QA ONLY",
    presets: [
      {
        id: "p",
        name: "Synthetic author claim; never machined",
        context: i.context,
        parameters: i.parameters,
        tested: true,
        test_date: null,
        notes: "No physical test. Diagnostic synthetic author attestation.",
      },
    ],
  };
};
const old = make(runtimes["old-wasm"], project),
  current = make(runtimes["corrected-wasm"], project);
const stable = clone(project);
stable.tools.forEach((t) => (t.diameter = 6));
const compatible = make(runtimes["old-wasm"], stable);
const originals = path.join(raw, "synthetic-presets");
fs.mkdirSync(originals, { recursive: true });
for (const [name, v] of Object.entries({ old, current, compatible }))
  fs.writeFileSync(
    path.join(originals, name + ".json"),
    JSON.stringify(v, null, 2) + "\n",
  );
const before = Object.fromEntries(
  fs
    .readdirSync(originals)
    .map((n) => [n, hash(fs.readFileSync(path.join(originals, n)))]),
);
const results = [];
function check(label, name, p, c, expected, action = "load") {
  const p0 = JSON.stringify(p),
    c0 = JSON.stringify(c),
    start = performance.now();
  let value, error;
  try {
    value = runtimes[label](
      action === "load"
        ? {
            action,
            project: p,
            op_id: opid,
            collection: c,
            preset_id: c.presets[0].id,
          }
        : { action, json: typeof c === "string" ? c : JSON.stringify(c) },
    );
  } catch (e) {
    error = String(e);
  }
  const success = error === undefined;
  results.push({
    runtime: label,
    name,
    expected,
    success,
    error,
    ms: performance.now() - start,
  });
  assert.equal(success, expected, `${label} ${name}: ${error}`);
  assert.equal(JSON.stringify(p), p0);
  assert.equal(JSON.stringify(c), c0);
  return value;
}
check("old-wasm", "old-created-old-loaded", project, old, true);
assert.deepEqual(make(runtimes["old-native"], project), old);
check("old-native", "old-created-old-loaded", project, old, true);
check("old-native", "new-signature-under-old-parser", project, current, false);
check("old-native", "unchanged-compatible-old", stable, compatible, true);
for (const label of ["corrected-wasm", "corrected-native"]) {
  check(label, "new-created-new-loaded", project, current, true);
  check(label, "old-numeric-signature-mismatch", project, old, false);
  check(label, "unchanged-compatible-old", stable, compatible, true);
  for (const [name, mutate] of [
    [
      "real-diameter-change",
      (p) =>
        (p.tools.find((t) => t.id === p.operations[0].tool_id).diameter += 0.1),
    ],
    ["machine-configuration", (p) => (p.machine.rapid_speed += 1)],
    ["material-revision", (p) => (p.material.revision = "2")],
    ["operation-strategy", (p) => (p.operations[0].kind.offset = "inside")],
    [
      "physical-tool",
      (p) =>
        (p.tools.find((t) => t.id === p.operations[0].tool_id).physical_id =
          "another-physical-bit"),
    ],
    ["machine-limit", (p) => (p.machine.spindle_rpm_max = 100)],
    [
      "tool-limit",
      (p) =>
        (p.tools.find((t) => t.id === p.operations[0].tool_id).max_rpm = 100),
    ],
  ]) {
    const p = clone(stable);
    mutate(p);
    check(label, name, p, compatible, false);
  }
  const renamed = clone(old);
  renamed.presets[0].name = "Renamed";
  renamed.presets[0].id = "duplicated";
  check(label, "renaming-duplicating-cannot-bypass", project, renamed, false);
  const imported = clone(compatible);
  imported.presets.push({ ...clone(imported.presets[0]), id: "unrelated" });
  const loaded = check(
    label,
    "import-author-claim-with-unrelated-entry",
    stable,
    imported,
    true,
  );
  assert(
    !JSON.stringify(loaded.params.cutting.preset_snapshot).includes(
      "Diagnostic synthetic author",
    ),
  );
  const loadedBytes = JSON.stringify(loaded);
  imported.presets = [];
  assert.equal(JSON.stringify(loaded), loadedBytes);
  results.push({
    runtime: label,
    name: "deletion-does-not-change-copied-operation",
    success: true,
    expected: true,
  });
  const incomplete = clone(stable);
  incomplete.operations[0].params.cutting.rates[0].rpm = null;
  check(
    label,
    "incomplete-operation-inspection",
    incomplete,
    compatible,
    true,
    "validate_collection",
  );
  const inspection = inspect(runtimes[label], incomplete);
  assert(inspection.check.errors.length > 0);
  check(
    label,
    "reviewed-load-can-complete-operation",
    incomplete,
    compatible,
    true,
  );
  const bad = clone(compatible);
  bad.presets[0].parameters.rates[0].rpm = null;
  check(label, "incomplete-preset-refused", stable, bad, false);
  const duplicate = clone(compatible);
  duplicate.presets.push(clone(duplicate.presets[0]));
  check(
    label,
    "duplicate-id-refused",
    stable,
    duplicate,
    false,
    "validate_collection",
  );
  const unattested = clone(compatible);
  unattested.presets[0].tested = false;
  check(label, "missing-author-attestation", stable, unattested, false);
  check(label, "malformed-document", stable, "{", false, "validate_collection");
  check(
    label,
    "oversized-document",
    stable,
    " ".repeat(2 * 1024 * 1024 + 1),
    false,
    "validate_collection",
  );
  const badRate = clone(compatible);
  badRate.presets[0].parameters.rates[0].rpm = 999999;
  check(label, "edited-overlimit-rates-refused", stable, badRate, false);
  // Untrusted external authors can make claims: schema validation is not authentication.
  const external = clone(compatible);
  external.presets[0].parameters.rates[0].feed_mm_min = 321;
  check(
    label,
    "external-edited-valid-author-claim-is-not-proof",
    stable,
    external,
    true,
  );
}
const after = Object.fromEntries(
  fs
    .readdirSync(originals)
    .map((n) => [n, hash(fs.readFileSync(path.join(originals, n)))]),
);
assert.deepEqual(after, before);
const native = inspect(runtimes["corrected-native"], project),
  wasm = inspect(runtimes["corrected-wasm"], project);
assert.equal(native.context.signature, wasm.context.signature);
fs.writeFileSync(
  raw + "/PRESETS.json",
  JSON.stringify(
    {
      results,
      original_files_before: before,
      original_files_after: after,
      original_signatures: {
        old: old.presets[0].context.signature,
        current: current.presets[0].context.signature,
      },
      same_corrected_native_wasm_context: true,
      limits:
        "Real commands on Node/WASM and native. No installed WebView, browser file dialogs, author authentication or physical test claim.",
    },
    null,
    2,
  ) + "\n",
);
console.log(
  `${results.length} expected outcomes verified; original synthetic files unchanged`,
);
