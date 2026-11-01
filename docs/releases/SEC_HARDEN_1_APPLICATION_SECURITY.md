# SEC-HARDEN-1 — Application security and import robustness

2026-11-01 · **Accepted bounded software-qualified milestone; locally consolidated. HARD STOP.**

## Identity and scope

Implementation is isolated on `feature/sec-harden-1`, based on accepted
SAFETY-FIX-1 documentation commit `c33f69f6d6181f7bac2b1cebcdfe5b830f2d5b1c`
(implementation `aac6064`, software/tests `e09fdca`). Canonical `FabCAM-v1.1`
remains `16eb34fd07354627b3c3c2f48a227b30ba1cc3c2`; SAFETY-FIX-1 was committed
on its own branch, not merged into canonical. The authorized local consolidation
is recorded below; the qualified source bytes are unchanged.

The final logical software inventory contains **1,330 files**; SHA-256:
`498fe344f19035d8b6e9fc1d874790373c212aa5073f693774a4fa9ac289c6d0`.
[Per-file inventory](evidence/sec-harden1/software-files.json) uses the inherited
sorted `path + NUL + file SHA256 + LF` method, resolving the inherited image
symlinks to their logical bytes. Documentation and ignored runtime artifacts are
outside this software inventory. No goldens or machining algorithms were changed.

## Confirmed boundaries and corrections

| Boundary / affected implementation | Before / reachable condition | Correction and preserved controls |
| --- | --- | --- |
| Tauri `capabilities/default.json`, `tauri.conf.json` | A compromised bundled renderer had ambient home/documents/downloads/temp filesystem grants; CSP was absent. | Remove ambient scopes and `fs:default`; retain explicit operations with native-dialog dynamic file grants. Production CSP restricts sources, IPC and WASM; no general JS `unsafe-eval`. |
| `commands.rs::import_path`, `watch_source_paths` | Renderer-supplied source paths bypassed the native selection boundary. | Canonical target checked against dynamic FS scope; at most 128 watches; ungranted sources are omitted and reported. Native launch paths are granted by an internal non-command helper. |
| Custom export and preset storage | Separate authorization already existed. | Preserve custom export's canonical destination, symlink refusal, dynamic grant and transactional publication. Preserve fixed app-data storage, safe preset IDs, CAS/locking and existing symlink/size guards. These were not equivalent to the broad raw plugin scope. |
| HTTP `security::admit`, all heavy route handlers | CORS headers did not prevent a foreign-Origin simple multipart request from running. CPU work had no common admission ceiling. | Reject Origin before body extraction; two admitted jobs, no queue, excess 429. CPU blocking workers retain permits until actual exit. Existing 64 MiB request limit and CPS budgets remain. |
| `input/limits.rs`, SVG and DXF readers | Compressed XML, references and block expansion could allocate before product limits; deep DXF recursion silently omitted geometry. | Admission and emission budgets fail the entire import; no successful partial drawing. Ordinary import coordinates and precision remain unchanged. |
| `svg_font.rs::tessellate_arc`, `sim/stl.rs::parse_stl` | Extreme finite arc ratios could derive unbounded subdivision; actual-length checking alone did not impose a product mesh cap. | Check prospective subdivision before allocation; retain STL checked byte-length/count consistency and add explicit input/triangle caps. |

These are local trust-boundary/resource protections, not a sandbox against hostile
native processes running as the same OS user.

## Native file workflows and CSP

