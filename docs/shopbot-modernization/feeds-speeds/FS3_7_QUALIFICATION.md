# FS3.7 — software architecture and source-pack qualification

Date: 2026-10-12. Source baseline: 8271dd3. Subsequent explicit user instruction authorizes publication of FS3.7/FS3.8 on feature/shopbot-mvp; physical qualification gates remain separate.
Engineering issue: ivac-3957. Human review gate: ivac-tm7u (open). FS4 unstarted.
Applied skills: feeds-speeds, ivacam-architecture, cam-validation,
project-documentation and cam-ux-ui. No physical evidence is claimed.

## Implementation and phase disposition

A: [audit](FS3_7_AUDIT.md) identifies KEEP/EXTEND/REPLACE boundaries.
B: [ADR-0005](../adr/ADR-0005-source-packs-and-cutting-knowledge.md) resolves
shared schema/engine, application taxonomy, storage, reconciliation and privacy.
C: pure JSON loader, bounded validation, reviewed registry, native app-data storage
and browser IndexedDB are implemented. D: Sienci becomes one generic pack with
original six-row numerical data and verified license. E: explicit taxonomy,
specificity ranking and conservative comparable-record selection are implemented.
F: generic source Settings and secondary record/rank/provenance explanation keep
the compact inspector workflow. G: analytical, native, frontend and real-browser
qualification and persistent documentation are complete for human review.

See [source-pack contract](SOURCE_PACKS.md) for exact schema/policy and
[license matrix](DATA_SOURCES.md) for distribution decisions. No further data
coverage or physical calibration is inferred from completion of these phases.

## Executed checks

All commands run from the checkout (frontend commands from frontend/), with
RUSTC_WRAPPER cleared and the installed Node 24 toolchain. Existing dependencies
and pinned Cargo.lock were used.

| Command / check | Result | Evidence scope |
| --- | --- | --- |
| cargo test --locked -p ivac-feeds-speeds | 69 tests + one doctest pass | 38 existing engine, 12 dataset and 19 source-pack tests; parsing/version/license/distribution/identity/provenance, taxonomy, tool traits, normalized units and attribution, exact/fallback ranking, tested preset precedence, mode selection, disagreement, incomparable contexts, deterministic ordering, public registry |
| cargo test --locked -p ivac-core feeds_speeds | 13 adapter tests pass | Applied/manual persistence, public migration numerical parity, private-body omission and unavailable-source reopen with resolved rates |
| cargo test --locked -p ivac-tauri source_packs | Two storage tests pass | Validated local-only files, immutable duplicate version refusal, invalid JSON refusal, Git-root rejection; temporary canonical directory only |
| cargo test --locked -p ivac-core --features cps --test shopbot_m2_post --test cps_posts | Eight ShopBot goldens + four CPS integration tests pass | Actual import/CAM/post regression; FANUC reference oracle is optional when absent |
| npm test | 112 files; 1,074 pass, one optional skip | Six new source/compatibility/privacy tests and complete frontend regression suite |
| npm run check | Zero errors/warnings | Svelte/TypeScript contract |
| npm run lint | Pass | ESLint and frontend Prettier |
| cargo clippy --locked -p ivac-feeds-speeds -p ivac-core -p ivac-tauri --all-targets -- -D warnings | Pass | Includes test targets |
| cargo fmt --all -- --check | Pass | Rust formatting |
| cargo run --locked -p xtask -- schema-check | Pass | Generated schema agrees with Rust; frontend API regenerated |
| npm run build | Pass, release WASM rebuilt | Actual validateSourcePack export and production browser build; existing mixed static/dynamic Tauri event import warning is nonfatal |

An initial core post run without `--features cps` executed zero tests and is not
counted as qualification; the corrected feature-enabled run above supplies evidence.
Clippy caught helper placement after a test module; it was moved and rechecked.
The import handler now captures its input before awaiting validation so cleanup
cannot use a cleared event.currentTarget; invalid and duplicate imports were
retested against the rebuilt bundle.

## Real browser protocol and evidence

