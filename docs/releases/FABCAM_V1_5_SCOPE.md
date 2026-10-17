# FabCAM 1.5 — ProtoMAX Waterjet scope and acceptance

2026-10-15 · **ROADMAP APPROVED — NOT IMPLEMENTED / NOT QUALIFIED.**
Approval recorded 2026-10-15; V1 requires macOS, Windows and Linux. Separate
waterjet contract, implementation and physical gates remain required.
[Release proposal](RELEASE_ROADMAP.md). No machine control or output format is
implemented or verified by this documentation checkpoint.

## Bounded first workflow

SVG/DXF import → shared GEOM1 inspection/cleanup → waterjet job/material/thickness
context → contour selection/manual positioning → supported kerf/entry intent →
preview cuts and non-cut transitions → capability validation → confirmed compatible
output → actual target-software import/transfer → supervised physical validation.

No nesting optimizer, multi-sheet inventory, common-line cutting, additional machine
families or milling-style dummy cutter required. Existing manual file placement may
suffice; establish any actual missing placement requirement during feasibility.

## ProtoMAX Feasibility & Output Contract

A proposed early documentation/research gate in the existing WJ track, independently
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
  operator sets. Establish whether geometry-only handoff or process-aware output is
  actually supported. Geometry import alone is not proof of a CAM-output contract.
- Confirm required proprietary-software dependencies and supported deployment OS;
  distinguish file generation, software import and actual controller acceptance.
- Inspect reusable source geometry, operation ordering/offsets and process boundaries;
  record the smallest additive model/ADR and independent verification approach.

**Current blocker:** no verified ProtoMAX output contract or accepted sample in this
repository assessment. “OMX” is a research candidate, not a promised supported format.
Do not assume milling G-code/OpenSBP compatibility or invent an API. If documentation
or lawful samples cannot establish the contract, stop implementation and request an
operator-guided software export/import demonstration and vendor documentation.

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

## V1.5 acceptance matrix

| Workflow/capability | Current status | V1.5 requirement | Evidence / missing work | Release blocker |
| --- | --- | --- | --- | --- |
| Output/transfer contract | Unknown; no qualified exporter | Required first | Actual machine/software, legal format references, accepted sample/import | Yes |
| Process model | Assessment only | Required bounded explicit intent | Additive ADR, parameter ownership, compatibility/capability refusals | Yes |
| Shared geometry cleanup | GEOM1 implemented/software review | Required reuse | [GEOM1](../shopbot-modernization/GEOM1_RESULTS.md); human acceptance, waterjet contour/duplicate/closure integration | Yes for reliable sources |
| Manual positioning | Existing file transforms, no new nesting model | Required usable placement; optimizer excluded | Verify enough for target job; qualify any minimal missing positioning separately | Conditional development |
| Kerf / contours / islands | Generic offsets available, waterjet unqualified | Required supported cases | Independent dimension/side/closure/order tests; no double compensation | Yes |
| Pierce / lead / quality | No verified target semantics | Required only as owned by contract | Verify responsibility and supported parameter transitions; reject unknowns | Yes |
| Preview / diagnostics | Generic display reusable; milling removal not WJ proof | Required cut/traverse clarity | Process-specific preview and actionable failures; persisted intent tests | Yes |
| Output verification | No independent waterjet decoder | Required truthful verification boundary | Decoder where feasible; otherwise documented alternate checks below | Yes |
| Persistence / compatibility | Shared foundations only | Required | Waterjet saved intent/schema tests; no silent milling reinterpretation | Yes |
| Software qualification | None for waterjet | Required | Simple, island, open/slit if supported, acute, duplicate, units, invalid cases; output correspondence | Yes |
| Target-software acceptance | Unverified | Required | Actual versioned import/preview workflow evidence | Yes |
| Physical validation | Not performed | Required | Authorized operator campaign below | Yes |
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
