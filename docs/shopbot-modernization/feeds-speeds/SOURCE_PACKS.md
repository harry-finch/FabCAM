# Source packs and normalized cutting knowledge — schema 1

See [ADR-0005](../adr/ADR-0005-source-packs-and-cutting-knowledge.md),
[licensing matrix](DATA_SOURCES.md) and `ivac-feeds-speeds/src/source_pack.rs`.
The generated API publishes FsSourcePack and normalized explanation types.

## Pack envelope

JSON only, at most 2 MiB per pack, 32 enabled packs and 16 MiB per calculation.
Unknown fields, unsupported schema versions, duplicate pack/record IDs, unsafe
identity/version strings, missing license/provenance and invalid engineering data
are refused. No archives, path references, executable content, remote loading or
best-effort partial normalization. Stable IDs/versions are separate from names.

A pack contains:

- `manifest`: immutable `identity {id, revision}`, `schema_version: 1`, name,
  publisher, origin, optional source URL, license/optional license URL, attribution,
  visibility public/private, distribution bundled/local_only/external_reference,
  redistributable/transformation permission flags, default evidence and notes.
- `dataset`: existing version-1 dataset with catalogs, scoped profiles and exact
  original cutting data, including record-level license/evidence/trial metadata.
- `records`: one-to-one profile identities, source_record_id, original classification,
  optional explicit material_taxon and normalized tool traits.

Manifest default evidence is descriptive; individual record evidence is
controlling and never promoted by agreement. Local Tested/Validated records must
remain exact scoped presets, with trial/criteria metadata required by the engine.
External references have no cutting records. Empty local authoring packs are
allowed; they do not produce cutting recommendations.

## Taxonomy and normalization

Application taxonomy revision 1 owns stable `material/<taxon>@1` identities:
wood/MDF/plywood/poplar/birch/hardwood/softwood;
plastic/PMMA/cast/extruded/POM/HDPE/polycarbonate/PVC;
metal/aluminium/6061/7075. Parents describe explicit fallback applicability, not
automatic equivalence. Record material_taxon accepts descendants with a visible
specificity penalty. An exact cast-PMMA record outranks a plastic-family fallback.
Sienci's composite stock category is intentionally unmapped; selecting generic
MDF does not silently qualify every MDF stock against those six references.

Physical tool family/diameter/flutes remain hard applicability criteria; missing
exact identity/diameter/flutes reduce rank. Optional upcut/downcut/compression,
material, coating and shape retain source classification. Such extra traits require
an exact bound tool identity because the CAM cannot infer missing physical traits.
No coating or helix is invented from display names. Current engine remains limited
to supported flat-endmill Profile/Pocket/full-slot/finish contexts; schema retains
surfacing/engraving for future data, without pretending the engine supports them.

Normalized chipload = feed/(RPM*flutes), cutting speed = pi*diameter*RPM/1000,
DOC ratio = DOC/diameter, full-slot radial ratio = 1. Nominal ratios remain nominal.
Derived quantities are clearly marked and original values are preserved.
The normalized block has `derived: true`; `chipload_derived` distinguishes computed
feed/RPM/flutes from copied source chipload targets, and `radial_engagement_derived`
distinguishes full-slot scope derivation from copied nominal source stepover. Diameter
ranges or missing flute counts may leave derived quantities unknown. No arbitrary
interpolation, large-diameter extrapolation or geometry-based engagement inference.

## Matching and reconciliation policy

Policy `fs37-compatible-records-1` calls the existing Engine for each eligible
record, retaining its original mode, limits, rounding, diagnostics and evidence.
All hard applicability checks precede ranking. Lexicographic lower-is-preferred
rank exposes these dimensions: local exact validated/tested preset tier, material
specificity (descendant distance), machine selector exact/family/any, exact tool,
exact diameter/range, known flutes, known DOC, known entry, original profile tier.
Unknown operation does not qualify a record. No unexplained confidence percentage.

Only equally ranked records with the same authored RPM/DOC/stepover, engagement
scope and reduction rules can be reconciled. Their calculated RPM/DOC/stepover
must also agree after constraints. Noncomparable top contexts remain ambiguous.
Never average. Among authored records for the requested mode, Prudent selects
minimum effective chipload, Normal the lower median, Rapid maximum. Deterministic
pack/record ID tie ordering is used only after value/rank ordering. A >25% span
between compatible chipload endpoints produces a visible disagreement notice;
this is an explanation threshold, not a cutting-data correction. Aggressiveness
never converts reference evidence into Tested/Validated or bypasses hard limits.

Explanation retains each pack/record identity, original and normalized classification,
rank, derived values and range. Saved summary contains policy/engine version,
selected/contributing pack/record IDs, evidence and compatible source count only.
No saved private record bodies or derived source quantities.

## Storage, privacy and compatibility

Public bundled registry lives under frontend/public/feeds-speeds/registry.v1.json.
Build review must explicitly add a licensed pack; user imports cannot modify it.
The only initial public pack is the existing six-row licensed Sienci subset.
Native imports are stored through `app.path().app_data_dir()` under
`feeds-speeds/sources`, using the configured application identity. No hardcoded OS
paths or requirement to place data in Git. Root is rejected if inside a Git tree
or using symlinked directories; immutable ID/version JSON files cannot overwrite
existing versions or follow file symlinks. Unix pack files use 0600 permissions.
Standalone browser installs use origin-local IndexedDB; no local dataset is sent
to the remote HTTP transport. Cutting calculations use native IPC or local WASM.
No telemetry, automatic upload, synchronization, backup or pack export is added.

Preferences hold enablement/ID/version references. Legacy custom JSON preferences
are migrated only after validated local storage succeeds; failure keeps their body
for a retry. Legacy sienciEnabled remains a compatibility input; modern Settings
uses a generic registry. Legacy dataset format 1 API requests still use unchanged
single-source Engine policy. Portable v1/v2/v3 files remain readable. New saves
with source-pack summary use guarded `ivac-project-knowledge`, version 4.
Private Apply drops the full proposal snapshot, preserving authoritative rate
slots/geometry plus safe summary. Public Apply keeps compatible public snapshots.
A missing local pack never prevents open/generation or changes saved manual values;
the operation reports unavailable provenance locally. Old apps reject v4 rather
than silently dropping new provenance. No FS4 or new public lab license is implied.


## FS3.8 authoring extension — schema 2

Loader accepts pack schema 1 and 2. Public references retain schema 1. Local
RecordOrigin.lab and lab_history require schema 2; dataset/profile-set version 1
and the Evidence enum remain unchanged. Reserved sorbonne-fablab pack ID requires
private/local-only distribution, a 16-digit ordered timestamp revision and lab
metadata for each record. Draft authoring records are inactive theoretical data;
Tested/Validated attestations and exact scope are validated. History contains prior
full profiles and authoring metadata with the same stable preset ID, not recursive
record snapshots. The loader validates prior snapshots too, rejecting malformed
identity, dates, context, numeric values or qualification declarations.

Native Sorbonne import/save share a marker lock; saves compare expected current
head. Browser transactions provide the same immutable write/head check. Only the
latest Sorbonne revision is loaded; old immutable bodies remain stored. Source
preferences follow latest head without cutting-value mutation. Safe saved knowledge
provenance can resolve prior revisions through retained current history. Explicit
backup exports current pack/history, with private observations; restore refuses
older/equal installed heads. See [authoring](PRESET_AUTHORING.md),
[policy](LOCAL_VALIDATION_POLICY.md) and [ADR-0006](../adr/ADR-0006-local-preset-authoring-and-validation.md).
