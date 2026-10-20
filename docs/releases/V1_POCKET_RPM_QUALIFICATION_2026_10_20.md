# V1 Pocket rough/finish RPM boundary

2026-10-20 · `ivac-2zll` · bounded software qualification, no physical claim.
Regression: `v1_freeze_*` in `crates/ivac-core/tests/shopbot_m2_post.rs`.
Four tests passed in the targeted run; the integrated run owns the final receipt.

## Exact rule

The bundled ShopBot post allows supported RPM at section entry but refuses a
`spindleSpeed` record inside a section. CpsRecorder initializes each section's RPM
from its resolved tool context; equal reassertions emit no record. A genuine
change emits a record rejected by whole-program preflight, before successful
program output. The diagnostic now identifies the operation and explains the
intra-section limitation and explicit choices. No output command, tool identity,
RPM setting, machine scaffold or success/refusal boundary was changed.

A distinct valid finish tool plus actual finish offsets causes the dual-tool
driver to create a tool-change section, preserving its independently resolved
RPM. Reusing the same tool ID does not create that boundary. Declaring a finish
tool with `pocket_nocontour` and no wall ring does not execute a tool change or
that unused finish RPM. Values below refer to effective resolved RPM after saved
cutting settings and machine constraints, not merely text shown in the tool form.

| Actual Pocket case | Manual spindle / manual_m0_pause | Lab spindle 1 / manual_m0_pause |
|---|---|---|
| One tool, 17000 rough / 17000 wall | Pass | Pass, preloaded flat tool 1 |
| One tool, 17000 rough / 10000 wall | Refuse intra-section spindleSpeed | Same refusal |
| finish_tool_id equals primary ID, different wall RPM | Same refusal; no split | Same refusal |
| Distinct tool 2, 17000 rough / 17000 finish | Pass; manual stop/load/Z-zero/start pauses retained | Refuse tool 2/tool changes |
| Distinct tool 2, 17000 rough / 10000 finish | Pass; tool 2 and requested finish RPM retained | Refuse tool 2/tool changes |
| Distinct tool 2 at 10000, primary's unused wall override 17000 | Pass with tool 2 at 10000 | Refuse tool changes |
| Distinct declared finish tool, no wall ring | Pass rough-only, no tool 2/10000 output | Pass rough-only with preloaded tool 1 |

Each of the first six cases is run twice per spindle mode and compared for exact
success-output or refusal-text determinism. Additional positive coverage exercises
two separate operations with tool 1/17000 and tool 2/10000 and checks both manual
RPM instructions. ATC, manual M6 prompt and Ignore profiles remain refused:
the bundled post requires `manual_m0_pause`. All unrelated tool, stock, origin,
entry, provenance and numeric guards still apply. Tests are real CAM→bundled CPS,
not evidence that every geometry/finish strategy or machine profile is qualified.

## V1 disposition

Retain the supported combinations and the explicit unsupported intra-section
transition. The improved message asks the user to choose equal rough/finish RPM
explicitly for a single tool or use separately qualified manual section boundaries;
it never edits rates or substitutes tools. Pipeline failure is an error, not a
partially successful program. Custom CPS and new spindle-control behavior remain
deferred. Lab control remains wiring-specific, not generic spindle automation.

`ivac-2zll` is documented as a proposed V1 exclusion with positive supported
controls, **not automatically closed or human accepted**. The final freeze decision
must accept this scope. Supporting new intra-section spindle events would change
the machine-output contract and requires a separate authorization/qualification.
