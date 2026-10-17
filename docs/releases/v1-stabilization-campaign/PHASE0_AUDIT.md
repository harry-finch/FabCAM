# V1 stabilization campaign — Phase 0 / Stage A

> Policy update 2026-10-16: [REL1-B1/B2](REL1_B_PLAN.md) supersedes Apple identity/notarization as a V1 requirement. Community clean-install/integrity gates remain required. Historical evidence below is unchanged.

2026-10-16. Audit and plan only; no production code or packaging configuration modified.
User requires a HARD STOP for important identity/migration or credential-dependent decisions.

## Baseline and state

- Repository HEAD `4ebd43394829f1fb8372e5f9152143b9dcf3c868` (dispatch documentation).
- Actual three-platform candidate source `5ac4023576d72d450912357887273195faab0a72`, version 0.9.0-beta.1; CI run 38863177268 succeeded on all three targets, including frontend, type, packaging and version checks.
- Existing uncommitted macOS smoke-test report/STATUS/DEVLOG preserved. See ../desktop-preparation/MACOS_RUNTIME_RESULTS.md and local dist-test/desktop-ci/38863177268/RESULTS.md.
- Native macOS launch, square preview/export golden parity, V-Carve text calculation, exact project save/reopen passed on existing profile. These are bounded tests, not clean-install or physical qualification.
- Today: generation-context (3), document-lifecycle (24), generate-palette-policy (4): **31/31 pass** via frontend vitest. No new behavior claimed tested.

## Existing generation architecture — KEEP / EXTEND

`frontend/src/lib/components/GenerateBar.svelte:run` validates geometry/document state,
captures documentEpoch + generationContextKey, builds request and dispatches to the existing client.
Paths: edited drawing, streaming per-operation, coarse stream, buffered fallback, buffered two-sided.
Native `crates/ivac-tauri/src/commands.rs` runs work off UI thread; streaming has token-specific
events/result/error/cancel channels and ready handshake. CPS is compiled in by default.
`api/tauri.ts` registers listeners before ready; WASM worker client/wasm.worker.ts already isolate computation;
HTTP streaming remains an existing adapter. No scheduler/process/worker replacement justified.

`state/generated.svelte.ts` owns transient pipeline state. GenerateBar guards callbacks by document epoch;
context key and exportGeneratedGcode independently reject stale output. Same-document request identity
is not a first-class guard in run(): introduce a runtime-only run sequence, and prevent concurrent starts.
Every event, completion, error and finally cleanup must own both epoch and sequence. Keep context checks.

Confirmed progress defect: op_completed sets both opFraction=1 and increments opIdx;
GenerateProgress computes (opIdx+opFraction)/opTotal and opIdx+1 in the label. One completed op
therefore produces 200% / 2 of 1. This also appeared in native smoke observation. Operation count
is not a reliable percentage of total runtime; postprocessing/preview are not represented by that ratio.
Existing completion timeout is not request-owned; cover it when replacing transient presentation.

### UX7 plan — ivac-upkvw.1

EXTEND only: runtime request-scoped lifecycle + performance.now(), 250 ms display cadence.
Start immediately after valid preflight acceptance, before request preparation. Stop after accepted
result is transformed/stored and normal completion handoff is available, or on owned failure/cancel.
Label elapsed as generation duration, never machining estimate. Preview simulation/rendering is
asynchronous and must remain a distinct pending state; do not claim all preview validation completed
at the CAM response boundary. Inventory actual preview-ready signals during implementation.

Use indeterminate indicator; show actual operation name as context only when events provide it.
Preparing request / receiving result / preparing preview labels require real observed boundaries;
no invented kernel subphases or percentages. Retain terminal duration until next valid request/document.
Edits make completed status stale, and a result computed from edited inputs must not claim current success.
Cancel remains pending until acknowledgement; buffered paths must disclose unavailable cancellation.
Dispose ticker on terminal state, document switch and component teardown. No artificial delay.

Tests: fake monotonic clock; success/rejection/validation/transport/cancel; fast run; repeated/overlap;
late same-document response; document switch; changed inputs; teardown; two-sided/nonstreaming/native/WASM
adapter limits. Existing lifecycle/context/export and output suites must remain green.

