# M3 — loadable Sorbonne ShopBot profile

Software deliverable approved; committed and pushed at `b8a073a`.
Based on approved/pushed M2 `047a75872ca7ea7d28077555c9859f92fa9a7411`.

## Deliverables

- [Native profile](../../machines/sorbonne-shopbot-prs.ivac-machine.json), using the existing version-1 machine snapshot format.
- [Loading instructions](../../machines/README.md).
- [Machine record](shopbot/SORBONNE_MACHINE.md), with provenance for reported 120 × 60 cm, 2011, SB3.8 and every remaining parameter explicitly **TO VERIFY ON MACHINE**.
- Integration coverage of the actual machine import/export functions with desktop I/O/store doubles, workspace profile round trip, CPS request adapter and wire clearance propagation.
- Rust import/CAM/post test confirms that the snapshot's zero clearance sentinel refuses SBP export. No production CAM, post, UI, API or schema change.

## Acceptance evidence

| Criterion | Result |
| --- | --- |
| Profile loads | Existing `loadMachine` accepts the checked-in file; `saveMachine` and reload preserve its fields |
| Known facts documented | User-reported nominal XY/year/software recorded with source and conversion; no generic catalogue values substituted |
| Unknown physical values explicit | Machine record covers XYZ, jog/feed/plunge, spindle/RPM, tools, clearance, park/zero, arcs and controller; unknowns marked exactly TO VERIFY ON MACHINE |
| No arbitrary authoritative safety values | No physical speed/RPM/clearance recommendation; zero clearance/Z are documented sentinels; software defaults are identified as unqualified |

Positive clearance must be authored before export. This is an export guard, not
a complete physical qualification gate; replacing zero does not certify the
remaining unknowns. Nominal XY is not an enforced controller travel boundary.
Tests use synthetic tooling, never a recommendation for the lab machine.

## Validation

Executed with Node 24.21.0, pnpm 10.34.6 and Rust 1.88.0 on macOS:

```sh
cd frontend
pnpm exec vitest run src/lib/services/shopbot-machine-load.test.ts
pnpm check
cd ..
cargo test -p ivac-core --features cps --locked --test shopbot_m2_post m3_unverified_clearance_refuses_sbp_export -- --test-threads=1
cargo fmt --all --check
cargo clippy -p ivac-core --features cps --test shopbot_m2_post --locked -- -D warnings
git diff --check
```

Import/round-trip integration: **1 pass**. CAM-to-post refusal: **1 pass**.
Frontend type checks: **zero errors/warnings**. Rust formatting and targeted
Clippy pass; frontend test file ESLint and Prettier pass; Git whitespace check passes.
M2's broader regression/build qualification remains the software baseline.
Desktop filesystem/dialog calls are mocked: no live native picker or Tauri bundle
was exercised. No SB3.8 parser, Preview Mode, VCarve comparison or hardware test.

## Handoff

M2 and M3 approved and pushed. M4 subsequently authorized; see M4_VALIDATION.
Beads task `ivac-c5lt` closed after software qualification.
Physical measurements and operational details belong to the unverified machine
record. The original M3 stop was satisfied by explicit user approval.
