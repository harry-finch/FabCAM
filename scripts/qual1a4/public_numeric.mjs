// Actual shipping WASM JSON-text API. Inspector echoes authored entry angle without
// converting its validation errors into generation acceptance.
import fs from "node:fs";
import init, {
  presetCommand,
  generate,
} from "../../crates/ivac-wasm/pkg/ivac_wasm.js";
const [raw] = process.argv.slice(2);
await init({
  module_or_path: fs.readFileSync(
    new URL("../../crates/ivac-wasm/pkg/ivac_wasm_bg.wasm", import.meta.url),
  ),
});
const rows = JSON.parse(fs.readFileSync(`${raw}/NUMERIC.json`));
const req = JSON.parse(
  fs.readFileSync(
    new URL(
      "../../tests/fixtures/perf-v9/profile-contours-linuxcnc.json",
      import.meta.url,
    ),
  ),
);
const p = structuredClone(req.project),
  op = p.operations[0];
op.params.cutting.entry_angle_deg = "__LEXEME__";
const template = JSON.stringify({
  action: "inspect",
  project: p,
  op_id: op.id,
});
const view = new DataView(new ArrayBuffer(8));
const bits = (n) => {
  view.setFloat64(0, n);
  return view.getBigUint64(0).toString(16).padStart(16, "0");
};
for (const row of rows) {
  const text = template.replace('"__LEXEME__"', row.lexeme);
  try {
    const value = JSON.parse(presetCommand(text));
    row.wasm_public = {
      bits:
        typeof value.parameters.entry_angle_deg === "number"
          ? bits(value.parameters.entry_angle_deg)
          : null,
      validation_errors: value.check.errors,
    };
  } catch (e) {
    row.wasm_public = { error: String(e) };
  }
  if (
    row.classification === "finite" &&
    row.wasm_public.bits !== row.oracle_bits
  )
    throw new Error(`WASM text incorrect ${row.id}`);
  if (row.classification === "overflow" && !row.wasm_public.error)
    throw new Error(`overflow accepted ${row.id}`);
}
fs.writeFileSync(
  `${raw}/PUBLIC_NUMERIC.json`,
  JSON.stringify(rows, null, 2) + "\n",
);
const rejects = [];
// A valid control must reach generation; the inspector's lexeme placeholder
// must never leak into these independent CAM-validation requests.
generate(req);
for (const [field, value] of [
  ["step", 0],
  ["step", 1],
  ["step", 1e100],
]) {
  const q = structuredClone(req);
  q.project.operations[0].params[field] = value;
  try {
    generate(q);
    throw new Error("accepted invalid CAM request");
  } catch (e) {
    if (String(e).includes("accepted invalid")) throw e;
    const expected = value === 0
      ? "Set explicit depth per pass or an explicit depth schedule"
      : "Depth per pass must be finite and positive";
    if (!e || typeof e !== "object" || e.kind !== "misconfigured" || e.message !== expected)
      throw new Error(`Wrong CAM refusal for ${field}=${value}: ${JSON.stringify(e)}`);
    rejects.push({
      field,
      value,
      error: typeof e === "object" ? e : String(e),
    });
  }
}
fs.writeFileSync(
  `${raw}/PUBLIC_CAM_REFUSALS.json`,
  JSON.stringify(rejects, null, 2) + "\n",
);
console.log(
  `${rows.length} actual WASM text inputs examined; finite bits exact; CAM refusals separate.`,
);
