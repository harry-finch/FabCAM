# Targeted HATCH + SAFE-Z corrections

2026-10-18 · **LOCAL SOFTWARE CORRECTIONS — HUMAN REVIEW PENDING**
Engineering ivac-ds9ck; human gate **ivac-th8y2 OPEN**.
User approved the contracts and these two targeted fixes. TEST1 has **not** started.
No commit, push, package, publication or machine execution.

## Implemented boundaries

**HATCH / ivac-aoany.** Before DXF expansion or recovery, an allocation-free
ASCII group scan detects HATCH entities inside BLOCKS/BLOCK definitions. The complete
import fails with an actionable explode/re-export hint. It includes unused definitions
and legacy-encoded names, rather than returning displaced or silently missing contours.
Top-level HATCH recovery, metadata and ordinary block expansion are preserved. This is
an explicit unsupported-input refusal, not new transformed/nested HATCH support.

**Known-stock clearance / ivac-jnom4.** Shared milling preflight compares operation
and header clearance with the declared stock top, in absolute program-frame mm.
It runs before geometry emission/cache reuse in buffered and streaming entries.
It rejects nonfinite/equal/below/rounding-ambiguous clearance and invalid stock-top values.
Disabled operations are ignored; header-only jobs and Homing safe-Z are covered.
No automatic value adjustment, new Z-origin mode or operation serialization change.
Non-milling processes and unknown-stock behavior remain unchanged and unqualified.

The guard accounts for standard four-decimal output (0.0001mm, or inch precision
converted to mm) plus1e-8mm arithmetic uncertainty. Identity-Z custom formats are
also checked after rounding, so a three-decimal format cannot erase a small gap.
This is a numerical serialization bound, **not a physical clearance recommendation**.
Custom renamed/scaled/disabled axes, arbitrary CPS scripts and controller macros are
not qualified as faithful work-Z output by this preflight.

A structured `UnsafeClearance` error identifies the operation (0 for program header),
actual clearance/top and corrective action. HTTP `/generate/gcode` performs this
cheap preflight before committing headers: invalid clearance returns400 with no
program, rather than200 followed by an aborted stream. Other pipeline entries use
the same guard; asynchronous execution/worker/streaming architecture is preserved.
Existing unsupported numeric Z, two-sided, parking/station and ShopBot strategy/tool
guards remain intact. No ShopBot-specific rule was added to geometry.

**Cache compatibility:** pipeline version58→59, stable-key snapshot deliberately
updated. Saved models and upstream attribution unchanged; earlier audit changes retained.
Existing distributed/test candidates remain pipeline54, not this source.

## Red → green evidence

[Commands, raw logs and fingerprints](hatch-safe-z-2026-10-18-evidence/README.md).
The initial three regressions FAILED on pipeline58 before correction: block HATCH
successful misplaced import, insufficient known-stock clearance accepted, and streaming
invalid job wrote output. All now pass with their intended refusal assertions.

Fourteen focused tests cover top-level/block/unused/legacy-encoded HATCH; equal,
negative, nonfinite and rounding-edge clearances; disabled operations and unknown
stock; header-only jobs; stock edits after cache hits; unchanged input values;
metric/inch/custom-format precision; milling-only behavior; byte-stream and HTTP
refusal; qualified Profile/Pocket/Drill; two clearances/tool change/XY translation;
independent OpenSBP modal-Z decoding and known-position preview rapid checks.
HTTP adds its own regression; legacy CPS zero-clearance now has an explicit refusal test.

The legacy GRBL post-format snapshot used a zero-clearance synthetic fixture. The
format-only oracle now explicitly has **no declared stock**, so it makes no safety
claim; its old digest remains byte-identical. The known-stock geometry-parity fixture
uses explicit5mm clearance. A separate regression asserts that restoring declared
stock to the legacy zero-clearance case refuses. No expected digest was weakened to
accept unsafe known-stock output; optional FANUC remains unexecuted when absent.

Fresh results (final evidence manifest authoritative):

| Check | Result |
| --- | --- |
| Focused HATCH/SAFE-Z | 14 PASS |
| Core with CPS | 1,202 PASS serial |
| Twelve integration suites including focused tests | 116 reported PASS /115 executed; optional FANUC absent |
| HTTP | 16 PASS, including structured pre-stream refusal |
| WASM native bridge | 22 PASS; wasm32 compilation PASS |
| CPS / independent Node | 46 reported /40 executed; 39 Node PASS |
| Frontend lifecycle/state/package tests | 1,328 PASS,1 skip;10 Node package-state PASS |
| Static | Typecheck, core/adapter Clippy, fmt and diff whitespace PASS |

No new browser UX screenshot review, packaged native install or physical test claimed.
Tauri is compile/Clippy checked here; previous native watcher qualification is retained,
not represented as a fresh installation/runtime run.

## SAFE-Z contract coverage and remaining limits

| Contract | Evidence in this targeted checkpoint | Remaining limit |
| --- | --- | --- |
| Z01 | Profile/Pocket/Drill ordinary air traversals; valid positive clearance retained | Not all operations or controller cycles. |
| Z02 | XY-shifted two-operation OpenSBP witness; original project unchanged | Existing frame regressions retained. |
| Z03 | Nonzero numeric Z refusal retained; nonfinite clearance/top refused | No numeric Z capability enabled. |
| Z04 | Original18/5 witness now fails with known/unknown work area, before any output | Targeted defect resolved in software. |
| Z05 |23mm positive control, equal/below/near-rounding and inch/custom-format cases | Does not certify arbitrary custom output transformations. |
| Z06 | Existing fixture/holder warning regressions retained | **Not newly qualified**: fixture collisions remain governed by existing warning/export policy. No unconditional whole-trajectory fixture refusal added. |
| Z07 | Two operations/tool change/clearance heights, standard generic park; ShopBot park refusal retained | Initial position unknown; controller/machine-frame stations not a proved work-frame trajectory. |
| Z08 | Cache edit/restore, zero-byte streaming refusal, HTTP400 and existing frontend lifecycle suite | No new packaged UI end-to-end run on all adapters. |

**Full SAFE-Z qualification remains open under ivac-j09eh**, especially fixture/holder
policy and all transition semantics. Do not equate a valid clearance configuration
with a collision-free program. The targeted guard does not reject legitimate vertical
retracts from cut depth: decoded witness tests distinguish upward retracts from air
traversals instead of requiring every rapid endpoint above stock. No new material-
removal proof engine or universal swept-volume certification was introduced.

Human review must accept these two bounded fixes and their limits before TEST1.
Broader SAFE-Z release evidence, platform clean-host checks, separate UX reviews and
physical ShopBot trials remain required. No permission to machine or publish inferred.

**HARD STOP — awaiting validation; TEST1 implementation not launched.**
