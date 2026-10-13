# FS3.9 — Bounded normalized matching

Policy `fs39-normalized-matching-1`. Engineering implementation, human approval
pending. FS4 remains **DEFERRED / NONBLOCKING**, physical lab access unavailable.
See [coverage](FS3_9_COVERAGE.md) and [ADR-0011](../adr/ADR-0011-bounded-cutting-reference-adaptation.md).

## Audit and boundaries

FS3.7 already normalizes chipload, cutting speed, DOC/D and nominal radial ratio,
but requires all source scopes to match before ranking. The six public rows have
exact LongMill machine, composite stock identity, source tool identities,
3.175/6.35 diameter, one flute, full-slot operation and exact authored DOC.
Thus changing diameter alone cannot fix a generic softwood Pocket refusal.
The physical CAM tool model lacks helix/coating/geometry classification beyond
Endmill: unknown traits must not be inferred from names.

KEEP the legacy Engine and exact calculation/rounding/clamps. EXTEND the source
pack resolver with an explicit derived layer. No CAM geometry, postprocessor,
Tool/CuttingParameters ownership or generation semantics change. Legacy dataset
requests remain strict. Source packs without transfer declarations permit only
normalized diameter/DOC adaptation within their otherwise matching reference
scope. Tested/Validated/exact presets never broaden. The router transfer class cannot be an individual machine identity for a physical trial. Unsupported tool/process,
missing material, invalid input and absent engineering basis still refuse.

## Matching dimensions

| Dimension | Rule |
| --- | --- |
| Process | Milling only; CAM adapter rejects other processes |
| Mode | Same authored Prudent/Normal/Rapid; no hidden mode conversion |
| Tool family, flutes | Strict; no flat/compression/V/ball or 1F/2F mixing |
| Helix, shape, material, coating | Same confirmed source classification; extra tool IDs only in explicitly validated series |
| Physical traits unknown | No inference; operator selects compatible reference classification in Tool library |
| Material | Existing explicit taxonomy parent fallback; new source-category mappings list exact allowed taxon identities |
| Machine | Original applicability retained; new transfer only to declared catalog identities, visible ranking penalty |
| Operation/engagement | Strict except explicit FullSlot → nominal Pocket basis; no radial chip-thinning uplift; Profile requires full-width confirmation |
| DOC | Derived recommended ceiling, never silently expands requested depth; source-scaled DOC retained |
| Entry, thickness | Existing authored constraints remain strict; unknown source entry remains diagnostic |
| Source rights | Transformation permission required; bundled registry remains explicit |

The reviewed schema-3 pack declares softwood and MDF, **not all wood or all
plywood**, a selectable `machine/fablab-router@1` applicability context, its two
confirmed single-flute flat-upcut reference tool identities, and slot-to-pocket
transfer. This is FabCAM policy, not source-provided ShopBot or Pocket validation.
Selecting that router class is explicit; display names do not qualify unknown or
industrial machines. Selecting a reference tool attests compatible classification,
not a manufacturer SKU or identity of the physical cutter. Generic 2F remains
unsupported by these rows. Missing mechanical limits remain unknown.

## Diameter and normalized formulas

Audit found only two anchors separated by a factor of two. Relative distance is
`abs(Dsource − Drequest) / Drequest`, not absolute mm:

- ≤10%: Close reference;
- >10–25%: Approximate reference;
- >25–50%: Conservative extrapolation;
- >50%: reject direct adaptation; no fabricated generic/theoretical cutting data.

These are versioned eligibility thresholds, **not measured physical limits**.
Let `q = min(Drequest/Dsource, 1)`. Below 10% distance use `f=q`; beyond 10% use
`f=q²` for smaller cutters. This is a named conservative eligibility/adaptation
heuristic, not a stiffness/force model or assertion that slower feed is universally
safer. Reference RPM is retained toward smaller diameters; larger diameters cap
RPM to source cutting speed. Chipload and independent source plunge scale by f.
DOC = min(source DOC/D × requested D × additional q beyond 10%, source DOC,
requested DOC when supplied). Larger tools receive no chipload or DOC uplift.
Known machine/tool ceilings and existing rounding apply afterward. Effective
chipload is recomputed from final feed/RPM/flutes. User review of unknown limits,
entry, hold-down and chips is still necessary; no physical qualification claimed.

