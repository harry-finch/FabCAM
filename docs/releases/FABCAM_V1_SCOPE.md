# FabCAM 1.0 — ShopBot Milling scope and acceptance

> Current macOS policy: [REL1-B1 community distribution](v1-stabilization-campaign/REL1_B_PLAN.md) is V1 REQUIRED; valid ad-hoc DMG, checksums and clean-host install/runtime are blocking gates. Developer ID/notarization/stapling are optional REL1-B2 POST-V1. Planning only; no publication authorization.

> Current 2026-10-15 checkpoint: [scope freeze and triage](V1_SCOPE_FREEZE_TRIAGE.md).
> User selected Windows 11 x64; latest Ubuntu is pinned to 26.10 x64. macOS reference
> remains arm64/26.5.2. GEOM1 explicitly not yet reviewed. Proposed initial packages:
> DMG, NSIS, DEB. This target refinement supersedes earlier unspecified-variant text;
> no platform runtime qualification or implementation approval is implied.

2026-10-15 · **ROADMAP APPROVED — macOS, Windows and Linux required for V1.**
User amendment recorded 2026-10-15; exact target variants and release qualification remain pending.
[Release strategy, classifications and decisions](RELEASE_ROADMAP.md).
This is an acceptance plan, not certification or permission to machine.

## Minimal functional scope

New/open project → SVG/DXF import → inspect/repair → stock and work origin →
machine and physical tool selection → supported operation → saved cutting values
→ generate → resolved preview and warnings → OpenSBP export. Physical setup,
zero establishment and execution belong to an authorized operator/controller.

Single-sided metric milling, supported flat cutters and the separately bounded
V-bit workflow. Existing whole-file transforms suffice; no complete CAD editor,
nesting, two-sided setup, generalized V-carving or FabSBP dependency.

## Supported-operation proposal

“Include” means proposed release scope, not current physical qualification. The
exact combinations below limit claims; existing unsupported settings must remain
visible and fail before export. No blanket Engrave/V-bit post authorization.

| Operation | V1 disposition and bounded software scope | Qualification / remaining boundary |
| --- | --- | --- |
| Profile | Required: inside/outside/on-line compensation, passes, ordinary closed contours; supported climb/conventional direction | [M1](../shopbot-modernization/M1_QUALIFICATION.md), [UX1](../shopbot-modernization/ux/UX1_QUALIFICATION.md), [CAM3](../shopbot-modernization/CAM3_QUALIFICATION.md). Open contours only in qualified on-line cases; no arbitrary tab/lead topology claim. |
| Pocket | Required: ordinary bounded regions, qualified concavity/island/stepover/allowance cases | M1/UX1. Deep nested Auto and XOR remain suspect; triage before declaring these combinations supported. No all-algorithm/all-topology blanket claim. |
| Drill / peck | Required: positioning, depth schedule, peck/retract semantics of existing qualified cycle expansion | M1 and [M2](../shopbot-modernization/M2_QUALIFICATION.md); physical positioning/depth trial required. No unqualified advanced controller cycles. |
| Tabs / leads | Required limited ordinary Profile use: Direct entry, square/circle matrix, Rectangle/Ramp tabs, Off/Straight/Arc leads, both directions | CAM3 48-combination software matrix; curved rectangular tabs PARTIAL (excess arc stock), no universal holding-strength claim. Ordinary straight-span tab workflow prioritized physically. |
| Entries | Required existing Direct and explicitly supported Ramp cases | UX1/CAM3; Ramp with tabs/leads, Helix and short-entry fallback remain refused where unsupported. Never silently substitute strategy. |
| Spoilboard surfacing (UX3) | Conditional included/limited: explicit rectangular X/Y raster, flat cutter, declared margins/overtravel, safe retract between rows | [UX3](../shopbot-modernization/UX3_QUALIFICATION.md): coverage/software parity; no physical flatness/clearance qualification. Not general surface machining. |
| Dogbones / T-bones (CAD1) | Conditional included/limited: explicit compatible corners, cutter-sized geometric edit, preview/history | [CAD1](../shopbot-modernization/CAD1_QUALIFICATION.md); tool/side/source guards retained. GEOM1+relief and authored-marker conflicts remain refused. No post-specific geometric patches. |
| V-Carve Perimeter | Conditional included/limited: saved Perimeter remains contour groove at capped depth, progressive passes | [A1–A4](../shopbot-modernization/VCARVE_QUALIFICATION.md). Not integral clearing; unsupported island corner cases refuse. |
| V-Carve Full medial axis | Conditional included/limited: sampled variable-depth interior spines, continuous chord clearance and completion proof | Same report: sampled coverage 256/800 cells on reference rectangle, not an area bound; missing branches/complete arbitrary-region clearing unqualified. Overcut/undercut refuse. Text workflow needs reopen triage. |
| Halfpipe / inherited advanced operations | Retain existing behavior but outside proposed ShopBot supported matrix | No new post enablement; preserve non-regression tests. Paired inlays, rest machining and two-sided deferred. |