## UX6-C presentation — ivac-upkvw.2

Shared tokens already in frontend/src/app.css: 4/8/12/16/24 px spacing, 28 px inspector controls,
graphite surfaces, blue accent. Use repository `.agents/skills/professional-workshop-ui/SKILL.md`
(found locally despite absent catalog), cam-ux-ui and existing UX6 General/Geometry/Mounting/Availability.

MachineDialog.svelte Program output still has generic arcs / optional-stop rows alongside CPS config;
built-in formatting already has a retained disclosure. Preserve draft values and handlers, use compact
rows and a selected-post summary. For ShopBot explicitly distinguish stored arc intent from linearized
OpenSBP output, keep unsupported combinations and manual spindle/toolchange requirements intelligible.
CpsPostConfig properties remain typed controls, compact units/help; never change post commands or capability.

ToolLibraryDialog advanced contains legacy ToolDetailBasics + ToolRowExpandedEditor.
ToolRowExpandedEditor forces Pass overrides open for section=legacy and repeats its heading.
Collapse advanced legacy groups, remove duplicate heading, align labels/value/unit, keep unknown values
and field callbacks. Preserve Review & apply, operation defaults vs inventory vs FS recommendations.
No schema, resolver, synchronization or persistence modification. UI acceptance requires real rendered
screenshots (running/completed/failed, Program output, Process & notes), keyboard/focus and narrow layout.
No updated screenshots exist at this audit-only checkpoint.

## REL1-A branding and compatibility decision — ivac-upkvw.3

Actual source: tauri.conf.json productName=ivaCAM; macOS bundleName=FabCAM;
Info.branding.plist CFBundleDisplayName=FabCAM; window/About and icon already FabCAM.
Binary ivac-desktop and Rust crates are internal. Actual .app and installer filenames remain ivaCAM.
CI artifact labels already FabCAM. Debian metadata is Package: iva-cam; actual candidate formats verified.
Preserve attribution, GPL, org.opcenter.ivacam, data paths, project kind and extensions.

IMPORTANT: local pinned Tauri CLI 2.12.3 config.schema.json productName documentation confirms it also
controls Linux kebab-case package name and resource directory, Windows install directory/Start menu/
HKCU product key and default MSI upgrade code. Therefore a global productName rename is not cosmetic.
No application-data migration is proposed (Tauri app_data_dir derives from stable identifier), but installer
identity/upgrade compatibility still needs an explicit policy. macOS may leave old ivaCAM.app beside FabCAM.app.

Recommended decision for approval: public names FabCAM, stable bundle/data/project identity, explicit
compatibility transition for prior ivaCAM test installations (old MSI upgrade GUID pinned where MSI is built;
Linux package replacement/conflict policy and Windows NSIS old-location detection validated before claiming
upgrade support; never delete user data). No new package is produced until this decision is accepted.
Do not silently rename every ivaCAM string. Check Linux desktop entry, Windows menus/uninstall metadata,
macOS names and app resource paths in resulting packages, not only JSON. Update BUILDING/icons docs/collector
fixtures only with the approved implementation. Prior candidates remain preserved.

## REL1-B root-cause evidence — reuse ivac-2p53v.4

Exact commands/results retained in codesign-verify.log, codesign-details.log, spctl.log, stapler.log.
`codesign --verify --deep --strict --verbose=4 /Applications/ivaCAM.app`: FAIL,
`code has no resources but signature indicates they must be present`.
`spctl --assess --type execute --verbose=2`: same FAIL.
Bundle inventory: Info.plist, Resources/icon.icns, MacOS/ivac-desktop only. No nested framework,
sidecar/backend executable or _CodeSignature/CodeResources. CPS is linked, not bundled as a separate tool.
otool dependencies are system libraries/frameworks. Codesign details: linker-signed ad hoc,
Info.plist not bound, no sealed resources/TeamIdentifier. This is an incompletely signed app bundle;
there is no evidence of a valid distribution signature later damaged, nested signing defect or entitlement rejection.
No explicit entitlements configured; do not invent entitlements as a fix. Hardened runtime not established
by current signature (flags adhoc/linker-signed, no runtime flag).

