# ivac-feeds-speeds

Pure, deterministic milling recommendations from caller-supplied scoped profiles.
FS2 adds an explicit JSON loader and separate external reference data. There is
no UI, geometry, CAM adapter, project migration or post integration. Normal
dependencies: existing Serde and serde_json versions; no I/O/network dependency.

`Engine::new(ProfileSet)` validates data and owns an immutable snapshot.
`Engine::recommend(&FeedsSpeedsRequest)` returns Available or an explicit
Unavailable reason without changing the request or reading external state.
Recommendations are proposals; persistence/application belongs to FS3.

```rust
use ivac_feeds_speeds::{Engine, FeedsSpeedsRequest, FeedsSpeedsRecommendation,
                       ProfileSet, ValidationIssue};

fn propose(data: ProfileSet, request: &FeedsSpeedsRequest)
    -> Result<FeedsSpeedsRecommendation, Vec<ValidationIssue>>
{
    let engine = Engine::new(data)?;
    Ok(engine.recommend(request))
}
```

Only flat end mills and Profile/Pocket/FullSlot/Finish are initially supported.
Full slots require explicit full-width context; nominal pocket engagement does
not describe its instantaneous maximum. Unknown engagement matches only an
explicitly authored unknown-safe profile. Missing context never satisfies a
constrained selector. Other tools/operations are explicitly unsupported.

Selection: exact Validated → exact Tested → machine/family → ReferenceData →
authored Theoretical. Within a tier, strict applicability-scope containment wins;
equal or incomparable maximal scopes are ambiguous. No fallback values or
automatic aggressiveness multiplier exist. Family membership is explicit.

RPM starts from the selected data target, rounds to nearest integer (positive
half-ties upward), then respects known machine and tool bounds. Chipload-target
profiles calculate feed at the resulting RPM. Exact-value data reduces feed proportionally when RPM falls (including rounding),
never raising its implicit chipload; it does not raise feed when RPM increases. Feed and plunge clamp independently and round down to integer
mm/min; effective chipload is derived from the resulting feed/RPM/flutes.
DOC/stepover never silently increase known requested DOC/nominal stepover.
Only named, sourced reductions in (0,1] are supported. Nonfinite, zero or
unrepresentable output is unavailable; no zero-feed success or wrapping cast.

Source evidence and aggressiveness are independent. Any actual adjustment,
including rounding, gives `AdjustedOutsideEvidence`; it does not promote or
silently retain a validated-result claim. Provenance retains the selected profile,
request, identities/revisions, selected family memberships, policy version and
before/after adjustments.
Trial/criteria reference checks establish structural completeness only: the
caller remains responsible for legitimate evidence and profile revisions.

`load_dataset_json(&contents)` validates a format-1 `DatasetDocument` and returns
an immutable `LoadedDataset` with `document()` and `engine()` accessors. The host
owns I/O and dataset selection. All invalid rows reject the whole document;
unknown fields/versions, dangling catalog references and contradictory tool
geometry/maximum RPM are refused. Complete profiles only; no implicit inheritance.
See [data format, source attribution and scope](../../data/feeds-speeds/README.md).
The separate six-profile dataset is LongMill external ReferenceData, full-slot,
single-flute upcut only. It never matches a ShopBot or lab stock/tool automatically.

Tests in `tests/qualification.rs` use original synthetic software data only.
Run `RUSTC_WRAPPER='' cargo test -p ivac-feeds-speeds --locked --offline`.
See [FS1 qualification](../../docs/shopbot-modernization/feeds-speeds/FS1_QUALIFICATION.md).

See [FS2 qualification](../../docs/shopbot-modernization/feeds-speeds/FS2_QUALIFICATION.md).

FS3.9 extends source-pack resolution with bounded normalized reference adaptations,
immutable source snapshots, explicit transfer scopes, graph taxonomy and sorted
matching traces. Historical FS3.9 kept exact mathematics unchanged; the audited policy-2 update below revises RPM/feed coupling. See
`docs/shopbot-modernization/feeds-speeds/FS3_9_MATCHING.md` and the reproducible
`coverage` example. Schema3 is additive; physically supported exact presets never
inherit diameter/class transfers. No production cutting dataset is embedded here.

## Audited policy 2 (software qualification only)

Engine provenance now reports `fs1-policy-2`. Previously saved cutting values and
snapshots remain authoritative; only explicit new recommendations use this policy.
Generic Direct entry emits a suitability warning. A machine feed ceiling emits
`LowEffectiveChipload` relative to the selected target, not an invented universal
material threshold. RPM is not automatically retuned; plunge strategy/geometry is
not invented.

`validate_dataset(&document)` shares validation with JSON loading. Structured
`*_checked` pack/trace APIs expose `SourcePackError`; legacy String-returning APIs
remain compatible. CandidateRank is typed and Copy in Rust, while its serialized
12-cost array and OpenAPI/TS representation stay compatible.

For one adapted anchor, let delta = abs(source − requested)/requested and
q = min(requested/source, 1). Set t = clamp((delta − 0.1)/0.4, 0, 1):
chipload/plunge factor = q^(1+t), DOC = min(source DOC, normalized DOC × requested × q^t).
Larger cutters retain chipload/DOC ceilings and RPM cutting-speed cap. This smooths
the old 10% step but can be less restrictive than the previous squared factor above
10%; it is a documented heuristic, not new physical evidence. Selection/eligibility
boundaries can still jump. Explicit quality order: Exact, Close, Interpolated,
Approximate, Extrapolated, Generic, Theoretical. Interpolation requires matching rank
context and declared adaptation scope in both anchors.

Comparable numeric DOC/stepover/authored-RPM calculations tolerate absolute 1e-9
roundoff in native units; scopes, identity, integer output RPM and hard limits remain
strict. Trace and recommendation share one candidate pass; taxonomy traversal uses
static borrowed entries. Pack size is still checked by streaming serialization,
without allocating a full JSON copy or reparsing datasets. No measured speedup claimed.

### Follow-up policy-2 qualification

The uncommitted policy-2 checkpoint also scales absolute plunge down by the final
RPM/source-RPM ratio for both data modes (never upward). This preserves axial
advance per revolution, not a universal physical plunge chipload model. ExactValues
with an RPM floor now reports low effective chipload against the authored target,
including when a feed ceiling also applies.
Adaptation material scopes include known taxonomy descendants; transfer ranking uses
the nearest permitted scope plus one transfer cost. This does not establish evidence
for unlisted sibling taxa. Tool classifications are indexed once per validation;
missing classifications are ignored and contradictory known classifications rejected.
