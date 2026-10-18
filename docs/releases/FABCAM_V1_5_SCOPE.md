# FabCAM 1.5 — Native ProtoMAX OMX scope and acceptance

2026-10-15 · **ROADMAP APPROVED — NOT IMPLEMENTED / NOT QUALIFIED.**
Approval recorded 2026-10-15; V1 requires macOS, Windows and Linux. Separate
waterjet contract, implementation and physical gates remain required.
[Release proposal](RELEASE_ROADMAP.md). No machine control or output format is
implemented or verified by this documentation checkpoint.

## Product decision — native OMX, no LAYOUT dependency (2026-10-18)

**Required V1.5 objective, not verified capability:** FabCAM replaces OMAX LAYOUT
for normal waterjet preparation and directly exports ProtoMAX-compatible `.omx`
for actual installed OMAX MAKE. MAKE retains execution/controller responsibilities.
LAYOUT may supply offline research references, but is not needed for conversion,
quality, leads, sequence, cutting intent, bridges or final program preparation.
DXF-for-LAYOUT does not satisfy this release. If direct OMX proves infeasible,
record a V1.5 blocker and obtain a human product decision; never silently downgrade.
User will provide real OMX samples; none inspected/generated in this planning task.

## Bounded first workflow

SVG/DXF import → shared GEOM1 inspection/cleanup → waterjet job/material/thickness
context → contour selection/manual positioning → supported kerf/entry intent →
preview cuts and non-cut transitions → capability validation → native OMX export
→ actual installed OMAX MAKE → authorized ProtoMAX physical validation.

No nesting optimizer, multi-sheet inventory, common-line cutting, additional machine
families or milling-style dummy cutter required. Existing manual file placement may
suffice; establish any actual missing placement requirement during feasibility.

## ARCH-MACH architectural preparation

[ARCH-MACH](ARCH_MACH_PLAN.md) plans geometry/process/capability/adapter/output-validator/
qualification boundaries and a repeatable [integration guide](../development/ADDING_A_MACHINE.md).
Initial contract/guide precedes production OMX writer architecture, informed iteratively
by OMX1 evidence. Reuse current seams, add only proven missing waterjet semantics;
no elaborate generic framework may delay ProtoMAX. First practical validation requires
real WJ/MAKE/physical qualification, not documentary completion. Not a V1 release gate.

## OMX1 — Native Format Research & Output Contract

Refinement of the existing early feasibility gate, **V1.5 CRITICAL**, independently
authorized and time-bounded by its deliverable, not a background implementation.
It can run while V1 stabilizes if it does not displace ShopBot critical work.

Required evidence before implementation:

- Actual machine model/configuration and installed OMAX/ProtoMAX software versions,
  supported workflow and available operator access; do not assume names or versions.
- Authoritative, legally usable manuals/specifications describing accepted files and
  import/export. Record source/version/license and allowed redistribution of samples.
- One operator-supplied known-good simple contour file and its accepted target-software
  setup/preview; compare units, dimensions, contour order, process parameters and
  transfer steps. Request evidence without executing machine actions.
- Parameter ownership: what FabCAM exports, what target software resolves, what the
  operator sets. Establish how native OMX carries the required process intent and what MAKE
  resolves. Geometry-only DXF/LAYOUT handoff is not the approved product contract.
- Confirm required proprietary-software dependencies and supported deployment OS;
  distinguish file generation, software import and actual controller acceptance.
- Inspect reusable source geometry, operation ordering/offsets and process boundaries;
  record the smallest additive model/ADR and independent verification approach.

**Current blocker:** no verified ProtoMAX output contract or accepted sample in this
repository assessment. Native OMX is now the required release target, but structure/semantics remain
unverified; the requirement is not a claim of an implemented compatible writer.
Do not assume milling G-code/OpenSBP compatibility or invent an API. If documentation
or lawful samples cannot establish the contract, stop implementation and request an
operator-guided software export/import demonstration and vendor documentation.

[OMX1 detailed lawful reference-file plan](OMX1_PLAN.md) defines controlled differences,
versions/provenance/rights, facts versus hypotheses and structural/semantic/MAKE gates.
No text/XML/JSON/G-code assumption or invented field. HARD STOP before production
writer architecture if the contract is insufficient.

Deliverable: a short evidence dossier, ownership table, supported/unsupported matrix,
one proposed architecture decision and go/no-go. No comprehensive industrial-waterjet
framework. Implementation requires separate approval of that contract.

## Reuse and process ownership proposal

[Existing architecture assessment](../shopbot-modernization/FUTURE_PROCESSES.md)
finds Mill/Laser/Drag/Plasma modes but no qualified waterjet process. Common contour,
transform, ordering and lead helpers can be reused; milling Z/tool/spindle fields and
CPS spindle/pierce conventions are not waterjet intent. The current IR has no complete
waterjet pierce/quality contract. Keep geometry, process planning, output and verification
separate; never inject kerf/pierce rules into GEOM1.

