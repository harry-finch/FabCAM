# FS3.9 — Public coverage audit

Generated with `cargo run -p ivac-feeds-speeds --example coverage`.
Pack: `sienci-router-reference` @ `2026-10-13.1`; policy: `fs39-normalized-matching-1`. Six original records, no new cutting rows.

## Controlled audit context

Explicit fablab-router class, confirmed flat upcut 1F source series, Prudent, nominal Pocket 50%, DOC 1 mm, direct entry (source entry unknown), no invented hard limits. This is derived coverage, not physical validation. Quality labels describe diameter adaptation; material/machine/full-slot transfers remain separately disclosed.

C = close normalized derivation; A = approximate; I = interpolation; E = conservative extrapolation; — = missing.

| Material / diameter mm | 1 | 2 | 3 | 3.175 | 4 | 5 | 6 | 6.35 | 8 | 10 | 12 | 12.7 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| MDF | — | — | C | C | A | I | C | C | A | E | E | E |
| Plywood | — | — | — | — | — | — | — | — | — | — | — | — |
| Poplar plywood | — | — | — | — | — | — | — | — | — | — | — | — |
| Birch plywood | — | — | — | — | — | — | — | — | — | — | — | — |
| Hardwood | — | — | — | — | — | — | — | — | — | — | — | — |
| Softwood | — | — | C | C | A | I | C | C | A | E | E | E |
| Cast PMMA | — | — | — | — | — | — | — | — | — | — | — | — |
| Extruded PMMA | — | — | — | — | — | — | — | — | — | — | — | — |
| POM | — | — | — | — | — | — | — | — | — | — | — | — |
| HDPE | — | — | — | — | — | — | — | — | — | — | — | — |
| Polycarbonate | — | — | — | — | — | — | — | — | — | — | — | — |
| PVC | — | — | — | — | — | — | — | — | — | — | — | — |
| Aluminium 6061 | — | — | — | — | — | — | — | — | — | — | — | — |
| Aluminium 7075 | — | — | — | — | — | — | — | — | — | — | — | — |

Generic aluminium (including 6061/7075) has no data. Plywood, birch and poplar are not equated to the source's **soft plywood** category. Cast/extruded PMMA, POM, HDPE, polycarbonate and PVC have no data. No generic wood, plastic, metal or theoretical fallback records are bundled.

## Exact source coverage

| Material scope | Tool | Diameter | Operation | Machine | Modes |
| --- | --- | --- | --- | --- | --- |
| Source softwood / soft plywood / MDF category | Flat upcut 1F | 3.175, 6.35 | Full slot | LongMill reference identity | Prudent / Normal / Rapid |

These six exact combinations retain original RPM/feed/plunge/DOC values. Exact diameter alone does not make a transferred Pocket/ShopBot/material context an exact reference.

## Operation and physical-family coverage

| Context | Coverage |
| --- | --- |
| Profile with explicit full-width confirmation | Full-slot basis, bounded diameter adaptation |
| Profile without known engagement | Missing |
| Pocket with nominal engagement | Explicit full-slot-derived policy, no chip-thinning uplift |
| Full slot | Supported within material/tool/machine policy |
| Finishing | Missing; no silent slot-to-finish transfer |
| Surfacing | Unsupported engine operation; missing dedicated data |
| Flat upcut 2F | Missing for every audited diameter |
| Compression, downcut, ball, V-bit | Missing / incompatible |
| Unknown machine or physical tool traits | Refused; explicit applicability selection required |
| Industrial router | No automatic fablab-class transfer |

## Priorities for later work

P0: real ShopBot softwood/MDF Pocket Ø6 and Ø3, distinguish 1F and 2F, DOC/WOC/entry and tool identity. P1: plywood/birch/poplar and common plastics with approved rights. P2: small-tool Ø1/Ø2, aluminium and dedicated surfacing cutters. Physical testing remains FS4 **DEFERRED, NONBLOCKING**. Public growth: first review additional rows from the already licensed pinned Sienci resource; other vendors require data-specific rights review. No proprietary tables imported.
