# FS1 — Pure recommendation engine MVP

> Historical evidence: current policy-2 corrections and qualification are recorded in [FS audit improvements](FS_AUDIT_IMPROVEMENTS.md). Existing fixtures below describe their original checkpoint.

Date: 2026-10-12. Status: engineering handoff; human approval pending.
Engineering: `ivac-86b8`; human approval gate: `ivac-406n`.
User approved FS0 and authorized FS1 (“je valide, passe à
la suite”). Source base remains published `f6d17dd`; FS0 documentation and FS1
implementation are uncommitted. No new publication was authorized.

## Implemented boundary

`crates/ivac-feeds-speeds` is a workspace/default member with Serde as its only
normal dependency and serde_json only for tests. No dependency on ivac-core,
geometry, UI, CPS, machine posts, filesystem, clocks or network. No production
cutting data is bundled. Existing CAM/public models, generated OpenAPI/TypeScript,
project files, app and ShopBot post are unchanged.

- `src/model.rs`: copied context, scoped complete profiles, explicit family
  membership, typed available/unavailable results and provenance.
- `src/validation.rs`: validates request and supplied profile set; missing
  values remain unknown, malformed data cannot bypass the Engine constructor.
- `src/matching.rs`: eligibility, tiers and strict scope containment.
- `src/engine.rs`: deterministic selection/calculation, constraints and traces.
- `tests/qualification.rs`: original synthetic arithmetic/context fixtures,
  including a material ID labelled plywood for FS-01. No physical data claim.

`Engine::new(ProfileSet)` owns a validated immutable snapshot;
`recommend(&FeedsSpeedsRequest)` returns a proposal without mutating inputs.
`ProfileSet` is a versioned in-memory domain contract with Serde round-trips;
FS2 distribution, catalogs, authoring, flattening/inheritance and file loaders
are not implemented. Callers supply complete profiles. Domain schema version 1
and `0.5.0/fs1-policy-1` are independent of immutable dataset/profile revisions.
The engine does not create `ResolvedCuttingParameters` or Apply/save behavior;
those CAM-owned responsibilities remain FS3.

## Qualified calculation policy

Profiles supply their own mode/operation-specific RPM, chipload or exact feed,
plunge, DOC and applicable stepover. Material data never lives in algorithm code.
No implicit mode factor, universal plunge/feed ratio or theoretical constants.
An authored theoretical profile can be selected; absent useful data is Unavailable.

1. Validate positive finite quantities, ratios in (0,1], positive optional limits,
   domain version and nonempty immutable IDs/revisions. Contradictory RPM limits,
   including tool maximum below machine minimum, refuse before selection.
2. Apply only ordered, sourced, named reduction rules in (0,1] within their
   enclosing profile scope. Exact presets do not accept reduction rules.
3. DOC is capped to known requested DOC. Nominal stepover is capped to known
   requested ratio. These adjustments remain visible; no silently increased
   existing depth/nominal lateral engagement.
4. Select RPM from supplied target, round to nearest positive integer with
   half-ties upward, then apply machine upper bound, tool upper bound and machine
   lower bound. Unknown limits are explicitly diagnosed, not fabricated.
5. ChiploadTarget data calculates feed using final RPM × flutes × corrected
   chipload. ExactValues data keeps supplied feed when RPM changes. This distinction
   is intentional and tested; effective chipload always reflects final output.
6. Clamp XY feed/plunge independently to supplied ceilings, then floor to integer
   mm/min. Preserve floating precision for DOC/ratios. Zero, overflow, nonfinite
   or unrepresentable output yields Unavailable rather than saturation/wrapping
   or zero-feed success. Nonfinite arithmetic is rejected before clamps.
7. Report effective chipload = final feed / (final RPM × flute count). Every
   actual clamp/reduction/rounding/context cap records field, reason, before,
   after and rule ID/revision when applicable.

Source evidence remains Theoretical/ReferenceData/Tested/Validated, independent
of Prudent/Normal/Rapid mode. Any actual adjustment gives
`AdjustedOutsideEvidence`, even rounding or a conservative reduction. FS1 does
not implement transformed-value validation envelopes: this is deliberately
stricter than the future option to qualify those transformations. Unmodified
output reports WithinDeclaredProfile, which means within the **declared** source
scope, not an independent certification of physical evidence. Trial/criteria
reference validation checks structural completeness only.

No profile-specific hard limit beyond supplied scope/targets and request machine/
tool constraints is invented. Dataset authors must distinguish physical maximum
constraints from preferred targets; FS2 must qualify any additional data-model
constraints before providing usable production profiles. Clamped RPM is a changed
operating point, visibly outside source evidence; no machine cutting capability
is inferred from successful arithmetic.