| Concept | Proposed responsibility, subject to verified target contract |
| --- | --- |
| Original/edited contours, IDs, closure, selection and transforms | FabCAM shared geometry/GEOM1; retain open paths if explicitly supported, reject ambiguous bindings |
| Inside/outside/on-line and kerf | Explicit waterjet intent; decide whether FabCAM offsets or target software compensates, never both |
| Pierce, entry/exit and jet-off traverse | Explicit process semantics; target-software ownership must be confirmed; no invented times/heights |
| Material/thickness and quality | Waterjet-specific job/process values, provenance and units; target documentation decides which are exported versus resolved there |
| Traverse/cut settings | Use verified machine/process constraints or target-software resolution; no milling FS recommendations or invented cutting values |
| Cut order and retained islands | FabCAM planning if contract supports it; verify transferred order and target-software modifications |
| Fixtures/no-cut regions | Explicit geometric intent and known constraints; no universal collision assurance |
| Pressure/calibration/machine zero/physical setup | Operator and target software/controller; outside initial FabCAM execution scope |
| Etch, common-line, multi-sheet and advanced optimization | Outside first V1.5 support unless separately justified and approved |

Keep process-specific inspectors in the existing compact Professional Workshop UI.
Show material/thickness, cut intent/quality/entry only where supported. Hide irrelevant
milling RPM/plunge/cutter controls without hiding saved incompatible settings/errors.
No general multi-process dashboard. Milling FS remains independent.

## Bounded waterjet implementation milestones

All **PLANNED — NOT IMPLEMENTED / NOT QUALIFIED**; audit existing WJ labels retained.

| Milestone | Required scope / acceptance |
| --- | --- |
| WJ1 — Native Waterjet Operation Model | Explicit Waterjet Profile, material/thickness, quality Q1–Q5 only where verified, traverse/cut, kerf/entry/pierce intent and machine limits. Assign project/operation/profile/OMX/MAKE ownership; quality may start per operation but accommodate verified per-segment semantics. No milling RPM/Z/ToolEntry/FS emulation or duplicate MAKE calculations |
| WJ2 — Pierce, Leads, Kerf and Collision Qualification | Independent lead/pierce/quality/kerf matrix against neighboring contours, islands/narrow/protected regions and invalid inputs. Dynamic/static pierce responsibility from OMX/MAKE contract; no invented universal lengths/times/material rules or double kerf. Reject or explicitly delegate only under verified contract |
| WJ3 — Waterjet Holding Tabs and Cut Sequencing | Bounded uncut XY gaps, cut-off/traverse/re-pierce and bridge retention per verified OMX. Not milling Z-lift tabs. Inner-before-outer dependency analysis shares OP-ORDER foundations but accounts for holes/open cuts/slug detachment/held states/software constraints. Warnings or explicit ordering proposals; no silent authored-order changes |
| WJ4 — ProtoMAX Physical Qualification | V1.5 release gate: actual MAKE native-file acceptance, interpreted coordinates/geometry/quality/pierce/leads/traverse/order/kerf, dimensional trials, warning/refusal cases and ShopBot non-regression. Human-supervised physical evidence, not just sample resemblance |

OMX1 contract/architecture approval precedes production writer commitment. WJ1 model
and writer may be iteratively qualified with WJ2/WJ3, but all supported scope must pass
software/MAKE gates before authorized WJ4. Unsupported bridges/strategies remain explicit;
no mandatory LAYOUT preparation can substitute for missing FabCAM functionality.

## V1.5 acceptance matrix

