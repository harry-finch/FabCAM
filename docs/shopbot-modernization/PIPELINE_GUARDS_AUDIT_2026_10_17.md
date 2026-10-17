# Pipeline audit — lot 7 — 2026-10-17

Engineering tracking: ivac-qe2e2. Reviewed the independent report against the
owned post, pipeline and kernel contracts; severities are not accepted blindly.
Incoming working tree already contains lots 4–6 and workspace-cleanup records.
This lot neither republishes those changes nor claims physical qualification.

## Findings and disposition

| Report | Source assessment and bounded correction |
| --- | --- |
| Critical: unequal `MS` rejects ordinary ShopBot jobs | Not reproduced for the bundled post. `shopbot.cps::setFeed` deliberately writes both MS fields from each IR movement's scalar feed; cutting and plunging can have different successive MS settings. The decoder owns that closed output subset, not arbitrary OpenSBP. Keep rejection of unequal/modal MS and unknown motion; add a cut/plunge feed-switch regression. No new controller semantics are guessed. |
| High: unowned fixture movements skipped | Confirmed. Unknown/header/footer op IDs now check all tools used by enabled operations, including finish tools. Active-tool provenance is absent, so guessing the first or last operation is unsafe. Known operation IDs keep their existing tool selection. Collision radius is `max(nominal, effective)/2`: positive wear must not shrink the physical envelope, while negative wear can enlarge it. Existing unknown-holder conservatism remains. |
| High: RPM ceiling changes chipload | Clamping already emitted a warning; the feed consequence was missing. Preserve saved/manual feeds, add explicit unchanged-feed/material-per-revolution factor to the warning and EN/DE messages. Use the effective machine consistently for rough and finish clamps. That adapter currently changes only the spindle ceiling, so the feed-adapter inconsistency was latent, not a demonstrated different feed ceiling. |
| Medium: two-sided extents omit arcs and surfacing | Confirmed. Use arc-aware BBox, conservative swept-tool padding (two radii cover outside compensation plus cutter reach), explicit planar rectangle/margin/overtravel, and actual deepest face-relative scheduled depth. Both sides are authored in the same design frame: no extra mirror belongs in this guard. Reject two-sided heightfields and back-face planar surfacing until their footprint/depth/transform adapters are qualified. |
| Medium: automatic dowels fabricate geometry/rates | Confirmed. Reject invalid layouts instead of clamping margins or silently changing count. Require 2–1024 finite, contained, distinct, nonoverlapping holes (1024 is an allocation work cap, not a physical capability). Require exactly one matching declared Drill with valid tip geometry and explicit drilling RPM/feed/plunge. Reuse it unchanged rather than clone an endmill/laser or invent a 118-degree tip and cutting rates. ID overflow is refused. |
| Low: JSON remaining-material key | Confirmed allocation/formatting overhead. Replace full serde JSON with a bounded exact little-endian binary key: version, radius/depth, attributed targets and topology, relevant cut XYZ. Exclude unused line/op movement metadata and rapid/above-depth moves. Retain nonfinite bypass, exact equality, 8 MiB budget and existing bounded memo. A digest alone would weaken collision safety. Target operation identity is retained because returned regions carry it. |

## Regression invariants

- Low, known-position unowned rapids intersecting a declared clamp warn; distant
  paths do not. Positive and negative wear retain conservative cutter bounds.
- 24,000 to 12,000 RPM leaves authored 800/100 mm/min rates intact and reports
  the factor 2.000; no automatic FS Apply or persisted-value rewrite.
- A two-semicircle front source overlaps a back feature near its crest even
  though its chord endpoints do not; a front planar through-cut refuses even
  with a negative start depth. Synthetic back-face transforms remain refused.
- Dowel count/margin/spacing failures refuse before emitting either program;
  existing drill identity, tip and explicit drill rates survive injection.
- Binary keys distinguish geometry, topology and output attribution; irrelevant
  movement metadata does not invalidate the planar kernel's identity. Key size
  is checked against the prior JSON representation, without a timing claim.

## Validation

Final combined-tree software checks ([evidence and source hashes](pipeline-guards-audit-2026-10-17-evidence/checks.json)):

- Core debug library, `--features cps --lib -- --test-threads=1`: **1,145 PASS**, including all seven new lot-7 regressions and the final ID-zero ambiguity case.
- Release integrations: geom1_contract 23, geometry_source_ux 7, vcarve_qualification 10, shopbot_m2_post 27, cps_posts 4, drawing_audit 13, v1_stabilization 3: **87 reported PASS**. The optional FANUC oracle is absent and returns early: 86 checks actually execute.
- Frontend **1,324 PASS / 1 existing skip**, plus 10 WASM guard tests; Svelte/TypeScript zero diagnostics; lint/Prettier PASS.
- Final core Clippy `--features cps --lib -- -D warnings`, Rustfmt and whitespace checks PASS.

The first full release library run passed 1,144 and failed only the intentional
pipeline-version hash snapshot. The audited 52 → 53 discriminator changes its
expected key to `0x62217c70411c0abe`; the complete final serial rerun passes.
The initial frontend run exposed the new warning parameter missing from the
explicit test allowlist; adding the actual Rust-provided `chipload_factor` fixed
that contract check. Both initial failure logs are retained beside final evidence.

Pipeline discriminator advances 52 → 53 because warnings and guarded output can
change. Public application version remains beta.2 until a publication decision.
Engineering ivac-qe2e2 is complete; human review **ivac-rnf0l** remains open.

## Limits

Only known-position returned segments can be checked: the physical machine's
initial position is unknown and must not be invented by preview. Fixture bounds
are conservative diagnostics, not avoidance. Two-sided native code remains
outside the qualified ShopBot V1 scope; CPS two-sided refusal stays intact.
Remaining two-sided special-depth/lead/registration limitations are not a claim
of a complete material or holding-strength proof. No machine action, installer,
native/WASM production rebuild, commit, push, tag or release is part of this lot.
