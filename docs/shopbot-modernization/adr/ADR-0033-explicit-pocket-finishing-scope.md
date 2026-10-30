# ADR-0033: Explicit versioned Pocket finishing scope

Status: Accepted — bounded implementation software-qualified; bounded software human acceptance recorded; native/physical acceptance excluded
Date: 2026-10-30

## Approval amendment — 2026-10-30

The maintainer approved ADR-0033 and authorized ALPHA-3.2 implementation. They
explicitly waived the requirement to preserve historical projects' executable
behavior. Clauses requiring old-output identity are therefore optional regression
evidence, not an implementation blocker. No automatic destructive file rewrite,
unknown-field fallback or unversioned machining reinterpretation is authorized.
The implementation may keep existing v1 dispatch where this avoids unnecessary
changes; no migration subsystem is required. New v2 scope remains explicit and
all geometry/output guards remain mandatory. No commits or publication.
The proposal text below is retained as the reviewed architecture history.

## Context

ALPHA-3.1 is accepted and locally committed through
`81e8d47d3ca6860303d534393bad37056ea0cfb2`.
`main_finishing_v1` intentionally generates another full reachable Pocket clearing
pass. In particular, positive lateral allowance with zero floor allowance does not
mean boundary-only finishing. Reinterpreting these saved values would change CNC
output without an authored decision.

The existing nested stage boundary is strict in Rust JSON, frontend project and
preset-snapshot validation, operation templates and the explicit WASM pre-conversion
guard. Negative probes reject both an added scope inside v1 and an unknown stage
discriminator. An unrelated optional field is not safe: portable operation extension
fields and general WASM fields can be accepted without altering the v1 program.
[Preflight and source-bound evidence](../../releases/ALPHA_3_2_POCKET_FINISHING.md).

## Accepted decision (historical-preservation waiver above applies)

1. **KEEP** portable project v10, native `cutting_version: 10`, collection v2 and
   operation-template envelope v1. Preserve drawing v1, diagnostic/snapshot v2 and
   existing CAM protocol. There is currently no demonstrated need for project v11.
2. **EXTEND** the existing `MainFinishing` tagged union with a Pocket-only variant,
   proposed discriminator **`pocket_finishing_v2`**. Retain `main_finishing_v1`
   unchanged for Profile and historical Pocket operations. No new CAM engine.
3. The new variant has required `main` and required nullable `finishing` as before.
   A non-null finishing value additionally requires a typed `scope`:
   `walls_only`, `floor_only`, or `walls_and_floor`. Preserve the explicit cutter,
   rates and depth-per-pass fields. Unknown/missing fields and scope values refuse.
4. Keep both allowance fields explicit in the transport. `walls_only` requires
   `floor_allowance_mm == 0`; `floor_only` requires `lateral_allowance_mm == 0`.
   The UI displays only applicable inputs. Contradictory authored values refuse;
   changing scope previews the removed allowance rather than silently discarding it.
5. Advertise **`pocket-finishing-v2`** in addition to `cutting-parameters-v10`.
   Requests containing the new variant require both capabilities, across normal,
   worker, streaming, two-sided and drawing-machining adapters. No fallback to v1.
   Rust validates the actual discriminator even if a client skips capability checks.
6. Opening/saving an existing v10/v1 operation keeps its contract, values and
   executable behavior. Show historical Pocket finishing as **Full-area finishing
   (existing behavior)**, with an explicit zero-allowance repeat indication.
   Moving to a new scope is a deliberate atomic, undoable edit with a before/after
   summary; it invalidates generated/export qualification. It is not a file migration.
7. New ordinary Pocket operations may author v2 with Finishing OFF, after approval.
   Profile and unaffected families keep their existing contracts. A v2 contract on
   an unsupported operation refuses; a legacy Pocket is never upgraded just by opening.
8. Presets, loaded snapshots and operation templates preserve the discriminator
   and scope verbatim. Context matching includes them, replacing today's hard-coded
   v1 signature. Do not transfer a historical tested attestation to a new scope;
   no cross-context reuse or ALPHA-4 behavior is introduced. Existing collections
   are preserved; old readers reject new nested values before storage/application.
9. `R` remains independently retained stock. Walls-only Main targets `(R+L,Z)`;
   floor-only Main `(R,Z+F)`; combined Main `(R+L,Z+F)`. Finishing targets `(R,Z)`
   with scope-specific obligations. Positive allowances remain >=0.05 mm; 0.02 mm
   output bounds remain unchanged. Zero applicable allowance is an explicitly
   acknowledged/labeled repeat, never evidence of reserved-stock removal or finish
   quality. Floor material-removal claims require a positive floor reserve.
10. Preserve v1 dispatch/proofs for unchanged authored requests. New wall-only paths
    must independently prove lateral **and axial** coverage and safe access. New
    geometry acceptance is evidence-driven; a parsed v2 request is not CAM eligibility.

This is a new incompatible **nested machining contract**, despite unchanged outer
version numbers. Approval was recorded above; implementation and qualification follow ALPHA-3.2.

## Alternatives considered

| Alternative | Assessment |
|---|---|
| Infer walls-only from L>0 and F=0 | Reject: changes existing executable semantics |
| Add an optional field outside guarded stages | Reject: observed ignored-field behavior in portable/WASM readers |
| Add scope to v1 without a discriminator change | Old strict readers reject it, but overloading one discriminator obscures historical executable meaning |
| Project v11 / cutting v11 / collection v3 everywhere | Not required by tested nested guards; reserve as fallback if an actual reader bypass is demonstrated |
| Separate new machining engine/operation family | Unnecessary duplication; extend existing stage planner and geometry proof |

## Consequences

The compatibility boundary is narrow, but all consumers must handle the union
explicitly: schema/codegen, operation creation, rebinding, save/load, presets,
templates, cache/freshness, stage attribution, simulation/report and all transports.
Old readers cannot edit new-scope files; v1-only files remain compatible. Preserve
original user files and permit a separately named save on deliberate conversion.
No implicit downgrade or universal legacy migration subsystem.

No ShopBot-specific behavior belongs in this type. Existing same-tool RPM,
toolchange, SAFE-Z, placement, resource and export rules remain mandatory.
This is a milling Pocket extension, not a generic waterjet/process abstraction.

## Validation required after approval

- Archive actual pre-change readers; prove unknown variant and all scope refusals,
  including disabled operations and preset snapshots, before mutation/publication.
- Keep exact v1 program, motion, attribution and save/reopen regression references.
- Prove v2 round trips and no field loss in native JSON, real WASM, worker, HTTP,
  native commands, streaming, two-sided, drawing-machine, templates and presets.
- Missing capability must refuse before dispatch. Mixed contracts, unused positive
  allowance and malformed fields must refuse; no automatic conversion or rounding.
- Scope conversion/Undo/Redo, generation staleness, report/provenance and source
  identities must survive save/reopen. No transferred physical attestation.
- Independent wall-height/access, floor-area, curved/island and decoded-path proofs
  are separate requirements; passing serialization does not qualify them.
- Run actual source-bound canonical gates and browser walkthroughs. Native installed
  acceptance and physical CNC remain distinct. If any supported reader bypasses
  the nested guard, stop and propose the minimum required outer-version increment.