The public Ø6 Pocket example, DOC1, Prudent, compatible 1F upcut and explicit
router class gives 17000 rpm / 1530 feed / 765 plunge / 1 mm DOC. Its source remains
1620/810/2.8 at Ø6.35, immutable. Ø2 from synthetic Ø3 at 50% is conservatively
extrapolated; Ø2 from the actual Ø3.175 source is 58.75% away and refused.

## Interpolation

Prefer exact/close anchors, then justified interpolation, then approximate/distant
single anchors within the same material specificity. Interpolation requires an
accepted bracket in **one source pack/series**, identical material/machine/operation/
engagement/entry/thickness/family/flute/classification/mode/reduction context and
same evidence. Diameter span ≤2×; each anchor must first pass bounded adaptation.
A narrowest bracket with unique accepted anchor values is chosen deterministically per pack. Ambiguous anchor rows retain ordinary candidate reconciliation and disagreement instead of choosing by iteration order. No cross-vendor or
cross-material averaging. Across equally applicable packs retain existing
comparability/disagreement rules; incomparable contexts remain ambiguous.

`t=(D−Dlow)/(Dhigh−Dlow)`. Linearly interpolate chipload, cutting speed, DOC/D and
plunge/D; reconstruct RPM/feed/DOC/plunge for requested D, then apply known limits
and requested depth ceiling. Radial ratios must agree; nominal Pocket stepover is
retained from CAM, not attributed to the source. Both immutable original profiles,
IDs/versions, diameters, method, t and policy remain in derived provenance.

## Ranking, trace and fallbacks

Lexicographic: local exact trial priority; material specificity; original/derived
quality; machine-transfer penalty; original machine specificity; tool specificity;
diameter scope; flutes; DOC; entry; category; relative distance. Stable pack/record
ID order is the final tie-break. Prudent/Normal/Rapid still choose min/lower median/
max chipload among comparable same-mode equally ranked candidates. No raw-feed
average. A >25% chipload spread remains descriptive disagreement, not evidence
promotion. Match quality and evidence remain independent.

Parent traversal uses the declared taxonomy graph, rejecting invented string-prefix descendants. Taxonomy parent records keep their specificity penalty and Generic fallback label;
exact material outranks parent applicability. Theoretical fallback requires an
actual authored theoretical record. Neither family nor theoretical cutting values
are invented when public coverage is missing. Resolution trace `matching_trace`
returns sorted per-pack/record acceptance, selected contributors, relative diameter
distance, method and rejected scope dimensions. It is a pure diagnostic API, not
normal inspector content. Closest refusal scopes remain available in the UI. Failed derived eligibility omits transfer conditions already permitted by policy; Ø2/Ø3.175 exposes 58.75% versus the 50% ceiling rather than misleading machine/material refusals.

## Provenance, persistence and UX

The original profile is restored in recommendation provenance after computation.
Derived metadata is separate, contains source snapshots and adaptation parameters,
and is saved with public proposals. Knowledge summary stores policy, versions,
contributors and quality; **no private source quantities or bodies**. Private Apply
continues dropping full proposals. Reopen never invokes the engine, recalculates or
changes applied/manual values. Optional new fields deserialize old saves; source
schemas1/2 remain readable, schema3 requires the new loader. No new project format.

Compact inspector: source author/evidence, match label, source/request diameter,
explicit Apply and collapsed **Why these values?** with material/machine/operation
transfers and normalized method. Extrapolation uses restrained amber plus text.
Private saved projects retain quality but do not expose private source quantities.
Detailed licensing/original values remain under Source and provenance.
