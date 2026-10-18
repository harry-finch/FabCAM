# SAFE-Z extended checkpoint — 2026-10-18

**Targeted corrections implemented; full SAFE-Z / TEST1 D2 NOT qualified.**
User accepted TEST1 bounded review ivac-930yx and authorized this checkpoint.
Local pipeline60, version0.9.0-beta.2; base commit2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b
plus existing uncommitted audit work. No commit, push, package or publication.

## Corrected and reproduced before modification

### Swept holder on sloped movement — ivac-kot1d

Tip moves(0,0,0)→(10,0,11/20/40), cutter radius1, declared cylindrical body
radius5 and height20. A box/cylinder fixture at(1,4), Z5..10 overlaps the body
at an interior pose even though the cutter footprint misses. The former detector
used the cutter radius when fixture top was below the maximum tip Z, overlooking
that the holder reaches the fixture near the lower pose. The independent witness
failed before the fix; both directions and both fixture shapes now pass.

`sim/fixture_check.rs` now uses the maximum declared body radius over the whole
move after the existing Z gate. This is a conservative bounding envelope, not an
exact time-correlated swept-solid computation: false positives remain possible.
No tolerance relaxation, stock-removal proof or automatic motion correction.
Existing arc-capsule margins and separate unknown-body policy are preserved.
Analytic clear controls and contacts at ±1e-8mm around the cylinder boundary pass;
that numerical tolerance is not a physical machining clearance recommendation.

### Unsupported ShopBot home parking — ivac-dzsan

`park_xy` and tool-change stations were refused, but `park_at_home=true` was
accepted without implementing the requested machine-home motion. A dedicated
integration test reproduced this silent acceptance. The owned ShopBot post now
refuses that flag at preflight, with its existing unsupported-parking diagnostic.
No new machine command was invented; qualified ordinary jobs retain their output.
A direct CPS guard test also verifies refusal without partial output.

Pipeline version59→60 invalidates cached diagnostics/post results. The cache hash
snapshot is updated for this deliberate version change, not weakened.

## Qualification findings that need a decision

- Known fixture crossings return successful buffered results with critical warnings.
- UI export respects those warnings only when the existing user preference is on.
- Native raw streaming produced467 bytes and **zero fixture warnings** for the
  reproduced crossing. Preview-aware streaming reports the warning after bytes
  are already written. These tests characterize behavior; they are NOT passing
  safety acceptance tests. CPS direct streaming correctly refuses with zero bytes.
- A tool with only `shank_diameter_mm=6` has a computed body height0. Its high-clamp
  case produces no fixture warning or unknown-body warning. This is **unqualified
  metadata**, not a demonstrated clear setup. Complete-model classification remains
  to implement; no invented body dimensions were introduced in this checkpoint.

[ADR-0023](../shopbot-modernization/adr/ADR-0023-milling-program-publication-guard.md)
proposes a common non-bypassable publication guard with diagnostic preview,
explicit incomplete-model status and refusal of fixture-bearing direct byte streams
until a validated buffered program exists. This changes compatibility and needs
approval under the existing SAFE-Z contract's separate-review requirement for
stronger global prohibitions. The proposed guard is **not implemented** here.

## Z01–Z08 disposition

| Case | Evidence / remaining boundary |
|---|---|
| Z01 | Existing Profile/Pocket/Drill ShopBot regressions retained; no general cleared-stock proof. |
| Z02 | Source/fixture XY translation, unchanged Z, warning parity and cache mutation/restoration tested. |
| Z03–Z05 | Previous HATCH/known-stock preflight regressions rerun; invalid clearances refuse before output. |
| Z06 | Sloped holder defect corrected; box/cylinder analytic controls pass. Collision publication and partial metadata remain OPEN. |
| Z07 | Two tools with distinct5/8mm clearances decoded; below-top rapids are upward vertical continuations of previous cuts. Unsupported stations/home parking refuse. Unknown initial pose remains an operator prerequisite. |
| Z08 | Existing47 frontend lifecycle/fixture-policy tests pass; core cache fixture edits/restoration tested. Direct-stream publication parity remains OPEN. |

Supported generated two-tool tests prove a bounded flat-tool retract case, not
arbitrary material removal, holder/fixture avoidance at unknown initial XY, or
custom machine-frame post qualification. No physical machine was operated.

## Reproduce and evidence

```sh
SAFE_Z_EVIDENCE=/existing/empty/directory RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps,profiling --test safe_z_extended -- --test-threads=1
RUSTC_WRAPPER='' cargo test --offline -p ivac-cps --test shopbot_guards
```

Final-tree checks:12 focused tests (including4 shared oracle self-checks),
1204 core tests,14 HATCH/SAFE-Z regressions,27 ShopBot integrations,8 direct CPS
guards,16 HTTP tests,23 native WASM tests,47 focused frontend tests PASS.
TEST1 bounded replay:256/256PASS in28.35s, no timeout. Clippy, formatting and
`git diff --check` PASS. The first integration log has11 focused tests; the final
focused log adds the partial-body characterization for12. No browser/installation
or physical qualification is claimed.

[Evidence and SHA-256 manifests](safe-z-extended-2026-10-18-evidence/): failing
before logs, original diagnostic requests/responses, passing final logs, full
TEST1 archive and source hashes. Human review **ivac-6cske OPEN**; proposed
publication policy **ivac-h51gq OPEN**.
The shared oracle self-checks are included in the focused test count; they do not
inflate the number of independent pipeline qualification cases.

**HARD STOP:** review these corrections and approve/revise ADR-0023 before changing
the global publication policy. ivac-j09eh and overall TEST1 remain open.
