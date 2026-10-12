# ADR-0005: Generic source packs and deterministic cutting knowledge

Status: Implemented under explicitly authorized FS3.7; human checkpoint approval pending
Date: 2026-10-12

## Context

FS3.6 uses a pure engine but a Sienci-specific resource switch, imported JSON in
installation preferences, and full selected-record snapshots in applied proposals.
That last snapshot would expose private source data in shared projects. Existing
identity matching is safe but does not provide cross-vendor taxonomy or resolution.

## Decision

KEEP the pure engine's cutting mathematics, clamps, evidence checks and native
CAM adapter. EXTEND with a versioned JSON source-pack envelope around the existing
validated dataset, manifest, per-record origin/classification and explicit material
mapping. One native loader validates both public and local packs; legacy dataset
format 1 remains readable. No archives, network fetches or executable content.

Taxonomy is application-owned, versioned and independent of pack publishers.
Material mappings must be explicit; a composite source category is not an alias
for each constituent material. Exact identities remain supported. Tool geometry,
flutes and family retain existing hard applicability. Optional tool metadata
preserves helix/material/coating without inventing absent CAM characteristics.
Normalized quantities are computed from original records with derived attribution.

Resolve compatible candidates deterministically. Narrow local physically supported
exact presets retain priority; exact/specific reference applicability precedes a
parent material fallback. Explanation lists ranking dimensions, original/derived
values and selected pack/record versions. Combine no values: among equally ranked
references with comparable RPM/DOC/engagement context, Prudent selects the lowest
chipload, Normal the lower median and Rapid the highest authored same-mode
chipload, each through existing hard-limit calculation. Noncomparable equally
ranked contexts remain ambiguous. Range/disagreement is descriptive, never evidence
promotion. Lexical pack/record IDs only break identical-value ties.

Bundled packs require public/bundled/redistributable manifest plus explicit license,
attribution, transformation permission and reviewed build allowlist. Importing a
pack never adds it to that allowlist. Every user import is local-only regardless
of its authorship/rights; local-only is a distribution rule, not legal permission.
Tauri stores imports under app_data_dir()/feeds-speeds/sources; browser standalone
uses origin-local IndexedDB. Preferences persist references/toggles, not pack JSON.
Native storage accepts validated IDs, never user paths; JSON size is bounded.

Local pack contents never cross the HTTP transport: private recommendations run
locally through WASM/native IPC. Apply stores resolved rates and a small pack/record
identity/evidence/policy summary, not the private profile snapshot. Public snapshots
remain compatible. Missing packs on reopen are informational; saved values remain
authoritative. Legacy imported preferences migrate only after validated storage
succeeds. No private backup/export, cloud sync or telemetry is introduced.

## Alternatives considered

Two engines, vendor taxonomy as universal taxonomy, blind averaging, private packs
in Git/resources, and uploading local files to a remote CAM server are rejected.
Retain ambiguity rather than infer comparability, interpolation or machine rigidity.

## Consequences

Additive pack/response metadata and applied-summary fields require generated schema,
transport and frontend consumers. Legacy project versions 1/2/3 remain supported;
new private-safe summary saves need a guard against old consumers dropping evidence.
No changes to geometry/post algorithms or non-milling processes. Sienci's six rows
and license stay separate from GPL application code; no broader cutting coverage
or local validation is implied.

## Validation

Pack versions/identity/license/distribution, taxonomy mapping, derived values,
ranking/fallback/comparability/disagreement, deterministic ordering, storage paths,
bundle allowlist and private-safe save/reopen need analytical tests. Preserve
existing Sienci numerical qualification and ShopBot output checks. FS4 unstarted.

Software evidence: [FS3.7 qualification](../feeds-speeds/FS3_7_QUALIFICATION.md). Human gate ivac-tm7u remains open.
