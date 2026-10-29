# ADR-0032: Explicit Main Cutting and optional Finishing

Status: **Accepted — user approved v10/v2 boundaries; implementation in progress**
Date: 2026-10-28

Approval supersedes the preflight approval gate below. No further architecture approval is required unless a new incompatible or safety decision arises.

## Context

The ALPHA-3 product semantics are authorized. Their persistence policy is not yet
selected. The accepted ALPHA-2 descendant is `96181814c6f97ac19390dade7a0d3110b2f4d681`;
its executable software is `58ddde72295a01cbe43ce0270c98e1f79f2f1039`.
Profile requires Rough and Finish but emits the contour with Finish. Pocket's wall
tag and strategies are not an optional operation-wide finishing stage. Changing
these meanings changes machining, even when the JSON remains deserializable.

[The ALPHA-3 preflight](../../releases/ALPHA_3_MAIN_CUT_OPTIONAL_FINISHING.md)
contains actual old-reader tests, deliberately failing product baselines and
the proposed execution/qualification plan. No production implementation is present.

## Proposed decision

- Author new portable projects as **v10**, and generation requests with
  **`project.cutting_version: 10`**. Advertise a distinct cutting-v10 capability
  in native/HTTP/WASM/worker transports. Reject mismatched capabilities and
  versions without retrying through a v9 adapter.
- Profile and ordinary Pocket use an explicit, typed Main/Finishing contract.
  Main is mandatory, Finishing is an optional configuration, absent by default.
  Do not keep legacy Rough/Finish values as a fallback for these operations.
- Keep one authoritative cutter assignment and explicit rate/schedule ownership
  per stage. Any redundant wire assignment required by common operation ordering
  must be checked for equality, not resolved by precedence. The exact generated
  schema will be reviewed with the core implementation.
- Keep non-Profile/Pocket families' existing cutting meanings and output. The v10
  envelope does not authorize changing Drill, Halfpipe, specialized CAM or posts.
- Keep drawing primitives/identity v1, ALPHA-2 diagnostic/snapshot v2 and drawing
  generation envelope v1. Its nested machining request uses cutting v10; the
  geometry snapshot is never a machining certificate.
- Retain stock for a later operation separately from allowances removed by this
  operation's Finishing. No hidden allowance when finishing is absent.
- Use existing geometry kernels, common safe entry/emission and actual toolchange
  envelopes. Add stage planning/attribution, not a parallel postprocessor or a
  general new manufacturing IR. Never fake a ShopBot tool boundary.

### Existing files and collections

The minimal proposed policy is **explicit refusal of old v9 machining projects in
the new editor**, before replacing current state or writing any file. Preserve
their bytes and the previous ALPHA-2 reader for inspection. The user recreates or
explicitly reconfigures a new v10 project; no automatic Finish-to-Main copy and no
automatic finishing activation. A mixed editable v9/v10 operation document would
require an additional migration state machine and is not proposed here.

New presets carrying the stage contract need an explicit **collection v2** boundary.
Keep v1 collections readable and unmodified; unchanged families may retain exact
legacy-context matching. A v1 Profile/Pocket preset is historical and inapplicable
to new Main/Finishing operations. Writing new staged entries uses a new v2 collection,
not silent replacement of a v1 store. Existing identities/notes/attestations are
preserved as history, never transferred as proof of the new context. Context
signatures must include the stage contract, finishing state and relevant strategies,
tools and allowance semantics. ALPHA-4 reuse remains excluded.

## Alternatives considered

| Alternative | Evidence and disposition |
|---|---|
| Add operation-level flags to v9, retaining old rates | Unsafe: old native and frontend readers accept them; native generation ignores the flag and emits the old program. |
| Add a discriminator beside v9 `rates` | Native JSON/frontend reject it, but real WASM ignores it and emits the old program. Not a sufficient guard. |
| Replace the required v9 cutting shape or use an unknown mandatory Main role | The tested native/frontend/WASM boundaries reject this. This is a credible guarded-v9 alternative, **not proven impossible**. It still breaks the existing cutting shape/capability and introduces two meanings beneath the v9 label. |
| Portable v10 plus cutting v10 | Recommended: explicit rejection already exists at the old outer reader and native generation boundaries. Clear document/engine capability, independent of ignored optional fields. |
| Preserve legacy behavior through automatic conversion | Rejected: the user waived old machining semantics, not file preservation; copying Finish to Main would invent an equivalence. |

The version increment is a recommendation, not a claim that every guarded v9
design is unsafe. ALPHA-3 §7 requires approval before implementing an increment.
This proposal therefore stops before persistence or production changes.

## Consequences

Old documents/collections remain on disk, but v9 projects need a new configuration
to machine in ALPHA-3. New readers, schemas, presets and capability checks must
agree on the contract. Main and Finish must be identifiable on emitted movements,
including same-tool stages. Simulation must consume actual attributed movements;
labels or intended allowances cannot prove material removal. Unaffected CNC
output stays byte-exact; intentional Profile/Pocket differences get separate
reviewed geometry/motion evidence.

## Validation

Already exercised: outer v10 restore rejects before state mutation; native and
real-WASM cutting-v10 generation refuse; WASM streaming emits no events; additive
field counterexamples; new-role/replacement-shape rejection; Profile rate mismatch,
retained stock, axial schedule, four Pocket strategies and current ShopBot refusal.

After approval: new-reader round trips; old-reader rejection of actual new files,
not just probes; explicit old-file preservation; new preset collection rejection
by old readers; full native/WASM/CLI/HTTP capability matrix; stage swept geometry,
floor coverage, cutter assignment, tabs/leads, limits, cancellation, Undo/Redo,
freshness, browser FR/EN review and canonical release gates. Installed-native and
physical acceptance remain separate. No implementation commits before final review.