`security find-identity -v -p codesigning`: **0 valid identities found** locally; no secrets printed.
`xcrun stapler validate`: no ticket, exit 65. No submission was made: notarization status NOT TESTED,
not 'rejected'. CI candidates intentionally inject no signing secrets. Release workflow has optional APPLE_*
secret references; their existence/validity in GitHub has NOT been checked. TAURI_SIGNING_* updater signing
is distinct from macOS Developer ID signing. No Apple submission authorized.

Safe proposed development remediation after branding decision: explicitly sign the assembled final test
bundle ad hoc through Tauri, then strict-verify before DMG creation, with explicit 'development only'
manifest status. Establish on a copy/new build, preserving current candidate. This may fix structural signature
validity but CANNOT qualify Gatekeeper or notarization. Do not mutate installed baseline during diagnosis.

Distribution requires a Developer ID Application certificate/private key in a protected keychain, team
identity and approved notarization credential setup (App Store Connect API key or Apple ID/app-specific
password as supported). Configure secrets outside Git; never paste into chat. After explicit authorization:
final branded build -> sign -> strict verify -> notarize -> staple -> validate -> Gatekeeper -> clean-host
install. No credentials fabricated, no keychain changes or signing/notarization attempted in Phase 0.

Definitions: unsigned lacks a usable signature; ad hoc seals content without trusted developer identity;
Developer ID authenticates developer signing; notarization is Apple's submitted-artifact assessment;
staple attaches its ticket; Gatekeeper acceptance is a separate real distribution assessment.

Sources (checked 2026-10-16): https://v2.tauri.app/reference/config/ and
https://v2.tauri.app/distribute/sign/macos/ ; pinned local config schema is authoritative for installed CLI.

## Independent implementation order

A audit (this document) -> human package-transition decision -> B REL1-A minimal branding + tests ->
C UX7 lifecycle/timing -> D UX6-C presentation -> E local test builds/visual qualification and available
signature checks. REL1-B Developer ID/notarization stays blocked until credentials and explicit authorization.
No backend protocol or project-format migration is needed for the proposed UX work.

## Baseline platform matrix (not results for unimplemented changes)

| Area | macOS | Windows | Linux |
|---|---|---|---|
| CI build | PASS | PASS | PASS |
| Installation, existing profile | PASS | NOT TESTED | NOT TESTED |
| Clean installation/upgrade | NOT TESTED | NOT TESTED | NOT TESTED |
| Public application name everywhere | FAIL | FAIL | FAIL |
| Launch | PASS | NOT TESTED | NOT TESTED |
| Saved-project load/save/reopen | PASS | NOT TESTED | NOT TESTED |
| Raw SVG/DXF import matrix | NOT TESTED | NOT TESTED | NOT TESTED |
| Native generation, bounded fixtures | PASS | NOT TESTED | NOT TESTED |
| UX7 elapsed timing | NOT TESTED | NOT TESTED | NOT TESTED |
| Revised Program output UI | NOT TESTED | NOT TESTED | NOT TESTED |
| Revised Tool library UI | NOT TESTED | NOT TESTED | NOT TESTED |
| Machine output golden comparison in installed app | PASS | NOT TESTED | NOT TESTED |
| Full platform distribution qualification | FAIL | NOT TESTED | NOT TESTED |

Name FAIL evidence is baseline package filenames/metadata, not unperformed Windows/Linux screen inspection.
macOS: local build PASS; launch PASS; strict signature FAIL; Gatekeeper assessment FAIL;
notarization NOT TESTED; staple validation FAIL (absent ticket); clean-machine test NOT TESTED.
No new release candidate readiness. Physical validation remains separate.

## Human decision / stop

Approve public FabCAM package renaming with stable data identity and the explicit installer-transition
policy above, or require retaining old installer identities with customized packaging. Developer ID setup
is separate; continuing UX/development builds must not imply macOS distribution acceptance.

**HARD STOP — awaiting human review of the V1 stabilization changes and distribution readiness.**
