# FS3.9 — Software qualification and human handoff

Engineering issue ivac-02in; human review **ivac-t0fk** pending. No commit/push,
physical machining or FS4 evidence promotion. FS4 **DEFERRED / NONBLOCKING**.
Milling CAM4 human gate ivac-tvox remains independent and pending.

## Deliverables

- [Matching policy and formulas](FS3_9_MATCHING.md), [coverage matrix](FS3_9_COVERAGE.md),
  [ADR-0011](../adr/ADR-0011-bounded-cutting-reference-adaptation.md).
- Pure `ivac-feeds-speeds/src/adaptation.rs`, pack resolver and deterministic trace.
- Reviewed public schema3 pack revision 2026-10-13.1; six original profile bodies
  unchanged, no extra cutting rows. CC BY-SA attribution/permissions retained;
  schema1/2, legacy strict Engine and private storage boundaries retained.
- Compact RecommendationBasis inspector, explicit source author, diameter quality,
  visible extrapolation and collapsed Why these values?; Apply remains explicit.
- Optional public derivation provenance keeps both interpolation source snapshots;
  private Apply still drops proposal bodies/quantities. Individual machine identity
  is required for physical evidence; the router transfer class cannot be Tested.

## Automated evidence

Commands run from the repository, using CARGO_TARGET_DIR=/private/tmp/fabcam-cps-runtime,
CARGO_INCREMENTAL=0, RUSTC_WRAPPER='' and dev/test debug0; Node24 in PATH.

| Check | Result |
| --- | --- |
| `cargo test -p ivac-feeds-speeds --offline` | 94 tests +1 doctest pass, including 17 FS3.9 tests |
| `cargo test -p ivac-core --lib feeds_speeds --offline -- --test-threads=1` | 17 adapter/persistence/privacy tests pass |
| `cargo clippy -p ivac-feeds-speeds --all-targets --offline -- -D warnings` | Pass |
| `cargo fmt --all -- --check` | Pass |
| `cargo run -p xtask --offline -- schema-check` | Pass; generated TypeScript refreshed |
| Frontend `npm run check` | Zero errors/warnings |
| Frontend `npm run lint` | ESLint/Prettier pass |
| Frontend `npm test` | 1112 pass, 1 optional skip, 120 files |
| Frontend `npm run build` | Optimized WASM rebuilt and frontend build passes |
| `cargo build -p ivac-server --offline` | CPS-enabled review server rebuilt |

Build retains the existing Tauri static/dynamic import warning; no desktop bundle
or blanket milling/post requalification claimed. Checks are scoped to FS behavior.
The server is restarted on the existing 18772/CORS18767 review endpoint because
older binaries would reject the new public provenance field.

Coverage tests establish exact six-row numeric parity, normalized Ø6/Ø6.35 Pocket,
relative rather than absolute distance, conservative smaller and larger directions,
Ø2/Ø3 synthetic and actual Ø2/Ø3.175 refusal, two-anchor interpolation, strict tool/
flutes/operation, explicit machine penalty, declared taxonomy graph/parent priority,
deterministic ordering and trace, explainable no-match, disagreement across sources
and ambiguous anchors, no broadened physical evidence and immutable supplied data.
Core tests prove preview does not change the operation, Apply is explicit, manual
rates reopen unchanged and private derived source snapshots are not saved.

## Browser evidence

Actual localhost browser, bundled schema3 pack and locally executed WASM solver:

- Softwood, explicitly configured router applicability, confirmed upcut1F Ø6,
  nominal Pocket50%, DOC1, Prudent → 17000rpm /1530feed /765plunge /1mm.
  Source Sienci Labs /ReferenceData visible, 5.8% diameter gap and machine/material/
  full-slot basis disclosed; input rates remain unchanged before Apply.
- [Expanded explanation](FS3_9-near.png), [compact source](FS3_9-near-compact.png).
- Explicit Apply, manual feed1200, Save, reopen → feed1200 remains Modified;
  file version4 retains `bounded_normalized_reference` and Close quality. No
  recalculation on reopen and no console errors during that flow.
- Ø2.2 from Ø3.175 → 22000rpm /504feed /254plunge /DOC1, ReferenceData and
  Conservative extrapolation. [View](FS3_9-extrapolated.png).

- Ø5 interpolation → 18350rpm /1377feed /721plunge /DOC1, two original
  sources Ø3.175/Ø6.35 retained. [View](FS3_9-interpolated.png).
- Ø2 refusal explains the actual 58.75% gap from Ø3.175 against the 50% policy
  boundary. [View](FS3_9-no-match.png).
- Final resumed frontend check/lint/build and 120 test files passed (1112 tests,
  one optional skip). Native browser Ø6 recalculation reproduced the values above.
  [Final panel](FS3_9-final.png).

## Independent ShopBot export limitation

Native CPS accepts the additive provenance payload, but the test with rough17000rpm
and retained finish10000rpm fails on unsupported `spindleSpeed` in ShopBot preflight.
Tracked separately as **ivac-2zll**; [error evidence](FS3_9-shopbot-rpm-limit.png).
FS3.9 does not fix this post behavior or silently align independent finishing values.
For a software-only export check, explicitly setting finish17000rpm produced
77 moves /133 lines. The output retained manual roughfeed1200 (`MS,20.000000`)
and plunge765 (`MS,12.750000`), proving consumption of resolved values.
This conditional success does not qualify arbitrary RPM transitions or physical cuts.

## Human review

Open [the software-only fixture](FS3_9-software-pocket.ivac-project.json) in the
current browser build after reload. It declares a compatible router context and
1F flat-upcut reference classification; it is not a real Sorbonne tool inventory.
Calculate, review source/Why these values?, Apply only deliberately, edit/save/reopen.
For actual tools, confirm family/helix/shape/flutes in Tool library compatibility;
select router applicability in Machine settings only when appropriate. Generic2F,
compression, unknown class and unsupported material/operation must still refuse.
For later actual local trials use individual machine/tool identities; class-level
reference transfers are not physical trial scope.

Review thresholds and heuristic q/q² conservatism as software policy, not measured
stiffness or certified cutting limits. Known hard limits remain enforced and unknown
limits/entry remain unknown. No source rows are invented or reclassified Tested.

Priority gaps: real ShopBot Pocket Ø6/Ø3 in softwood/MDF with **separate 1F/2F**;
then plywood/birch/poplar, plastics and dedicated surfacing data with appropriate
rights. Ø1/Ø2 and aluminium remain later data/validation priorities. First future
public-source candidate is additional individually reviewed rows of the already
licensed pinned Sienci resource. Other vendors require data-specific rights review;
no proprietary tables imported or private Sorbonne pack populated.

**STOP at human approval; no further FS milestone or physical FS4 started.**