| Workflow/capability | Current status | V1.5 requirement | Evidence / missing work | Release blocker |
| --- | --- | --- | --- | --- |
| ARCH-MACH extension contract | Planning/initial guide only | V1.5 architecture foundation/practical validation | Minimal seams approved before writer; guide exercised by actual ProtoMAX integration | Required V1.5 architecture/evidence, not V1 |
| OMX1 native OMX contract/export | Required product target; no qualified exporter/verified contract | Required first, no LAYOUT dependency | Real lawful OMX corpus, versioned MAKE acceptance, native semantic ownership | Yes; infeasibility needs product decision |
| WJ1 process model | Assessment only | Required bounded explicit waterjet intent | Additive ADR, parameter ownership, compatibility/capability refusals | Yes |
| Shared geometry cleanup | GEOM1 implemented/software review | Required reuse | [GEOM1](../shopbot-modernization/GEOM1_RESULTS.md); human acceptance, waterjet contour/duplicate/closure integration | Yes for reliable sources |
| Manual positioning | Existing file transforms, no new nesting model | Required usable placement; optimizer excluded | Verify enough for target job; qualify any minimal missing positioning separately | Conditional development |
| Kerf / contours / islands | Generic offsets available, waterjet unqualified | Required supported cases | Independent dimension/side/closure/order tests; no double compensation | Yes |
| WJ2 pierce / lead / quality | No verified target semantics | Required only as owned by contract | Verify responsibility and supported parameter transitions; reject unknowns | Yes |
| WJ3 bridges / cut order | Unqualified | Required bounded holding/sequence scope and OP-ORDER assessment | Native gap/cut/traverse/re-pierce semantics and independent detachment/intent references; no silent reordering | Yes for advertised scope |
| Preview / diagnostics | Generic display reusable; milling removal not WJ proof | Required cut/traverse clarity | Process-specific preview and actionable failures; persisted intent tests | Yes |
| Output verification | No independent waterjet decoder | Required truthful verification boundary | Decoder where feasible; otherwise documented alternate checks below | Yes |
| Persistence / compatibility | Shared foundations only | Required | Waterjet saved intent/schema tests; no silent milling reinterpretation | Yes |
| Software qualification | None for waterjet | Required | Simple, island, open/slit if supported, acute, duplicate, units, invalid cases; output correspondence | Yes |
| OMAX MAKE acceptance | Unverified | Required native OMX, without LAYOUT | Actual installed MAKE version/interpreted geometry/cutting-intent evidence | Yes |
| WJ4 physical validation | Not performed | Required | Authorized operator campaign below; native OMX and MAKE path | Yes |
| ShopBot non-regression | Existing V1 foundation | Required | Rerun V1 matrix on V1.5 candidate, incl persistence/output safeguards | Yes |
| UX/docs/install/human approval | Future | Required | Waterjet workflow guide/limits and any target-software/platform dependencies | Yes |

## Verification boundary and physical gate

Prefer an independent decoder of the actual confirmed output: compare units, contour
geometry, kerf ownership, cut/traverse, order, entry/pierce/quality intent and provenance
against generated preview. The ShopBot decoder cannot certify waterjet output.

If a complete decoder is infeasible, propose an explicitly weaker boundary for human
approval: validate exported geometry and every documented process field independently;
record actual target-software import, dimensions, order/settings and preview; retain
file hashes/screenshots and operator observations. State what proprietary processing
cannot be independently inspected. A generic path preview alone is insufficient.
If necessary semantics cannot be verified at all, that remains a release blocker;
the alternate boundary is not automatic permission to ship.

**V1.5 — ProtoMAX Workshop Validation** is separate from ShopBot/FS4. Authorized human
records candidate/output hashes, exact machine/software, material/thickness and actual
setup, operator-approved predeclared dimensional tolerances/instruments. Confirm file
acceptance and units before sacrificial trials. Measure dimensions/kerf, closed contours,
islands, supported piercing/leads/order and repeatability; verify warnings and unsupported
cases in software. Record anomalies/corrections and repeat affected tests. No invented
physical values or production readiness from software tests. Machine execution remains
outside this task and requires explicit human authorization and supervision.

## Delivery and exclusions

Early feasibility can overlap V1 stabilization; actual WJ implementation follows the
confirmed contract and approved V1 priority. Then process/geometry integration →
software/output checks → target-software evidence → physical trials → candidate
installation/docs + ShopBot non-regression → human V1.5 go/no-go → separately authorized
publication. No speculative date, no FabSBP dependency, no automatic nesting requirement,
no full GEOM2/3 dependency and no direct machine-control integration.

## Post-V1 sequencing — AUDIT3 planning addition

[AUDIT3 retrospective](AUDIT3_PLAN.md) is planned immediately after explicit acceptance
of validated V1, before the main V1.5 implementation campaign. It is not a V1 release
blocker or additional ProtoMAX functional capability. Frozen V1 reference/evidence,
a bounded time box and explicit gaps preserve V1.5 as the next product-development
priority; previously authorized early feasibility may still proceed. No current
scope/acceptance evidence or implementation authorization changes here.

## Post-audit consolidation dependency policy — 2026-10-18

[Consolidation backlog](POST_AUDIT_CONSOLIDATION.md) must not delay ProtoMAX by
requiring every proposed Rust refactor. After V1 acceptance/time-boxed AUDIT3,
prioritize verified output contract, explicit waterjet semantics, software/output tests
and separate physical qualification. GEOM-CORE/CAM-DIAG/TOOL-TYPES enter only if
specifically necessary or demonstrably low-risk and separately approved. PERF2 remains
V2+/measurement-driven, not a V1.5 prerequisite; optional TIME2 ShopBot calibration
may fit V1.5 only if practical, never a ProtoMAX or initial ShopBot qualification gate.
