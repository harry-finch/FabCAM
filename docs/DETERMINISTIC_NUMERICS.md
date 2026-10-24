# Deterministic numerical boundaries

QUAL-1A.4 integrates the separately reviewed QUAL-1A/1A.2/1A.3 correction.
V-Carve uses `cam::vcarve_math` for the reviewed tangent and length call sites.
`libm =0.2.16` is a crates.io dependency, with default features disabled and
`force-soft-floats` enabled. Cargo.lock pins the official archive checksum
`b6d2cec3eae94f9f509c767b45932f1ada8350c4bdb85af2fcab4a3c14807981`.
There is no local path dependency or copied mathematical implementation.

The dependency declares MIT; its registry source includes `LICENSE.txt` and
per-file upstream copyright notices. Preserve those notices when redistributing
dependency source. No license text or third-party source was modified.

Workspace JSON text parsing pins `serde_json =1.0.149` with `float_roundtrip`.
Runtime consumers include CLI, HTTP, native commands and WASM preset JSON text.
WASM object generation instead receives already represented JavaScript numbers.
Tauri host build dependencies can retain separate resolver-2 feature sets.
No custom parser, output-grid input rounding or v9 schema change is introduced.

Correctly rounded inputs can change branch traversal and intermediate material
states. The accepted dense-circle comparison is fixture-specific; it does not
prove equal engagement, forces, spindle load, fixtures, holders or machining.
The historical Profile internal-coordinate difference remains separately recorded;
matching executable output does not imply universal internal bit equality.

Old preset signature strings remain exact and untouched. A mismatch refuses
application and requires manual review; neither numerical proximity nor software
validation establishes a new workshop test. No automatic migration is performed.

Regression ownership: `qual1a2_numeric` contains 328 independent rounding-oracle
cases, including the original 126 mismatches; `qual1a_planner` retains malformed
and budget refusals. Existing V-Carve, continuous envelope, NUMSAFE, SAFE-Z,
provenance, post and export contracts remain authoritative and unchanged.
The source-bound qualification and remaining host/physical gates are recorded in
[QUAL-1A.4](releases/QUAL_1A_4_PRODUCTION_INTEGRATION.md).