## Matching and operation policy

Eligibility ANDs all declared selectors. Exact identities include revision;
family selectors use explicitly authored memberships; missing facts do not match
constrained fields. Material scope is required even for generic reference data.
Ranges are inclusive. Multiple active revisions of one profile ID are rejected.

Tiers are exact Validated → exact Tested → machine/family ReferenceData → general
ReferenceData → authored Theoretical. Within a tier, strict applicability-scope
containment wins; equal or incomparable maximal candidates return AmbiguousProfiles
with sorted IDs. A singleton family and equivalent exact identity are equally
specific, not an arbitrary preference. Ambiguity never falls through to lower
tiers. Family membership ordering and candidate file order do not select a winner.

Only flat end mills support initial recommendations. Profile/Pocket/FullSlot/Finish
each require their own data. FullSlot requires explicit KnownFullWidth; a nominal
ratio (even 1) is not silently promoted to that physical assertion. Pocket nominal
ratio remains estimated, with a diagnostic that instantaneous maximum is unknown.
Unknown engagement matches only UnknownSafe data authored for the conservative
full-width assumption; it cannot use low-radial-only data. Finish context cannot
reuse rough Profile data. Compression/V tools/other families, Surfacing/Engraving
and non-milling contexts remain unsupported; non-milling operation/process tags
are structurally rejected instead of interpreted as milling.

Exact tested/validated presets require exact machine/material/tool identity,
flutes, diameter/DOC applicability, known entry/engagement, complete values and
trial reference; Validated also requires criteria reference. Recorded thickness
qualifiers cannot become wildcards on unknown thickness. Broad tested/validated
profile evidence is intentionally unsupported in this MVP; later acceptance
needs a qualified range protocol, not relaxing these checks silently.

## Validation and limits

Final targeted suite: **38 qualification tests plus one compiled API doctest**.
They cover FS-01/02, engine portion of FS-03, FS-04–09, engine serialization/data
portion of FS-13, FS-15/16. The invariant grid exercises 36 machine-limit contexts
and confirms positive output, bounds and final chipload. Range endpoints,
family containment/equivalence, ambiguous candidate reordering, exact preset
scope changes, source/rule errors, rounding/overflow and input immutability pass.

FS-10/11/12/14, CAM overlap conversion and schema/transport/save/manual-override
integration remain FS3 requirements. Engine purity is not proof of project
persistence. No new physical qualification or production material setting.

Executed on this macOS arm64 working tree:

| Command | Result |
| --- | --- |
| `RUSTC_WRAPPER='' cargo test -p ivac-feeds-speeds --locked --offline` | Final 38 qualification tests + 1 doctest pass |
| `RUSTC_WRAPPER='' cargo clippy -p ivac-feeds-speeds --all-targets --locked --offline -- -D warnings` | Pass |
| `RUSTC_WRAPPER='' cargo check -p ivac-feeds-speeds --target wasm32-unknown-unknown --locked --offline` | Pass; compilation, not browser runtime qualification |
| `cargo fmt --all -- --check` | Pass |
| `RUSTC_WRAPPER='' cargo run -p xtask --locked --offline -- schema-check` | Pass; existing schema unchanged |
| `RUSTC_WRAPPER='' cargo tree -p ivac-feeds-speeds --edges normal --locked --offline` | Serde tree only; no core/CPS dependency |
| `RUSTC_WRAPPER='' cargo test --workspace --exclude ivac-tauri --all-features --locked --offline -- --test-threads=1` | Pass; exit 0, no failing suites |
| `git diff --check` and Python changed-path/relative-Markdown-link/whitespace check | Pass, including untracked crate/documents |

The workspace run includes the first 34 FS tests; four additional matching/
authoring/provenance cases and the API doctest were subsequently verified in the final
targeted suite, without further arithmetic/matching changes. Serial execution avoids the
known pre-existing global pocket-cache test race `ivac-htc8`. No frontend or
desktop rebuild is necessary for this isolated library change. No Linux/Windows
or controller run is claimed.

## Handoff

FS1 stops at human review. Review this policy, crate API, source and synthetic
test evidence before authorizing FS2. FS2 must supply legally sourced fablab
cutting profiles and qualify the distribution/catalog/loading/authoring contract;
it must not reinterpret synthetic fixtures as usable cutting data. FS3 will
implement accepted persistence/adapter/consumer changes. No FS2 work, new commit,
push, PR, merge or upstream synchronization has been performed.


## Subsequent approval

User approved FS1 on 2026-10-12 (“ok alors je valide, on peut passer à la suite”)
and authorized FS2. ivac-406n closed. The preceding handoff records FS1's original
scope/results; see FS2 qualification for the subsequent loader/data delta.