Production preview, local WASM, 1280 × 720 viewport, 360 px inspector. All projects
and local pack data below are explicitly SOFTWARE ONLY. The local test pack was
created outside Git in /private/tmp, derived from the attributed CC BY-SA Sienci
fixture with synthetic altered feeds. It is not a calibrated or production pack.

1. Generic Settings shows only the reviewed built-in Sienci pack initially, with
   Public reference and CC-BY-SA-4.0. Importing the synthetic private/local fixture
   adds an enabled Local-only version 1 pack. Reload retains it. Invalid `{}` and
   reimport of the same immutable version are refused without adding a pack.
2. Open the existing FS3.5 manual-saved software fixture (17,000/1,500/810,
   pass depth 2 mm). Set depth to 2.8 mm and explicitly open Review/Recalculate.
   With Sienci + synthetic local enabled, Prudent proposes 17,000/1,000/810,
   2.8 mm and 0.0588 mm/tooth. Default source is “2 compatible sources”; a visible
   disagreement notice appears. Feed remains 1,500 before explicit Apply.
3. Explicit Apply changes feed to 1,000. Actual Save produces guarded
   ivac-project-knowledge version 4. Inspect the saved JSON: rate slots and safe
   selected/contributing IDs exist, full proposal is absent, and the private note
   sentinel PRIVATE-FS37-SENTINEL is absent. Retained artifact:
   [private-safe software project](FS3_7-software-private-safe.ivac-project.json).
   This is a software fixture containing safe identities/values, not a private pack.
4. Open the saved project on a separate localhost origin with no local pack.
   Inspector reports unavailable source provenance and keeps 17,000/1,000/810,
   pass depth 2.8 mm. Generate succeeds: 34 moves, 37 G-code lines, no warnings.
   This demonstrates saved-rate authority without recomputing from unavailable data.
5. With both packs enabled, pass depth 1 mm gives explicit incompatibility:
   requested 1 mm versus required 2.8 mm; duplicate source profile IDs do not break
   the refusal list. Disable the local fixture and restore 2.8 mm: Sienci alone
   proposes the original 17,000/1,620/810 and 0.0953 mm/tooth. Manual feed is kept.
6. Disable the synthetic local pack on the first test origin after verification.
   No real user private dataset or machine trial was imported.

[Settings capture](FS3_7-sources.png),
[two-source proposal capture](FS3_7-multi-source.png),
[missing-local provenance capture](FS3_7-missing-local.png).

## Compatibility, boundaries and remaining qualification

Legacy dataset requests still use the original single-source Engine; unchanged
Sienci values are checked against that path. Portable v1/v2/v3 remain readable;
v4 guards new knowledge metadata against older consumers. Missing packs are
informative only. Manual edits and post generation remain independent of pack
availability. No private source record bodies, original classifications, notes or
normalized source quantities are persisted in shared projects.

Native storage uses Tauri's platform app-data API and is compiled/tested on macOS;
a new packaged desktop app and actual Windows/Linux storage runs were not performed.
Browser origin storage is distinct from native app storage. No private backup/export,
cloud synchronization or telemetry is introduced. Immutable versions may be disabled
and replaced by an authored new version; a management/removal workflow is deferred.
Invalid installed local files fail closed, without changing saved cutting rates;
quarantine/recovery UX is deferred. Runtime import metadata never grants bundling
rights; public additions require reviewed registry changes and actual license evidence.

Only the pinned six-row Sienci subset is APPROVED_PUBLIC. User-entitled original
local measurements use LOCAL_ONLY infrastructure, initially no production data.
FabCAM/Sorbonne public original records and Onsrud/Amana/DATRON/Sorotec remain
LICENSING_REVIEW_REQUIRED. No new lab public-data license has been selected.
Physical Tested/Validated labels require exact scopes, trial evidence and criteria;
no lab preset or source agreement is treated as qualification performed here.

Human review should confirm schema/distribution/privacy decisions, conservative
matching and source explanation, then explicitly approve this checkpoint. FS4
readiness means that architecture can receive scoped physical evidence; it does
not mean cutting parameters are physically validated. STOP before FS4.
