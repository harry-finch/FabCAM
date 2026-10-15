# V1 Scope Freeze & Release-Blocker Triage

> Superseded handoff: the user approved this bounded lot on 2026-10-15.
> [Stabilization results](V1_STABILIZATION_RESULTS.md) record the subsequent fixes.
> Reproductions below remain historical evidence, not the current defect status.

2026-10-15 · ivac-oznar · checkpoint authorized by user.
Engineering triage delivered; bounded stabilization proposal awaits acceptance.
No production correction, platform installation, physical trial or release performed.

## Scope and human decisions

The approved release roadmap requires macOS, Windows and Linux. User clarified
Intel x64 for Windows/Linux, latest Ubuntu and initially only Windows 7 available.
After the compatibility review below, the user explicitly selected **Windows 11 x64
and finding a test environment**. Do not reintroduce Windows 7 as a V1 requirement.
User explicitly answered **GEOM1 not yet reviewed**: ivac-e2jnb remains OPEN.
Approval of this checkpoint does not constitute GEOM1 acceptance.

| V1 desktop target | Bounded qualification target | Package proposal | Current evidence / missing gate |
| --- | --- | --- | --- |
| macOS | Apple Silicon arm64, macOS 26.5.2 reference host | .app in .dmg | Historical native evidence; current integrated clean install/upgrade and CPS workflow pending. Intel macOS not implied by Windows/Linux x64 answer. |
| Windows | Windows 11 x64, exact edition/build recorded in test manifest | NSIS setup .exe initially; MSI expansion not required | Explicit user decision. Obtain accessible licensed physical host or VM; native install/runtime/upgrade and 3D test pending. No environment provisioned here. |
| Linux | Ubuntu Desktop 26.10 amd64, exact image/build and CPU capability recorded | .deb initially; AppImage/RPM expansion not required | “Latest Ubuntu” pinned as of this checkpoint; no actual host/installation verified. Check actual workshop OS before execution. Native WebKitGTK/CPS/3D qualification pending. |

Package choices are bounded stabilization proposals using existing build targets,
not new installer implementation. No support claim for all Linux distributions,
Windows ARM, Intel macOS or all historical OS versions. Pin candidate package hash,
OS build, GPU/driver and WebView version; repeat on the actual workshop configuration
if it differs. A VM's functional pass does not prove physical GPU performance.

### Windows compatibility evidence

Repository: `rust-toolchain.toml` pins 1.88.0; desktop workflow targets
`x86_64-pc-windows-msvc`; Tauri 2 renders through WebView2. Rust's official standard
Windows targets require Windows 10 beginning with 1.78. Microsoft identifies 109 as
the final WebView2 runtime for Windows 7/8.1. These establish that the current standard
build cannot be promised for Windows 7; this is not a runtime experiment on that OS.
A bespoke legacy port would require separate feasibility, not an installer flag.
Some generic Tauri prerequisite pages still mention Windows 7; they do not override
the actual Rust target/runtime constraints. Windows 11 was chosen explicitly by user.

