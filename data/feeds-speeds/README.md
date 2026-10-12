# Feeds & speeds data — format 1

`sienci-router-reference.v1.json` is a small, explicitly loaded external reference
subset. It is **not a ShopBot dataset or locally calibrated cutting advice**.
The library never embeds, loads or chooses this file automatically.

## Attribution and licensing

Adapted from **Sienci Labs, LongMill MK2 Feeds & Speeds — Metric**, page 1,
Single Flute Flat End Mill UC rows, under **CC BY-SA 4.0**:

- [Pinned source PDF](https://github.com/Sienci-Labs/Resources/blob/060e2291a1cf17bf9be4f151d7aa539a42ed2b0c/_downloads/FeedsSpeedsMetric.pdf).
- [Pinned source handbook](https://github.com/Sienci-Labs/Resources/blob/060e2291a1cf17bf9be4f151d7aa539a42ed2b0c/cnc-fun/handbook/cnc-feeds-speeds.md).
- [Pinned upstream license](https://github.com/Sienci-Labs/Resources/blob/060e2291a1cf17bf9be4f151d7aa539a42ed2b0c/LICENSE).
- Local license text: [CC-BY-SA-4.0.txt](CC-BY-SA-4.0.txt).

Retrieved 2026-10-12; source publication date unspecified. Source PDF SHA-256
is recorded in every profile. Keep attribution, source links, changes and the
CC BY-SA license when redistributing/adapting this dataset. This data license
is separate from the GPL-3.0 Rust implementation. No endorsement is implied.

Changes: transcribed six metric rows, mapped Reduced/Regular/Full Speed to
Prudent/Normal/Rapid, converted 1/4 and 1/8 inch exactly to 6.35 and 3.175 mm,
added immutable identities, catalogs, evidence and restrictive matching scope.
Feed, plunge, RPM and slotting DOC values are unchanged. No chipload-derived
feed, numerical reduction, interpolation or fabricated machine limit.

## Coverage and limits

One **source-defined softwood / soft plywood / MDF category**, two single-flute
flat upcut reference geometries, three source modes, full-width slotting only.
The handbook describes low-hardness materials dentable with a fingernail.
The category is not an automatic mapping for every MDF/plywood or lab stock.
Source machine context is LongMill MK2, not a real lab machine inventory entry.
Tool identities describe source geometry, not a lab tool inventory or proven SKU.

Exact source DOC is required; missing/different DOC produces no match. This
narrow initial policy avoids inferring a physical applicability envelope from
a single table target. Nominal pocket stepover does not guarantee physical WOC;
pocket DOC/stepover columns are deliberately excluded. No finishing, surfacing,
V-bit, compression, two-flute, metals/plastics, or unknown-safe fallback profile.
Entry strategy and thickness are unspecified; they are not claimed qualified.
Machine/tool maxima stay unknown, separate from preferred cutting targets.
The caller supplies known constraints; any engine adjustment remains visible
as `AdjustedOutsideEvidence`, with original `ReferenceData` evidence retained.

Every profile is ReferenceData. Manufacturer testing described in the handbook
is not a locally recorded Tested/Validated preset. New lab mappings/expansion
need explicit source review or FS4 calibration; changing a file in place under
the same immutable revision violates the authoring contract. No updater is built.

## Distribution contract

JSON format_version 1 wraps description/license, machines/materials/tools
catalogs and the domain schema_version 1 ProfileSet. See public DatasetDocument
and nested domain types in `crates/ivac-feeds-speeds/src/dataset.rs` and `model.rs`.
Named unit suffixes are the schema; only metric millimetres/minutes/RPM and
ratios in (0,1] are accepted. All supported fields are explicit Serde models;
unknown fields and unsupported versions fail rather than being ignored.
Catalog IDs are unique within kind and include immutable revisions. Exact
scope references and family memberships must resolve to catalogs. Exact tool
scope must agree with catalog geometry/flutes; a target above a declared tool
max is rejected. Labels never match. There is no inheritance, missing-parameter
completion or external reference resolution. Any invalid row rejects the file.

Call `load_dataset_json(&contents)` to obtain an immutable document and Engine;
the host handles file I/O and explicit dataset selection. Use `document()` for
catalog display and `engine()` for proposals. Reopening saved CAM values remains
FS3; loading a changed dataset does not itself apply anything to a project.

Software tests use the data file as a caller-supplied fixture, plus separately
labelled synthetic local trial records. Synthetic presets are not distributed.
See [FS2 qualification](../../docs/shopbot-modernization/feeds-speeds/FS2_QUALIFICATION.md).