V-Carve keeps its documented numerical budget, caps and sampling limits. Generic
Engrave, arbitrary V-bits on milling and V-Carve arcs remain rejected by the bundled
post. A successful export/preview never establishes physical clearance or complete
removal. If an optional operation is advertised as workshop-qualified, its selected
physical trial becomes a release gate; otherwise human approval must explicitly
accept its limited/software-only designation.

## V1 acceptance matrix

Evidence links are existing records, not new test results. Physical column refers
to the current integrated release scope; historical M4 does not certify later changes.
Blocker “Yes” means release evidence missing or unresolved in a required workflow.

| Workflow/capability | Current status | V1 requirement | Evidence | Missing work | Release blocker |
| --- | --- | --- | --- | --- | --- |
| Import SVG/DXF | Bounded software tests | Required | [M1](../shopbot-modernization/M1_QUALIFICATION.md), [GEOM1 audit](../shopbot-modernization/GEOM1_AUDIT.md) | RC real-file units, layer/curve/point cases, invalid input messaging | Yes: integrated acceptance |
| Geometry preparation | GEOM1 implemented/software-qualified; human gate open | Required bounded subset | [GEOM1 results](../shopbot-modernization/GEOM1_RESULTS.md) | Human UX, integrated edited-project workflow; no broad healing expansion | Yes: ivac-e2jnb |
| Stock / work origin / fixtures | Single-sided XY software-qualified | Required | [UX2](../shopbot-modernization/ux/UX2_QUALIFICATION.md), [ADR-0008](../shopbot-modernization/adr/ADR-0008-work-origin-frames.md) | Actual setup correspondence, RC stale/critical checks | Yes |
| Machine profile | UX6-A accepted; real profile has declared unknowns | Required | [UX6-A](../shopbot-modernization/ux/UX6_A_RESULTS.md), [M3](../shopbot-modernization/M3_QUALIFICATION.md) | Confirm actual machine/controller limits and spindle assumptions | Yes for workshop |
| Physical tools / assignment | UX6-B accepted; identity defect ticket still open | Required | [UX6-B](../shopbot-modernization/ux/UX6_B_RESULTS.md) | Reproduce same-ID shortcut, verify saved copies and changes | Yes until identity disposition |
| Operation creation / templates | UX4/4.1/UX5 accepted | Required creation; templates retained optional | [UX4.1](../shopbot-modernization/UX4_1_QUALIFICATION.md), [UX5](../shopbot-modernization/UX5_QUALIFICATION.md) | Native keyboard/labels/focus, correct source/tool rebinding | Yes if essential flow fails |
| Profile | Bounded software + historical M4 physical square | Required limited matrix | M1/UX1/CAM3/M4 | Current candidate physical dimensions/depth and direction | Yes |
| Pocket | Software bounded; nested-region risk open; no current physical trial | Required limited matrix | M1/UX1; ivac-dli7, ivac-j2po | Resolve/exclude affected combinations; island/depth trial | Yes |
| Drill / peck | Software-qualified; current physical trial absent | Required | M1/M2 | Position/depth/retract trial | Yes |
| Surfacing | Software-qualified, continuation accepted | Conditional included/limited | [UX3](../shopbot-modernization/UX3_QUALIFICATION.md) | Flat cutter/overtravel/coverage physical case if workshop claim | Conditional |
| Tabs / leads / entries | Bounded software matrix, curved tab limits | Required ordinary Profile subset | CAM3/UX1 | Holding material, lead clearance, supported entry trials | Yes for subset; not all combinations |
| Dogbones | Software-qualified, continuation accepted | Conditional included/limited | CAD1 | Fit/geometry physical sample if workshop claim | Conditional |
| V-Carve | Bounded software/UX accepted, full-area coverage partial, physical absent | Conditional included/limited | [V-Carve](../shopbot-modernization/VCARVE_QUALIFICATION.md), [final UX](../shopbot-modernization/VCARVE_FINAL_UX.md) | Text-only reopen; selected groove/spine trial if workshop claim | Conditional; broad V-carving not a blocker |
| Feeds & Speeds | Implemented through FS3.9.1; independent reviews open; FS4 deferred | Required bounded provenance/manual workflow | [FS3.9](../shopbot-modernization/feeds-speeds/FS3_9_QUALIFICATION.md), [FS3.8](../shopbot-modernization/feeds-speeds/FS3_8_QUALIFICATION.md) | Policy/UX/licensing approval and selected physical settings | Yes for claims used in release |
| Preview | Final ShopBot execution path authoritative; material view approximate | Required | [ADR-0007](../shopbot-modernization/adr/ADR-0007-shopbot-resolved-preview.md), V-Carve A4 | RC independent decoded parity incl edited geometry and origins | Yes |
| Warnings / readiness | Critical/stale guards implemented | Required | UX1/UX2/GEOM1 | Invalid configuration, collision, source/tool edit, canceled generation export tests | Yes |
| ShopBot output | Bounded CPS post + independent decoder; M4 historical physical | Required | [M2](../shopbot-modernization/M2_QUALIFICATION.md), [GEOM1](../shopbot-modernization/GEOM1_RESULTS.md) | Candidate command/rate/tool-change/work-origin review and hardware trials | Yes |
| Persistence | v1–v7 tests; stable bindings/manual values; native RC unqualified | Required | [GEOM1 serialization](../shopbot-modernization/GEOM1_SERIALIZATION.md), UX6/FS3.7 | Candidate clean/upgrade save/reopen, private data and tool/profile restoration | Yes |
| Performance | PERF1 B1/B2 accepted; cancellation target missed | Required representative usability | [A](../shopbot-modernization/PERF1_A_RESULTS.md), [B1](../shopbot-modernization/PERF1_B_RESULTS.md), [B2](../shopbot-modernization/PERF1_B2_RESULTS.md) | RC cold/warm/memory measurement; edited route separately | Yes for representative freeze/failure; 200 ms target conditional |
| Packaging | Historical macOS app built/launched; current integrated package untested | Required macOS, Windows and Linux | [branding](../shopbot-modernization/ux/A1_BRANDING_INTEGRATION.md), [build](../BUILDING.md) | Clean build/install/upgrade/runtime on all three platforms | Yes on each |
| Documentation | Rich engineering records; release guide incomplete | Required | [README](../../README.md), existing qualification docs | Practical verified tutorial/support matrix/install/license notices | Yes |
| Physical validation | Only bounded M4 r2 historical operator evidence | Required current campaign | [M4](../shopbot-modernization/M4_VALIDATION.md) | V1 Workshop Validation below; actual measurements | Yes |
| Human acceptance | Several independent gates open; release scope new | Required | [gate register](RELEASE_ROADMAP.md#existing-human-gates) | Relevant reviews, RC go/no-go | Yes |

## Required-work triage

Tracker priority is not final release severity. These are audit findings, not fixes
or proof that old tickets still reproduce after intervening work.

| Issue | Release impact / required disposition |
| --- | --- |
| ivac-hs9i same-ID tool shortcut | Potential wrong physical tool editing/identity. Reproduce first; supported assignment cannot silently target another copy. Block until disproved/fixed. |
| ivac-dli7 deep nested Pocket Auto | Potential preview/emitted-region mismatch. Block this scope until independent geometric oracle confirms it or an explicit effective refusal excludes it. Documentation alone cannot cure silently wrong output. |
| ivac-j2po XOR early empty result | Known three-contour parity limitation. Same rule; simpler combinations do not qualify arbitrary XOR. |
| ivac-2zll rough/finish RPM change | Existing fail-closed spindleSpeed rejection; workflow defect, not proven unsafe output. Preserve separate saved values; qualify support or clearly refuse/exclude mixed RPM. Do not overwrite values to pass. |
| ivac-vt6e text-only reopen | Reported empty 2D/misleading warning despite generated output. Reproduce on current desktop; blocks advertised text engraving if user cannot inspect sources. |
| ivac-5g51 partial WASM freshness | Candidate package must prove complete matching WASM assets; repair guard if it can ship mismatched artifacts. Packaging integrity gate. |
| ivac-57kp curved tab spans, ivac-vcp7 residual extensions | Explicit bounded limits; deferrable unless needed by chosen matrix. |
| ivac-u1ko Z origin, ivac-jtkj two-sided, ivac-13go old test expectation | Nonzero numeric Z/two-sided remain unsupported. Preserve refusal; reconcile stale test ticket against cancellation report before changing anything. |
| ivac-nh6q / ivac-q1nz1 cancellation | Review measured latency; no successful/partial output on cancel mandatory. Faster cancellation conditional. GEOM1 buffered route separately unqualified for latency. |
| ivac-htc8 cache test race | Serial historical regression policy known; establish repeatable release evidence, not a machining defect by itself. |
| ivac-snqn productName migration | Preserve stable identity unless separately qualified migration necessary. Branding rename not required to make release useful. |

Other low-priority LOD/cache auditing/source isolation/UI refinements are TECHNICAL
DEBT / DEFECT, assessed if reproduced on the release matrix. Android, tracing,
generators and extra tool families are post-V1.5 enhancements, not V1 blockers.

## Coordinates, output and persistence gates

Stock dimensions/position, XY work origin and Z=0 stock-top convention must be clear.
Retain rejection of nonzero numeric Z, unsupported WCS and two-sided cases; positive
safe Z and negative cut depth remain explicit. Declared keep-outs/holder geometry
support warnings, not automatic avoidance or certification of undeclared clamps.
Source/config/tool/fixture edits must invalidate output; canceled/failed generation
must not become exportable. Physical zero is never established by silent output commands.

Review exact bundled post capability restrictions: metric movement/unit conversion,
spindle/manual-start policy, safe travel/retract, saved feed versus plunge, command
rejection, coordinate precision, manual tool-change assumptions and final parking.
Compare independently decoded exported output against generated/resolved preview,
including operation/source provenance and work-frame translation. No unsupported
strategy substitution. Test critical-warning export blocking and freshness after
save/reopen, history, machine/tool changes and generation races.

Preserve original imports, stable IDs/lineage and unresolved binding refusal under
ADR-0021. GEOM1 already supports bounded line cleanup/joins/duplicates/zero-length
LINEs/simplification with preview/history; POINTs and curves are preserved with
conservative repair limits. Source/transform rebase and relief/modifier combinations
are not additional V1 requirements. Test v1–v7 compatibility and old-backend refusal;
no serialization change in this roadmap.

## Performance acceptance proposal

Use PERF1 saved request hashes and commands on a named release machine, native CPS
and actual desktop transport. Record first process call separately from warm samples;
never compare a cache hit to cold generation as a universal speedup. Source geometry,
tolerances, output and refusal outcomes must remain equal.

Recorded B1 Test repeat: native 832→15 ms, HTTP first frame 899→141 ms; relief
2259→1516 ms. B2 repeated Pocket analysis 337.845→0.222 ms and total
338.370→0.625 ms; small actual UI case had no material latency benefit. These are
fixture/hardware-specific. A's simulation replay Test was 2310–4051 ms including
debounce. [Cancellation](../shopbot-modernization/PERF1_CPS_CANCELLATION_RESULTS.md)
p95 was 216/218 ms native/HTTP, missing the proposed 200 ms threshold.

Proposed RC targets, requiring human approval and fresh measurement:

- Same nine PERF1 fixtures/posts plus ordinary SVG/DXF import and a saved GEOM1
  edited job; no crashes, incorrect result, stale export or retained partial success.
- On the same reference hardware, cold and warm stage medians separately no worse
  than 20% over their corresponding accepted B1/B2 measurements; investigate larger
  changes before acceptance. Preserve raw runs, counts, binary and input hashes.
- Test warm HTTP first frame ≤300 ms, repeat native Test ≤50 ms, representative
  Test simulation replay ≤5 s on that reference configuration. These margins derive
  from observed values; not guarantees for arbitrary text or machines. Cold limits
  use paired baselines, not these warm-cache thresholds.
- Record peak RSS and 20 repeated-generation cycles; cache retention must respect
  its existing limits, with no sustained unexplained growth or OOM on representative
  loads. Whole-process memory ceilings/large import latency remain to be measured;
  do not invent a qualified memory maximum from cache size alone.
- Keep UI progress/cancel usable; cancellation must discard stale results. Faster
  than the measured cooperative boundary is conditional unless the essential flow
  freezes. Edited drawing cancellation is buffered and has no inherited PERF latency proof.

## V1 — ShopBot Workshop Validation

**Required release gate, NOT PERFORMED for current release scope.** Authorized human
operator owns machine action; this task executes nothing. M4 evidence remains valid
only for its exact historical program and reported scope.

Minimum proposed campaign uses one actual machine/controller configuration, one
measured flat cutter, one identified sacrificial material/stock and operator-approved
workholding/settings. Select a suitable drill and a measured V-bit only if their
respective trials are included; never invent feeds or machine limits.

1. Record candidate/package and program hashes, machine/model/controller/software,
   profile, physical tool geometry/stickout, stock/material and fixture setup. Verify
   XY origin, stock-top Z, safe travel, spindle/manual-start and tool-change procedure.
2. Review exported programs and controller preview; perform appropriate supervised
   non-cutting/air trials before cutting. Agree XY/depth/clearance acceptance tolerances
   and measurement instruments before the trial, based on the real machine/use case.
3. Sacrificial Profile coupon: outside/inside dimension, final depth, selected
   climb/conventional cases, tabs and supported direct/lead/entry transitions.
4. Pocket coupon: ordinary region with island, floor depth, preserved island,
   engagement/entry and rough/finish values within supported post semantics.
5. Drill/peck coupon: hole positioning, depth and observed peck/retract behavior.
6. If workshop-supported: bounded surfacing area/edge clearance; CAD1 mating-corner
   sample; V-Carve groove and spine text samples. No blanket flatness/fit/coverage claim.
7. Verify export safeguards in software using invalid/stale/critical cases, never by
   deliberately executing unsafe programs. Record actual dimensions/depths, observations,
   repeat trial after corrections, discrepancies and final human acceptance per row.

FS4 minimum shares these coupons: record actual machine/tool/material/engagement,
resolved rates and observations for a conservative Profile and Pocket setting, plus
settings of any additional included trials. Scope each local Tested preset to that
context; repeatability and the existing validation policy are needed for Validated.
No automatic promotion of reference data or synthetic test attestations. Exhaustive
material/tool trials stay deferred; unverified recommendations remain identified.

## Packaging and documentation

Human amendment, 2026-10-15: **all three desktop platforms are release requirements**.
A failed supported Windows or Linux installation blocks V1 just as a failed macOS
installation does. Exact minimum OS versions, architectures, Linux distributions and
package formats remain to be fixed; no promise of every variant is inferred.
On each target, verify native CPS generation, SVG/DXF import, dialogs/fonts/paths,
preview, warnings/export guards, saved projects, tools/profiles/private data and
upgrade compatibility. Use the same reference fixtures and compare decoded output;
platform builds alone do not prove identical behavior. Packaging tests do not replace
the separate human ShopBot workshop campaign.

| Target | Existing evidence | Proposed V1 policy / remaining gate |
| --- | --- | --- |
| macOS Apple Silicon | Historical FabCAM .app build, launch/Finder and old project/settings access; latest GEOM1 Tauri compilation only | Required macOS target; Apple Silicon is the initial test candidate. Pin actual OS/architecture and verify current clean install, launch, dialogs, fonts, native CPS, persistence, reopen/export and upgrade. No current all-macOS claim. |
| macOS Intel/universal | Universal target configured in desktop workflow | Build/installation qualification missing for candidate; not promised unless selected. |
| Windows | Native/cross-build guidance and inherited build claims in BUILDING_WINDOWS; no current FabCAM installer/runtime evidence | Required for V1. Pin Windows version/architecture, build native package and verify actual install/upgrade/runtime and ShopBot workflow. Signing distribution policy must be satisfied. |
| Linux | deb/rpm/AppImage targets and build recipes | Required for V1. Pin supported distribution/version/architecture and package format; verify actual install/upgrade/runtime, WebView dependencies and ShopBot workflow. Build configuration alone is insufficient. |
| Browser / Android | Browser software reviews; Android inherited build-only evidence | Development/limited usage preserved; not substitute for desktop ShopBot CPS qualification. Current standalone WASM lacks CPS. Android not V1 platform gate. |

Audit sources: [build guide](../BUILDING.md), [Windows guide](../BUILDING_WINDOWS.md),
[signing](../SIGNING.md), [Tauri config](../../crates/ivac-tauri/tauri.conf.json),
[desktop workflow](../../.github/workflows/release-desktop.yml),
[identity ADR](../shopbot-modernization/adr/ADR-0012-fabcam-product-identity.md).
Current development version is **0.9.0-beta.2** ([version decision](VERSIONING.md)).
Approved REL1-A uses productName FabCAM and preserves identifier org.opcenter.ivacam,
internal names, application data, formats, attribution and existing icons. The version change does not migrate installation or
persisted identities, qualify installers, or declare the 1.0 horizon achieved.

Reproduce build from a recorded source snapshot/lockfiles/toolchain, confirm native
CPS enabled and full matching WASM bundle, record artifact hashes and prerequisites.
The workflow's floating tooling/signing/runner assumptions need an actual successful
candidate run; YAML alone proves no distributable. Apply REL1-B1 community macOS gates (Apple notarization is not required for V1)
and retain the existing Windows signing policy. Do not trigger tag workflows now.

Verify user-data directories actually resolved by Tauri app_data_dir, private FS packs,
workspace/profile/tool storage and browser IndexedDB separately. Test upgrade over
historical install, settings/recents, library/profile imports/exports, save/reopen and
backup/restore with unknown fields/manual values intact. No destructive migration or
installer rename without a qualified compatibility plan. Review dependency/icon/font
licenses, GPL-3.0-or-later and upstream ivaCAM attribution; never package private
manufacturer or Sorbonne packs as public release data.

Essential user documentation, built from existing technical records: install/upgrade;
new/open/import quick start; ShopBot machine/profile and physical-tool setup; saved
cutting values and FS uncertainty/private data; operation scope/configuration; stock,
XY work origin and external Z setup; preview versus physical proof; warning/error and
export readiness; supported/unsupported combinations, known defects, licensing and
upstream attribution. Verify tutorial on the candidate package. Keep V1.5/2.0 plans
clearly future. No cosmetic UX reopening: retain Professional Workshop UI, labeled
inputs, keyboard focus and explicit warning versus blocking state.

## Release acceptance checklist

| Gate | Evidence required to accept | Current release state |
| --- | --- | --- |
| Operation matrix and end-to-end workflow | Qualified scoped imports, geometry, operations, decoded output/preview and refusals | Pending integrated candidate |
| Coordinates and safeguards | Correct origins/depths, no unsafe supported output, critical/stale export blocked | Pending integrated candidate |
| Persistence | Project/tool/profile/FS manual values and v1–v7 compatibility | Pending native candidate |
| Desktop install and UX | macOS, Windows and Linux clean/upgrade tests and essential task completion | Pending |
| Physical workshop | Actual operator trials and measurements for advertised physical scope | Pending |
| Critical defects | Fixed with evidence or effectively refused/excluded, no silent mismatch | Triage pending |
| Documentation/limitations/licenses | Accurate user-facing guide and distribution contents | Pending |
| Human approval | Roadmap approved; relevant independent gates and RC go/no-go still required | Roadmap accepted; release approval pending |

No release ready claim until every required gate passes. Optional scope reduction
requires explicit human approval, never silent relabeling of a defect as unsupported.