Production policy permits bundled scripts, `wasm-unsafe-eval`, self fonts/images,
image data/blob, self/blob workers and Tauri IPC. Styles retain `unsafe-inline`
for current UI styling; objects, base replacement, framing and form submission
are disabled. Development alone adds the established loopback Vite/HMR ports.
Tauri's CSP handling and WASM directive follow the
[official CSP documentation](https://tauri.app/security/csp/).

Raw FS operations have no ambient directory scope. Dialog grants apply to an
individual selected path; the locked dialog plugin shares that grant between
read/write operations, allowing an opened project to be saved again. This does
not grant every path under its directory. Browser-file import sends bytes to Rust,
without a renderer-chosen temporary-file write.

Recent-project open and source reload reacquire missing authorization with a
native picker; only the actually selected path is read. Reload records the selected
path and geometry atomically, including Undo. Saved projects do not silently grant
access to their embedded source paths. Already granted watches continue; denied
ones produce a localized notice. OS startup file-open authorization is internal,
not an IPC grant command. Missing source leaves are normalized through their
existing parent so an already authorized watch can survive delete/recreate.

Qualification uses disposable files and the Tauri mock runtime/real FS scope,
frontend picker mocks, existing transactional-export tests and source inspection.
Selected, outside-scope and retargeted symlink cases are checked. **Installed native
WebView/dialog/plugin acceptance was not executed.** Browser CSP qualification is
not a substitute for macOS installed-app acceptance or a claim of race-free path
authorization against a hostile same-user filesystem mutator.

## HTTP policy and cancellation

Default remains `127.0.0.1:8766`. Non-loopback binds now refuse startup pending a
separately reviewed authenticated remote mode. IPv6 loopback binds are supported.
Explicit origins default to localhost/127.0.0.1 development ports 5173/1420,
`tauri://localhost`, plus the bound server origin. `IVAC_CORS_ORIGINS` may replace
the development list with explicit scheme/authority values; wildcard/`any`, `null`
and malformed configuration refuse. CORS responses use the same policy.

Missing Origin supports local CLI clients. `null`, foreign and duplicate Origins
refuse 403; missing Origin with `Sec-Fetch-Site: cross-site` also refuses. This is
browser request admission, **not authentication**: local non-browser callers can
omit/spoof headers. The service must remain loopback-only. Older browser requests
without both Origin and Fetch Metadata cannot be distinguished from CLI traffic.

Generation, raw/SSE streams, two-sided generation, imports, geometry/text/STL,
preset work and exposed CPS inspection/listing share two permits. Health/version,
OPTIONS and cancellation remain available after the Origin check. A 120-second
deadline covers the request before its response, including incomplete bodies and
buffered generation. On handler/body drop or timeout, cancellation is signalled.
A worker clone holds the permit until it exits; disconnect cannot admit unlimited
orphan jobs. SSE token registrations are removed through RAII.

Buffered, paired and raw generation forward cooperative cancellation through the
existing pipeline; streaming channel writes observe cancellation/disconnect.
Pre-cancelled generation publishes no program. Existing motion/publication guards
remain authoritative. Parsing/library phases, preparation, drawing generation and
CPS execution are not universally preemptible: parsers are bounded and CPS keeps
its existing execution budgets. A connected slow stream can retain a slot; there
is no universal stream wall-clock or idle deadline. This is bounded containment,
not a guarantee of instantaneous worker termination.

## Parser budgets and intentional new refusals

- Import bytes/path reads: 64 MiB; SVG decompression/XML: 16 MiB before `usvg`;
  XML nodes: 50,000; nesting: 64; admitted shape/path/point work: 200,000, with
  an additional emitted primitive ceiling. DTDs remain disabled.
- SVG `<use>` accepts bounded flat local references. Nested/recursive references,
  missing targets, duplicate IDs, conflicting href/xlink and marker expansion
  refuse explicitly. Markers must be flattened to explicit paths. Images remain
  excluded from machining geometry; image resolvers cannot read external files.
- DXF: bounded input/group-pair scan (1,000,000), recursion depth 32, visited
  expansion/emitted primitives 200,000, expanded text 16 MiB. Deep chains, repeated
  blocks and excessive output refuse without a partial imported result. Existing
  unsupported OCS/MINSERT and HATCH contracts remain in force.
- SVG-font path tokens/points and total font outlines are bounded at 200,000;
  arc subdivisions at 65,536. Nonfinite, zero or unrepresentably small angular
  steps refuse before iteration. Ordinary bundled-font geometry tests remain.
- STL: 64 MiB and 1,000,000 triangles, in addition to exact binary byte-length
  validation and bounded ASCII vertex accumulation. No transport/mesh redesign.

No exponential-scale or multi-gigabyte test was performed. New guards are shared
by core/native/HTTP/WASM where those transports expose the corresponding parser.
They are conservative product ceilings, not a general proof of optimal parser
memory usage; third-party parsing allocations and noncooperative phases remain.

## Qualification and evidence

[Runtime comparison](evidence/sec-harden1/runtime-comparison.json),
[HTTP checks](evidence/sec-harden1/http.json),
[WASM checks](evidence/sec-harden1/wasm.json),
[gate receipt](evidence/sec-harden1/qualification.json).

- **109/109 exact fixed corpus comparisons** against the immutable accepted
  SAFETY-FIX-1 CLI: 96 complete accepted responses and 13 exact refusals. This
  compares program, decoded moves, attribution and response content; no golden rewrite.
- SAFETY-FIX-1 targeted CLI and HTTP buffered/raw/SSE/paired refusals pass with no
  program publication; supported profile, ramp and shallow paired controls are exact.
- Real HTTP: 13 bounded checks, including baseline foreign multipart acceptance,
  new allowed/blocked requests, JSON preflight, CLI access, admission 429, disconnect
  and recovery. A deterministic worker sentinel separately proves cancellation,
  no excess worker start and permit retention until worker exit. Short live jobs
  may finish before disconnect; their recovery check alone does not prove early exit.
- Real WASM: buffered/pair/progress motion refusals, safe native-equivalent profile,
  ordinary SVG import, deep SVG refusal and forged STL count refusal pass.
- A separate loopback static origin served the built frontend with the **literal
  production CSP header**. In the in-app browser, Explore example imported its DXF,
  Profile generation completed with 187 moves and 107 program lines, and 3D rendered;
  console error/warning capture was empty. The example retained its own blocking
  diagnostics; this is startup/worker compatibility, not a machinable-job acceptance.
- Independent **Astra / Medium**, targeted read-only review: reference-resolution
  ambiguity, point clone charging and native reopen/reload authorization gaps were
  corrected. Final review found no further concrete blocker within this scope;
  final server timeout/Fetch Metadata/IPv6 additions received a narrow follow-up.

**Canonical `scripts/pre-release.sh`: 20/20 mandatory groups PASS, one complete run; 1,686 Rust/Tauri tests and 1,570 frontend tests passed.**
One inherited Rust ignored test and one frontend skipped test remain under the unchanged
canonical suite policy; no mandatory group was skipped or weakened. The frozen
1,330-file inventory remains byte-identical after the gate. Final native/HTTP/WASM
artifacts were rechecked after the gate; the 109 comparisons remain exact. The unchanged
canonical script uses a disposable alternate Git index containing only the reviewed
generated i18n key change for its drift comparison. The real index stays empty.
Generated outputs are included in the software inventory. Build artifacts and raw
logs remain local/ignored; no installer is built.

Existing V10 save/reopen, manual stock/import coordinates, TOOL-META, Main/Finishing,
strict and reviewed presets, collection V2, transaction/export, SAFETY-FIX-1 and
security regressions are exercised by the canonical suites. No schema, feed/speed,
postprocessor or cutting-role contract changes are introduced.

## Preservation and local consolidation

[Preservation receipt](evidence/sec-harden1/preservation.json): canonical, frozen
V1, safety audit and accepted safety implementation retain their HEAD/status hashes;
133 audit artifacts and all 1,325 accepted SAFETY source files remain unchanged in
their original worktrees. Archives/user files are not relocated, edited or deleted.
Only this implementation worktree contains the new changes.

[Exact proposed path inventory](evidence/sec-harden1/proposed-commits.json).
Reviewed dependency order (original path proposal retained as checkpoint evidence):

1. `fix(security): bound native access, HTTP work and import expansion` — core
   readers/cancellation, server policy/handlers, native capabilities/CSP/IPC,
   frontend permission recovery/localization, dependency edges and generated keys.
2. `test(security): qualify admission, parser limits and preserved CNC output` —
   focused core/server/Tauri/frontend regressions, including existing test-router
   adaptations required by middleware. Existing inline unit tests may be kept with
   their implementation file; the existing HTTP test-router adapters are included in implementation commit 1
   so middleware dependencies remain coherent at that revision.
3. `docs(security): record bounded SEC-HARDEN-1 qualification` — this report,
   sanitized manifests/receipts and current STATUS/roadmaps/index. Exclude raw logs,
   private projects/presets, caches, screenshots and compiled assets.

### Acceptance and consolidation receipt

The maintainer accepted SEC-HARDEN-1 and authorized three local commits. The original
qualification/proposal/preservation JSON receipts describe the **pre-consolidation
checkpoint**, including their `commits_created: 0` and proposed/not-staged fields;
they are preserved rather than rewritten as newly executed tests.

- Implementation: `73371e001e920ee422089887f8ba3b2d0569afbd` (25 reviewed paths, including generated i18n keys and
  existing HTTP test-router adapters needed by the new middleware).
- Software/regressions: `32b5fbbb6fb2b6a170c0531549fc081e5d0c2c10` (4 additional regression paths; depends on
  implementation). All **1,330 committed logical software files match** the
  accepted inventory, byte-for-byte, including symlink-resolved assets.
- Documentation/evidence: the commit containing this receipt; its exact SHA is
  recorded in the final local consolidation receipt and human handoff. It changes
  documentation only and depends on the software commit above.

[Consolidation evidence](evidence/sec-harden1/consolidation.json) records the exact
path groups and verification. No executable-source bytes changed, so the complete
release gate was **not rerun**. The existing 20/20 qualification remains applicable.
Current STATUS/roadmap entries are replaced in place; no new stacked banners.

**Accepted for bounded software security containment only.** Installed-native file
workflows, authenticated remote HTTP access and physical machining remain explicitly
**unqualified**. Other worktrees, historical references and retained evidence are
preserved. No merge, push, installer, release or hardware operation was performed.
**HARD STOP after the three local commits.**
