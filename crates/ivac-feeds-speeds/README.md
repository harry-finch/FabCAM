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
profiles calculate feed at the resulting RPM. Exact-value data retains its
supplied feed. Feed and plunge clamp independently and round down to integer
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
matching traces. Legacy Engine exact mathematics remain unchanged. See
`docs/shopbot-modernization/feeds-speeds/FS3_9_MATCHING.md` and the reproducible
`coverage` example. Schema3 is additive; physically supported exact presets never
inherit diameter/class transfers. No production cutting dataset is embedded here.
