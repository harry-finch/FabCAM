# FS2 — Initial data profiles and provenance

Date: 2026-10-12. Engineering handoff; human approval pending.
User approved FS1 and authorized FS2 (“ok alors je valide, on peut passer à la
suite”). Published base remains f6d17dd; new FS work is uncommitted.

## Delivered boundary

`ivac-feeds-speeds::load_dataset_json(&contents)` parses and validates a
format-1 DatasetDocument. LoadedDataset exposes immutable document and engine
references. The host owns file I/O and deliberate dataset selection. Existing
serde_json moves to normal dependencies alongside Serde; no new dependency
version, filesystem/network/UI/geometry/post dependency or default embedded data.
No recommendation arithmetic changed; engine policy remains 0.5.0/fs1-policy-1.

The envelope contains description/license, machine/material/tool catalogs and
schema-1 ProfileSet. IDs and revisions are immutable authoring identities;
labels never participate in matching. No inheritance, live remote reference,
automatic update, partial load or missing-parameter completion. The strict Serde
models define the JSON schema; no separate generated JSON Schema file is added.
Domain and distribution schema revisions are separate. Unknown fields/versions
and duplicate structural fields fail parsing. Semantic validation rejects
invalid ranges/data/claims, duplicate IDs, dangling exact/family catalog
references, mismatched exact-tool geometry/flutes, zero tool maximum RPM or a
preferred target exceeding a declared tool maximum. All rows must pass before
any engine is returned. Structural validation cannot prove source rights or
physical trials; those require human/source qualification.

## Initial source and dataset

[Data, licensing and authoring contract](../../../data/feeds-speeds/README.md)
records attribution, pinned upstream revision, PDF checksum and transformations.
Sienci Labs' public Resources repository includes both the handbook and metric
PDF under its CC BY-SA 4.0 license. The external CSV URLs alone were not used
as licensing evidence. Retrieved source PDF was parsed and page 1 rendered and
visually reviewed to confirm row/column alignment.

The six reference profiles are the two explicit single-flute flat upcut rows
(1/4 inch = 6.35 mm; 1/8 inch = 3.175 mm), for each Reduced/Regular/Full Speed
mode, using **slotting DOC only**. Feed, plunge, RPM and slotting DOC remain as
published. Modes map to Prudent/Normal/Rapid without numerical factors.
All evidence remains ReferenceData; source authors' testing is not a locally
recorded trial. No production Tested/Validated or theoretical profile is added.
Separate synthetic loader tests prove local record distinctions without shipping
synthetic cutting advice.

Scope is deliberately narrow: exact source LongMill MK2 reference context,
source-defined low-hardness softwood/soft plywood/MDF category, exact source tool
geometry and identity, full-slot/full-width, exact source DOC. Source entry
strategy, thickness, physical machine limits and tool maximum RPM are unknown.
These references are not automatically mapped to any lab stock/tool or ShopBot.
A 6 mm cutter never receives a 6.35 mm row; a two-flute tool does not receive a
single-flute row. Pocket DOC and nominal stepover do not establish instantaneous
WOC, so pocket/finish/profile transfer is unavailable. This delivers relevant
wood-based reference data, not an immediately usable ShopBot recommendation set.

Preferred target RPM/plunge/DOC are explicitly not hard limits. FS1 has no
source-specific hard cutting envelope beyond its matching scopes and supplied
machine/tool constraints. This subset imports no such unsupported envelope.
A changed request limit produces a traced adjustment and AdjustedOutsideEvidence;
it never creates a locally validated result. Unknown limits remain diagnostics.
Absent matching data remains unavailable rather than manufacturing a fallback.

## Verification

Final commands (all exit 0):

```sh
RUSTC_WRAPPER='' cargo test -p ivac-feeds-speeds --locked --offline
RUSTC_WRAPPER='' cargo clippy -p ivac-feeds-speeds --all-targets --locked --offline -- -D warnings
RUSTC_WRAPPER='' cargo check -p ivac-feeds-speeds --target wasm32-unknown-unknown --locked --offline
cargo fmt --all -- --check
RUSTC_WRAPPER='' cargo run -p xtask --locked --offline -- schema-check
git diff --check
```

**38 engine + 11 dataset tests + 1 API doctest pass.** Dataset checks include:
independent expected values for all six published rows; full document/result
round-trip; no ShopBot/unknown-machine transfer; no diameter/flute/tool/material
transfer; no pocket/finish/profile/missing-DOC transfer; evidence after clamps;
unsupported versions/fields/units, duplicate fields and malformed JSON;
dangling catalog revisions/family members; catalog duplicates, tool geometry
and hard-limit contradictions; incomplete/invalid data or unsourced evidence;
synthetic local Tested/Validated records distinct from external references.

FS-13 data-loading/version/provenance and FS-02/06/07/08 scope/evidence coverage
extends FS1. Core/public schema drift guard passes. The FS1 serial workspace
baseline remains the broader evidence; no broad workspace repeat, app build,
controller test or physical cutting is claimed in FS2. No CAM integration changed.

## Human gate and remaining limitations

FS2 stops for review of data provenance, scope and loading contract. No operator
app test or machine cut is required for this isolated milestone. FS3 is unstarted:
it will add catalog/context adapters, explicit accepted values, manual edits,
save/reopen behavior and consumer parity. Data expansion/actual ShopBot tool and
stock mapping needs explicit qualification; local calibration belongs to FS4.
M4's existing machining result is not retroactively a calibrated FS preset.

No new commit/push, PR, merge or upstream sync is authorized or performed. FS1
approval ivac-406n is closed; FS2 engineering ivac-ei1n is delivered; its human
approval remains open in ivac-otng. See STATUS/ROADMAP before continuing in a new session.
