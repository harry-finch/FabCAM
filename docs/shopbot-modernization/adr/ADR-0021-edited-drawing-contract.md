# ADR-0021: Source-preserving drawing edits and stable CAM bindings

Status: Accepted in principle — mandatory refinements formalized before implementation
Date: 2026-10-15

## Approved refinements

User explicitly approved phases A–F. [GEOM1_CONTRACT.md](../GEOM1_CONTRACT.md)
is normative for identity/lineage, authoritative Rust evaluation, coordinates and
v7/transport compatibility. Conservative unresolved bindings replace any ambiguous
automatic merge/split/delete remapping. [Implementation evidence and limits](../GEOM1_RESULTS.md) record the bounded GEOM1 delivery.

## Context

GEOM1 changes topology. Import objects and CAM objects currently use positional
chain IDs; multi-import offsets and whole-pool rechaining can change their meaning.
A successful numeric ID lookup is insufficient proof of source identity.
[Audit and qualification contract](../GEOM1_AUDIT.md) records the actual seams,
import losses, transform-pivot risk and existing lifecycle limits.

## Approved decision

Preserve original imported geometry and store one accepted edited snapshot per
import with stable segment/contour identity and lineage. Keep pending proposals
transient. Use a pure generic Rust geometry evaluator and one authoritative
frontend effective drawing, consumed by every rendering and generation path.
Resolve stable selections at the shared CAM boundary against real chain provenance;
reject ambiguous topology mappings. Legacy projects without edits keep their
existing semantics. First-edit binding conversion must prove source membership
or require explicit reselection; endpoint equality is not identity.

Order edits before file transforms (using the original pivot) and before explicitly
reapplied corner relief. Initially refuse relief/affected authored-marker conflicts.
Keep meaningful POINTs, arcs and live text intact outside qualified repair scope.
Batch geometry/reference changes in one Undo/Redo action; preserve manual cutting
values. Revalidate pending proposals and watched-source conflicts before Apply.

Introduce a guarded next project kind/version for edited drawings (proposed v7,
subject to checking the version registry at implementation). Require equivalent
backend capability/validation. Older readers must refuse rather than ignore edits.
New loader accepts versions 1–6 and all compatible surfacing/cutting/relief data;
new format validation must explicitly allow their combinations, replacing existing
version-equality checks where needed. Unedited saves retain their existing format.
No silent downgrade or automatic edit replay on a changed external source.

## Alternatives considered

- Mutate imported segments directly: loses original geometry; positional IDs and
  changed transform pivots can move/rebind unrelated geometry.
- Frontend-only evaluated overlay modeled solely on corner relief: no authoritative
  stable source contract through CAM rechaining; insufficient for topology edits.
- Limit to one unreferenced import: smaller, but fails the intended multi-drawing
  lifecycle. Available fallback only if explicitly accepted as reduced scope.
- Full parametric CAD history/entity rewrite: unnecessary for cleanup and too broad.

## Consequences

Additive project/API/type/transport work is required, plus a narrow provenance-aware
extension to source resolution. Reuse primitives, transforms, command bus, source
combination, CAM planning and posts. Do not change legacy fuzzy tolerance globally
or authorize extra ShopBot operations. Fork delta remains in geometry preparation,
source binding and lifecycle rather than machine output logic.

Stable contours are logical editable groups, not DXF/SVG authoring entities recovered
from a flattened import. Unsupported mapping must fail explicitly, including
synthesized text/pattern/frame interactions. Implementation complexity is real;
this decision is not a claim that existing IDs already satisfy the contract.

## Validation

Reference cases, numerical bounds, lifecycle and compatibility matrix are in the
audit. Independent checks must cover continuous simplification distance, immutable
originals, unchanged bindings of unrelated imports, generated-source membership,
legacy project behavior, rejected old readers/backends and decoded-output/preview
correspondence. Revisit the design if any mapping requires geometric guesswork or
if the shared resolver cannot preserve source-combine order and legacy semantics.

Architecture gate ivac-rcmta accepted. Implementation ivac-8rura authorized;
HARD STOP after GEOM1 qualification before the next milestone.