Primary sources checked 2026-10-15:
- [Rust Windows baseline](https://blog.rust-lang.org/2024/02/26/Windows-7/)
- [Microsoft WebView2 Windows 7 support boundary](https://blogs.windows.com/msedgedev/2022/12/09/microsoft-edge-and-webview2-ending-support-for-windows-7-and-windows-8-8-1/)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)
- [Canonical Ubuntu 26.10 announcement](https://canonical.com/blog/canonical-releases-ubuntu-26-10-stonking-stingray)

## Frozen functional baseline for stabilization

Use the [approved V1 matrix](FABCAM_V1_SCOPE.md). No capability expansion:
SVG/DXF; GEOM1 bounded line repair and explicit unresolved bindings; single-sided
metric stock-top Z and numeric XY work origin; machine/project physical-tool copies;
persisted manual rates and bounded existing FS; ordinary Profile, Pocket and Drill;
Direct/supported Ramp, ordinary qualified tabs/leads/direction; decoded ShopBot output,
resolved preview, critical/stale export blocking and v1–v7 persistence.

Retain limited CAD1, UX3 surfacing, templates and V-Carve groove/spine modes. Do not
claim full-region V-Carve or all modifier combinations. Optional physical support
claims require the corresponding trial. GEOM2-A1, full geometry editing, nesting,
two-sided, advanced inlays/rest machining and new post capabilities remain outside
this stabilization lot. WJ research is not launched by this checkpoint.

## Current-tree triage

Evidence levels distinguish executed reproduction from static audit and historical
reports. Passing characterization tests can demonstrate an existing defect.

| Issue / priority | Current evidence | Release disposition and smallest next work |
| --- | --- | --- |
| ivac-j2po — blocking geometry correctness | Executed `geometry_source_ux::xor_currently_stops_at_an_empty_intermediate_result`: 3 identical squares return empty. Independent parity expectation is one square. `source_combine.rs::combine_xor` exits on intermediate empty. | Fix fold termination and independently test 2/3/4 operands, empty intermediate followed by disjoint shape, resulting CAM/output. No broad boolean-engine rewrite. Until fixed or effectively refused, arbitrary XOR cannot be supported. |
| ivac-dli7 — blocking source/preview correspondence risk | Current shared Auto test gives area 400−144+16=272 and 2 regions. Static `pipeline/offset_builder.rs` Pocket Auto skips every selected descendant and treats nested selected objects as islands. Existing 2-level donut regression does not qualify 3-level parity. No new end-to-end emitted-cut reproduction here. | First add 3-level independent generated-path/region oracle; align region intent or explicitly refuse unsupported depth before export. This risk stays blocking until disproved/resolved; do not label it fixed by 2-level tests. |
| ivac-hs9i — blocking physical-tool identity risk | Static chain still present: operation gear sets numeric `toolsDialogFocusId`; App opens inventory; ToolLibraryDialog finds that ID in inventory draft. Distinct project/inventory copies with same ID are possible. 11 tool inventory/review tests pass but do not prove shortcut target. No new GUI reproduction here. | Reproduce project T1 D12 versus inventory T1 D6.35; make editing target explicit and preserve independent copies/rates. Verify Apply/Cancel/Undo/reopen/stale output; no new tool domain model. |
| ivac-5g51 — blocking package-integrity guard | Executed isolated production script with only placeholder `.wasm`: exit 0, “up to date”, JS/types/manifest absent. Actual package untouched. Source also permits stale-but-present output when wasm-pack unavailable. | Require complete matching release assets, test partial/stale/no-toolchain cases and clean build on three targets; preserve intentional dev fallback only if clearly separated from release. No CAM change. |
| ivac-vt6e — text workflow defect, release-blocking if advertised text is not inspectable | Static `OperationsList.svelte` warns no drawing when transformedImport absent, without text exception. Historical empty canvas after reopen not reproduced in this checkpoint. | Actual text-only save/reopen on desktop/normal+narrow viewport; separate misleading warning from rendering/fit issue. Fix only reproduced cause, preserve source/output guards. |
| ivac-2zll — explicit refused RPM transition | Executed 7 ShopBot guard tests include `spindleSpeed` refusal with no partial successful output. Historical rough17000/finish10000 whole-job case not rerun. | Keep unsupported transition refusal in V1 and make it understandable before generation where feasible. Do not unify saved values or broaden spindle control here. New dynamic RPM semantics need separate approval. |
| Windows/Linux/macOS candidate packages | No current three-platform installation qualification | Required independent gate. Existing workflow config and macOS/browser evidence cannot close it. |
| GEOM1 and FS/UX/cancellation human gates | GEOM1 explicitly not reviewed; other gates unchanged | Human acceptance required. No automatic closure from this checkpoint. |

Deferred: curved rectangular-tab precision beyond the ordinary matrix; tighter
CPS cancellation latency (216/218 ms historical p95 remains above 200 ms); advanced
residual analysis, nonzero numeric Z/two-sided support, cosmetic polish. Cancel must
still never yield exportable partial output. GEOM1 buffered cancellation has separate
latency limits and does not inherit PERF1 guarantees.

## Executed checks and limitations

- [Geometry source log](v1-triage/geometry-source.log): 7 tests pass; includes deliberate
  characterization of broken XOR and shared three-level Auto region expectation.
- [Frontend log](v1-triage/frontend.log): 22 passes across inventory, tool review and
  geometry-source UI. These are logic tests, not desktop/UI interaction qualification.
- [ShopBot guards](v1-triage/shopbot-guards.log): 7 pass, including spindleSpeed rejection,
  narrow V-Carve/tool/movement pairing and no partial output on refusal.
- [Isolated partial-WASM reproduction](v1-triage/partial-wasm-repro.json): production
  script copy, placeholder wasm only, no source inputs. Demonstrates completeness
  blind spot, not a claim that a full Vite build succeeds with missing bindings.
- Source/config hashes in [manifest](v1-triage/source-manifest.json). Current dirty tree
  preserved; no production edits or full regression/build run in this checkpoint.

Commands (repository root unless stated):

```sh
RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --test geometry_source_ux -- --test-threads=1
RUSTC_WRAPPER='' cargo test --offline -p ivac-cps --test shopbot_guards
# frontend directory, Node 24 on PATH
npm test -- --run src/lib/state/tool-library-review.test.ts src/lib/state/tool_inventory.test.ts src/lib/state/geometry-source-ui.test.ts
```

No current Windows/Linux runtime, physical machine, complete deep-Pocket path oracle
or GUI tool-shortcut/text-reopen reproduction is claimed. These specific missing
proofs are entry criteria for the bounded fixes, not a reason to declare green.

## Bounded stabilization sequence proposed for approval

1. **Geometry correctness:** reproduce deep Pocket Auto emitted behavior and fix XOR;
   qualify intended regions, no overcut/omission, offsets, preview and decoded output.
   Keep ShopBot-specific syntax outside the generic geometric correction.
2. **Tool identity and text lifecycle:** actual UI reproductions first, minimal fixes,
   saved values/history/stale-output regressions and focused Professional Workshop UI review.
3. **Release asset integrity:** complete WASM/package guard, clean reproducible build;
   then macOS .dmg, Windows NSIS and Ubuntu .deb install/upgrade workflows, native CPS
   and identical reference output. Secure a Windows 11 x64 host/VM before claiming this gate.
4. **Integrated candidate qualification:** core/CPS/frontend regression, schema/check/lint,
   current import-to-export/persistence/refusal matrix on all three targets and PERF1
   cold/warm evidence. Reconcile independent human reviews; no blanket closure.
5. **Workshop + RC:** only after candidate software readiness and explicit authorization,
   operator campaign from V1 scope, documentation and final human go/no-go.

Each fix begins with its independent counterexample. Do not mask unsupported output
or change source selection to make a test pass. Package formats and bounded lot scope
are for approval at this handoff; no implementation starts automatically.

## Handoff

Three-platform direction and Windows 11 choice confirmed; Ubuntu latest pinned to
26.10 for this plan. GEOM1 human review remains required. Target package/OS-build
qualification and listed unresolved defects prevent release readiness.

**HARD STOP — approve the bounded V1 stabilization lot before implementation.**
