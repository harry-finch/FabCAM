# GEOM1 serialization and transport contract

Revision 1, 2026-10-15. See ADR-0021, GEOM1_CONTRACT and GEOM1_RESULTS.

## Project envelope

Edited drawings require `kind: ivac-project-geometry`, `version: 7`. The existing
registry remains v1 generic, v2 FS, v3 cutting, v4 knowledge, v5 CAD relief, v6
surfacing. `drawing` is forbidden under these older envelopes. A v7 without drawing
is invalid. Unedited files retain the existing feature-dependent save-kind choice.
Explicit Reset removes the edited snapshot and restores the original source bindings;
that user action can return the document to the appropriate earlier format.

The drawing state contains revision 1, the import/source/transform freshness key,
original operation source bindings and a Rust response. That response contains:

- immutable imports (ID, source primitives, original pivot, similarity transform,
  monotonic next entity ID and original UI object tags for legacy conversion);
- accepted contours with stable IDs, edge IDs, source ordinals and parent lineage;
- merge/split/delete transitions;
- evaluated geometry, per-segment contour membership, closure and analytic bounds;
- diagnostics and most recent response metadata.

The evaluated display cache is **not trusted on load or generate**. File opening
reevaluates accepted local geometry in Rust and compares original primitives,
import count, transform and pivot with the file's original imports. Generation again
evaluates the accepted drawing. Unknown edit/transform fields and unsupported
contract revisions refuse in Rust. Malformed IDs, duplicate live provenance, missing
imports, impossible lineage and nonfinite values refuse. The saved format is an
editable project document, not a cryptographically signed machining certificate.

Unknown top-level inert metadata is preserved by restore/snapshot. Known fields keep
their existing migration/default rules; a future geometry feature must obtain a new
guarded contract, not hide in an inert extension. Compatible relief, cutting,
knowledge, tools, profiles, text, surfacing and operation data remain serialized even
when their combination with drawing repairs is deliberately not machinable yet.

## Transport envelope

`POST /drawing`: a tagged Start/Evaluate/Preview request, protocol `version: 1`.
Start supplies immutable imports plus the legacy work offset, auxiliary geometry
and live text needed for source-ordinal proof. Evaluate computes the accepted result;
Preview clones and validates a proposed batch. The response reports version 1.

`POST /generate-drawing`: `{version: 1, drawing, bindings, machining}`. `machining`
contains the existing PipelineRequest. There is deliberately no top-level `project`
field: pre-GEOM1 PipelineRequest decoding fails instead of discarding an optional
unknown edit field. The actual legacy HTTP endpoint is tested and returns 422.
Bindings name operations explicitly; duplicates/missing/unknown associations refuse.

Equivalent Tauri commands are `evaluate_drawing_cmd` and `generate_drawing_cmd`;
WASM exports are `evaluateDrawing` and `generateDrawing`. Missing function/endpoint
or incompatible response version fails explicitly. Frontend clients never retry via
a legacy generator. All three routes invoke the same Rust implementation. Native
CPS protections and browser/WASM CPS limitations are unchanged.

## Lifecycle

A transient preview is not serialized or exported. Apply stores the accepted
snapshot and adjusted bindings in a single history command. Undo/Redo is session-local;
opening a project clears old history but preserves accepted geometry/bindings.
The generation freshness key includes drawing state, stable bindings and conflicts.
Saving does not clear machining staleness. Watched source replacement does not replay
edits: the accepted snapshot survives with an explicit source conflict. Existing
file-transform changes require reset/reanalysis in this initial GEOM1 scope.

Reset is refused while imports/transforms differ from the accepted source key, or
when a newly created operation has no proven original explicit selection. Undo the
source change, or select intended dynamic layers for that new operation, before
resetting. Original operation IDs remain reserved while reset provenance exists,
including Add/Duplicate/template creation, so deletion/recreation cannot reuse an
old reset binding. Orphaned stable/unresolved bindings cannot generate or be saved
under v1–v6. Tests cover these reset/downgrade and malformed-binding counterexamples.
