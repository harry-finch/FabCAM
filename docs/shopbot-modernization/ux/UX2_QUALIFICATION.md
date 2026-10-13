# CAM2 / UX2 — Stock, Origins, WCS & Fixtures

Date: 2026-10-13. Software qualification complete within the scope below.
Engineering: ivac-69ak. Human review: **ivac-97fb**, pending.
UX1 review ivac-cb64 was approved by the user's instruction to continue.
No CAM3/CAM4 implementation, publication, physical test or new native bundle.

## Qualified scope and compatibility

Single-sided milling supports numeric XY work zero. The generic pipeline subtracts
that point once before CAM; bundled ShopBot output and returned resolved preview
use work coordinates. The desktop adds the captured XY point for drawing-frame
preview/simulation and pocket regions. Source lines, rates, depths and emitted
program text are preserved. This is a coordinate mapping, not post text rewriting.
See [ADR-0008](../adr/ADR-0008-work-origin-frames.md).

Manual stock retains its persisted bbox-centered reference and offsets. Primary
X/Y controls show/edit the effective minimum corner in drawing coordinates; the
secondary readout shows that corner relative to work zero. Old files do not move
on reopen. The base still follows drawing bounds: this is not a new fixed absolute
stock-anchor model. A 2D work-zero marker and stock-corner shortcut clarify frames.

The existing stock/fixture/project schema and persistence version are unchanged.
Fixture box/cylinder/polygon authoring reuses ordinary history and saved records.
XY is the shape center in drawing coordinates; polygon vertices are local to that
center. Z bounds are absolute in the existing operation-depth frame. Example
shape dimensions and height defaults must be edited to match real obstacles.

## Acceptance evidence

| Case | Evidence | Result |
| --- | --- | --- |
| UX2-01 stock reference | Pure corner-patch test preserves legacy offsets; actual saved setup has offsets 190/190 and effective corner 0/0 for bbox 10..110, stock 500 | PASS |
| UX2-02 XY output | Independent SBP decoding compares Profile square/circle, island Pocket and Drill paths before/after nonzero work zero; unchanged depths/rates and deterministic output; LinuxCNC native generation also checked | PASS |
| UX2-03 coupled inputs | Explicit approach point and polar Drill center translate with geometry; relative pattern dimensions remain relative | PASS |
| UX2-04 preview/regions | Drawing-frame adapter preserves source identity/program text; arc centers and pocket holes translate; fixture collision remains identical under translated work zero | PASS |
| UX2-05 unsupported setup | Nonfinite origins, invalid fixture sizes/self-crossing polygons and nonzero numeric Z reject before output; unsupported WCS remains visible rather than silently coerced | PASS |
| UX2-06 fixture workflow | Actual browser adds all three shapes, edits box, removes polygon, Undo/Redo restores/removes it; Save/reopen retains three shapes and work zero 20/30 | PASS |
| UX2-07 export gate | Collision setup produces 7 warnings, 1 critical; Download refuses with default block-on-critical setting. Moving Clamp 1 X from 7 to -15 invalidates prior generation; regenerate produces 6 noncritical warnings and Download succeeds | PASS |
| UX2-08 authored values | Actual saved/reopened operation preserves requested Ramp 3°, finish feed 1500 mm/min, depths and ReferenceData; no recommendation calculation or silent rate change | PASS |
| UX2-09 two-sided scope | Genuine two-sided CPS or translated XY rejects before legacy wrapper could emit invalid scaffold/unshifted registration; authored setup unchanged | PASS, unsupported capability |

Fixture candidates are checked against the resolved execution path, including
rapids and declared cutter/body/holder geometry. Without declared body/holder
geometry, candidate checking conservatively extends occupied height above the tip
for the known cutter XY footprint; a separate warning states wider holder clearance
is unknown. This does not certify machine clearance. The existing user setting
for critical-warning blocking remains authoritative; no automatic avoidance.

## Actual browser artifacts

- [Recovered software setup](../UX2-software-setup.ivac-project.json): work zero
  20/30, stock minimum 0/0, three outside fixtures, requested Ramp/manual feed.
- [Collision software setup](../UX2-software-collision.ivac-project.json): same
  origin; Clamp 1 center 7/60, size 20×10, occupied Z 0..10.
- [Collision/export refusal](UX2-fixtures.png).
- [Stock and work zero](UX2-stock-origin.png).

These are software-only examples using an existing public reference context,
not ShopBot machine calibration, cutting advice or a physical test protocol.
Actual recovered SBP begins J2(-10,-23.175), corresponding to drawing (10,6.825)
with work zero (20,30); M3 depths remain unchanged. Its finish feed MS25 mm/s is
1500 mm/min. A starting unknown operator position is not fabricated into preview.

## Checks

Native serial checks: 1,056 core library, 11 M1, 4 snapshots; final full ShopBot
suite 21 cases plus the subsequently added polar/approach case, 22 cases validated.
The affected fixture case was rerun after the final unknown-holder warning.
Frontend: 116 files, 1,096 passing / 1 optional skipped. Svelte/TypeScript check,
ESLint/Prettier, Rust fmt, Clippy with warnings denied, optimized complete WASM
and frontend build pass. Final MSRV-compatible Option predicates preserve behavior.
Tests use serial execution for the known global pocket-cache race ivac-htc8.

Representative commands (scratch target; root target/debug stays absent):

```sh
CARGO_TARGET_DIR=/private/tmp/fabcam-ux2-target CARGO_INCREMENTAL=0 RUSTC_WRAPPER='' cargo test -p ivac-core --lib -- --test-threads=1
CARGO_TARGET_DIR=/private/tmp/fabcam-ux2-target CARGO_INCREMENTAL=0 RUSTC_WRAPPER='' cargo test -p ivac-core --features cps --test shopbot_m2_post -- --test-threads=1
CARGO_TARGET_DIR=/private/tmp/fabcam-ux2-target RUSTC_WRAPPER='' cargo clippy -p ivac-core --features cps --all-targets -- -D warnings
npm --prefix frontend run test
npm --prefix frontend run check
npm --prefix frontend run lint
npm --prefix frontend run build
```

## Human gate and limits

Open the two software fixtures and review stock corner versus work zero, obstacle
editing and explicit regeneration/export refusal. No physical test is required
for this software gate. No new native app was delivered; historical M4 app is older.
Nonzero Z remains blocked (ivac-u1ko); genuine two-sided CPS/translated registration
needs separate qualification (ivac-jtkj). Unsupported raw G-code includes with XY
translation and non-milling translation reject rather than guess their frame.
Native unshifted two-sided behavior is historical, not newly qualified here.

CAM3 tabs/leads remains next proposed milestone after approval. FS4 stays deferred
and nonblocking; FS data, mathematics, evidence and UX remain frozen.
