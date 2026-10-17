# Feeds & Speeds audit improvements — 2026-10-16

Status: implemented locally, software-qualified; human review pending. Tracker: `ivac-klwgj`.
Scope: user-supplied module audit. No commit, push, release or artifact distribution.

## Changes and decisions

| Audit concern | Implemented behavior |
| --- | --- |
| ExactValues at reduced RPM | Feed follows the downward RPM ratio, including integer RPM rounding. A 24,000 RPM / 2,400 mm/min profile capped to 12,000 now proposes 1,200 mm/min. Raising RPM to a minimum never raises the authored feed. Adjustments remain outside declared evidence. |
| Feed ceiling | A diagnostic reports effective and pre-ceiling target chipload. No universal material rubbing threshold or automatic RPM retuning is inferred. |
| Direct entry | Generic entry profiles explicitly warn that axial plunge suitability is not established. No ramp-angle schema or inferred center-cutting capability added. |
| Diameter adaptation | Smooth transition between linear and squared reductions replaces the 10% jump. Larger-cutter source ceilings remain deliberate. |
| Candidate order | Explicit distinct quality costs; typed `CandidateRank` retains lexicographic order and the 12-number serialized array. |
| Interpolation | Anchors must share context specificity and adaptation policy; a better-ranked anchor cannot donate its specificity to an unequal pair. |
| Floating-point reconciliation | Absolute 1e-9 tolerance for equivalent numeric recommendation fields; integer outputs, identity/scope checks and safety bounds remain strict. |
| Trace | Shared recommendation evaluation populates the trace once. Invalid inputs return typed errors or empty unavailable traces; unchecked expectations removed from this path. |
| Dataset validation | Direct document validation avoids the JSON encode/parse roundtrip. A streaming serialized-size check retains the pack boundary without a full JSON buffer. |
| Taxonomy | Internal matching traverses a static borrowed table; public owned taxonomy API retained. |
| Errors | Additive checked APIs expose `SourcePackError`; existing String-returning adapters remain compatible. |

The adaptation formula uses `ratio = min(requested/source, 1)`,
`delta = abs(source-requested)/requested`, and
`transition = clamp((delta - 0.1)/0.4, 0, 1)`.
The reduction is `ratio^(1 + transition)`; DOC also retains its source ceiling.
This is a documented heuristic, not physical qualification. Between 10% and 50%
it can be less restrictive than the former squared rule. Candidate changes and
interpolation boundaries can still cause changes in recommendations; global
continuity is not claimed. The existing interpolation-boundary regression remains.

New calculations carry `fs1-policy-2`. Existing source-pack policy identifiers
remain compatible. Saved explicit parameters and manual overrides remain authoritative;
there is no automatic recalculation on reopen and no postprocessor dependency on FS.
Rust callers using rank-vector indexing must use named fields; the JSON array contract
is unchanged. Schema regeneration also aligns the previously stale OpenAPI information
version with the existing workspace version 0.9.0-beta.1; no release bump occurred.

## Validation evidence

Logs are retained in [fs-audit-evidence](fs-audit-evidence/).

- Before correction: the new RPM-cap regression failed (actual feed 2,400, expected 1,200).
- Engine: **107 passing tests**, including one doctest. Covers cap/rounding, diagnostics,
  transition continuity with integer rounding, anchor specificity/order, malformed input,
  direct validation, typed errors, rank serialization and numerical reconciliation.
- CAM integration: **17 passing tests**, including manual values, explicit application,
  offline generation and save/reopen authority.
- Frontend: **1,324 passing / 1 skipped**, plus **10 Node tests**; Svelte/TypeScript
  reports zero errors and warnings.
- Production frontend/WASM build passes. Vite retains bundle-size and mixed
  static/dynamic Tauri import warnings; these are not qualification failures.
- Formatting and whitespace checks pass. No performance speedup is claimed without
  a benchmark. No new rendered UI acceptance or physical machining test is claimed.

## Handoff and release separation

Review new recommendations and diagnostic wording before accepting this checkpoint.
The existing REL1-B1 DMG predates these FS changes and has not been rebuilt here.
Its clean-download/install/runtime gate remains open on the available MacBook M3,
macOS 14.6; Windows and Ubuntu tests remain planned separately. Software checks here
do not qualify those installers or cutting conditions.

**HARD STOP:** await human review; no next milestone, commit, push or publication.

## Follow-up review — four micro-details, 2026-10-16

Implemented within the same uncommitted policy-2 checkpoint:

1. Adaptation and rejection trace share taxonomy ancestor matching. Hierarchical
   transfer ranking uses the nearest explicitly permitted target scope plus one
   transfer cost. It does not infer suitability from the source name alone. Tests
   cover aluminium → 6061, unrelated wood rejection, selection and order stability.
2. Absolute plunge is reduced by final RPM / authored RPM for both data modes,
   before its own ceiling and floor rounding. It never increases at an RPM floor.
   The adjustment explicitly says `rpm_reduction_preserves_plunge_per_revolution`:
   axial advance per revolution is not a physical model of axial chip thickness.
3. Profile and tool-classification BTreeMap indexes are built once per pack
   validation. Missing classifications no longer mask a later known classification;
   contradictory known classifications for one tool identity reject independently
   of record order. This removes the nested record/profile search from each tool
   adaptation check. Catalog lookup remains linear; no whole-loader complexity or
   benchmark speedup claim is made.
4. ExactValues at higher enforced RPM reports reduced effective chipload relative
   to the authored target, also when a feed cap applies. Diagnostic translations
   now mention RPM or feed limits. Feed is never increased to compensate.

New regressions cover both data modes, machine/tool RPM caps, floor behavior,
combined floor/feed cap, fractional rounding, taxonomy and classification ordering.
Native installer, physical qualification and human acceptance remain open.

Follow-up validation: `RUSTC_WRAPPER='' cargo test --offline -p ivac-feeds-speeds`
passes **111 tests including one doctest**. `RUSTC_WRAPPER='' cargo test --offline
-p ivac-core --features cps --lib feeds_speeds` passes **17 tests**. Frontend
`npm run check` reports **0 errors / 0 warnings**. `cargo fmt -p ivac-feeds-speeds`
and `git diff --check` pass. Follow-up logs are in the evidence directory. The full
frontend suite and production build recorded above predate these four follow-up
changes and were not rerun; no new build/installer acceptance is claimed.
