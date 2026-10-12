# FS3.5 — ownership audit and smallest coherent redesign

2026-10-12. User explicitly authorizes this checkpoint, not FS4 or publication.
Audit completed before implementation. Recommendation arithmetic and owned
ShopBot post remain unchanged.

## Existing ownership and consumers

| Concern | Storage / serialized form | Consumers / disposition |
| --- | --- | --- |
| Physical cutter | core project/tool.rs ToolEntry, frontend ToolEntry; embedded portable/project tools and workspace inventory/profile copies | Diameter/type/flutes/length/holder/wear feed CAM/simulation; KEEP |
| Legacy cutting defaults | ToolEntry speed/feed_rate/plunge_rate, finish/drill variants, default_step/default_xy_overlap; frontend camelCase equivalents | ToolRowSummary/ExpandedEditor and tool presets write them; setup_resolver/saved_rates/effective_step and pocket builder inherit for absent operation blocks; KEEP as explicitly labeled fallback defaults |
| Operation rates | Optional OpParamsCommon.applied_cutting: complete per-tool/per-pass slots; older feed/plunge overrides | Core saved_rates/header/setup/CPS read authoritative snapshots; EXTEND for manual-only values |
| Operation geometry | params step/depth_list and pocket xy_overlap, inheriting tool defaults only when absent | Sole DOC/overlap authorities; KEEP, present inside cutting form without duplicating saved values |
| Recommendation | AppliedCutting.proposal full engine snapshot, context_key; frontend preview ephemeral response | Saved proposal preserves provenance, per-field origins persist; frontend appliedContextKey and core key detect staleness. Proposal currently mandatory, preventing independent manual-only cutting block; EXTEND to optional |
| Material | Existing project material Identity; stock is separate dimensions/placement | FS3 assistant selected material per operation then applied globally. REUSE identity at job/stock level, not a second material type |
| Sources | Sienci public JSON asset or operation-local uploaded contents | No persistent source configuration; MOVE to existing per-installation ivac.settings, load enabled data without invoking engine |
| Machine/tool reference matches | Project fsContext identity bindings and optional tool hard limit | No automatic equivalence: source LongMill is not ShopBot. Add explicit frontend machine/tool reference metadata; existing bindings remain readable |
| Tool availability | workspace.tool_inventory is library; project.tools is id-preserving machine available subset; workspace machine profiles snapshot subset | stockTool/syncStockedFromInventory manage copies and collisions. KEEP model; show available tools first, library picker second |

Portable legacy files are ivac-project/v1; FS3 files ivac-project-fs/v2. Tool
library/inventory/profile persistence copies definitions unchanged. Removing tool
cutting defaults would reinterpret old projects, affect drills/non-milling and
break inheritance. No automatic migration/removal is justified.

## Chosen redesign

Reuse AppliedCutting for resolved operation rates; make its proposal optional
so manual operation values do not require a recommendation or fabricated source.
Keep origins/context internally. Manual-only blocks do not produce recommendation
stale warnings. Recommendation Apply still snapshots exactly the same rates,
DOC/overlap and provenance. A guarded portable cutting format protects old FS3
readers that assumed a proposal. Existing v1/v2 files load unchanged. ADR-0004
records this small persistent ownership extension.

Default operation form: current RPM/feed/plunge, native pass depth and pocket
stepover, plain Modified labels, optional Suggest/Review action. Profile shows
its actual finish role as the main cut; Pocket shows rough role with secondary
finishing values under a disclosure only when relevant. Geometry/source/tool/depth
precede cutting; tabs/leads follow; depth schedule/directions stay advanced.

Focused recommendation panel shows known machine/material/cutter/operation,
DOC/engagement, independent mode/evidence/source, explicit Calculate/Apply/Cancel.
No source loading or origin enum strings in the normal panel. Enabled sources
are configured once in Settings; multiple available source suggestions remain
explicit choices, never a new hidden cross-dataset precedence rule.

Job material uses enabled source catalogs and preserves previously saved
identity when a source is disabled. Explicit reference compatibility is secondary
machine/tool setup, never inferred from names or dimensions. No lab scope is
invented. Sources unavailable/disabled produce a readable assistant state;
manual parameters and generation remain independent of source configuration.

Tool Library prioritizes physical columns; fallback speeds/step remain available
under clearly labeled compatibility disclosure. Machine tooling prioritizes the
available subset and Add tool from library; inventory remains intact.

## Qualification to establish

Manual-only and recommended blocks round-trip, old v1/v2 load, new format guard,
Apply parity, no automatic recalculation, stale context preserving rates, source
settings persistence, tool selection/availability invariants, non-milling legacy
inheritance, and unchanged resolved output with equivalent rates. Real UI review
must cover the normal job/material/tool/suggestion/manual/stale workflow.
Generate wording is a scoped follow-up: generating toolpaths also produces the
selected dialect/program today; a later neutral label must preserve extension
and export warnings. No post changes are needed for this checkpoint.
