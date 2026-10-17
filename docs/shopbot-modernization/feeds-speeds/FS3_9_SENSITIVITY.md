# FS3.9 — Diameter sensitivity measurements

> Historical evidence: current policy-2 corrections and qualification are recorded in [FS audit improvements](FS_AUDIT_IMPROVEMENTS.md). Existing fixtures below describe their original checkpoint.

Software characterization only; source pack `sienci-router-reference` revision `2026-10-13.1`, policy `fs39-normalized-matching-1`. No machining validation.

11997 samples: three modes × three scenarios, 1–14 mm at 0.01 mm plus source anchors and ±0.000001 mm around relative 10/25/50% boundaries.

Context: softwood, explicit router-class transfer, confirmed flat upcut 1F, Pocket nominal50%, direct entry. doc1/doc10: requested DOC ceiling1/10 mm, no supplied machine limits. limited: DOC10, synthetic caps16000rpm/700feed/300plunge (not ShopBot specifications). All accepted results remain ReferenceData.

Tables include every observed availability, quality, selected-record or source-pair transition. Δfeed compares adjacent samples, not hypothetical continuous limits; the CSV contains every sample and output. Floating-point evaluation can place an exact decimal boundary on either side; ±epsilon probes establish the two sides.

## doc1 / Prudent

| Diameter mm | Quality | Source mm | RPM | Feed | Plunge | DOC mm | Δfeed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.000000000 | Unavailable | none | — | — | — | — | — |
| 2.116666667 | Extrapolated | 3.175 | 22000 | 466 | 235 | 1.000000 | — |
| 2.540000000 | Approximate | 3.175 | 22000 | 672 | 339 | 1.000000 | +0.15% |
| 2.886364636 | Close | 3.175 | 22000 | 954 | 481 | 1.000000 | +10.03% |
| 3.527777778 | Approximate | 3.175 | 19800 | 945 | 530 | 1.000000 | +0.00% |
| 4.233333333 | Interpolated | 3.175+6.35 | 19500 | 1239 | 651 | 1.000000 | +57.43% |
| 5.772728273 | Close | 6.35 | 17000 | 1472 | 736 | 1.000000 | -2.90% |
| 7.055555556 | Approximate | 6.35 | 15300 | 1458 | 810 | 1.000000 | +0.00% |
| 8.466666667 | Extrapolated | 6.35 | 12750 | 1215 | 810 | 1.000000 | +0.00% |
| 12.700001000 | Unavailable | none | — | — | — | — | — |
## doc1 / Normal

| Diameter mm | Quality | Source mm | RPM | Feed | Plunge | DOC mm | Δfeed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.000000000 | Unavailable | none | — | — | — | — | — |
| 2.116666667 | Extrapolated | 3.175 | 22000 | 622 | 311 | 1.000000 | — |
| 2.540000000 | Approximate | 3.175 | 22000 | 896 | 448 | 1.000000 | +0.11% |
| 2.886364636 | Close | 3.175 | 22000 | 1272 | 636 | 1.000000 | +9.94% |
| 3.527777778 | Approximate | 3.175 | 19800 | 1259 | 700 | 1.000000 | +0.00% |
| 4.233333333 | Interpolated | 3.175+6.35 | 19500 | 1653 | 862 | 1.000000 | +57.43% |
| 5.772728273 | Close | 6.35 | 17000 | 1963 | 981 | 1.000000 | -2.87% |
| 7.055555556 | Approximate | 6.35 | 15300 | 1943 | 1080 | 1.000000 | +0.00% |
| 8.466666667 | Extrapolated | 6.35 | 12750 | 1619 | 1080 | 1.000000 | +0.00% |
| 12.700001000 | Unavailable | none | — | — | — | — | — |
## doc1 / Rapid

| Diameter mm | Quality | Source mm | RPM | Feed | Plunge | DOC mm | Δfeed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.000000000 | Unavailable | none | — | — | — | — | — |
| 2.116666667 | Extrapolated | 3.175 | 22000 | 777 | 391 | 1.000000 | — |
| 2.540000000 | Approximate | 3.175 | 22000 | 1120 | 563 | 1.000000 | +0.09% |
| 2.886364636 | Close | 3.175 | 22000 | 1590 | 800 | 1.000000 | +9.96% |
| 3.527777778 | Approximate | 3.175 | 19800 | 1575 | 880 | 1.000000 | +0.00% |
| 4.233333333 | Interpolated | 3.175+6.35 | 19500 | 2066 | 1082 | 1.000000 | +57.47% |
| 5.772728273 | Close | 6.35 | 17000 | 2454 | 1227 | 1.000000 | -2.89% |
| 7.055555556 | Approximate | 6.35 | 15300 | 2430 | 1350 | 1.000000 | +0.00% |
| 8.466666667 | Extrapolated | 6.35 | 12750 | 2024 | 1350 | 1.000000 | +0.00% |
| 12.700001000 | Unavailable | none | — | — | — | — | — |
## doc10 / Prudent

| Diameter mm | Quality | Source mm | RPM | Feed | Plunge | DOC mm | Δfeed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.000000000 | Unavailable | none | — | — | — | — | — |
| 2.116666667 | Extrapolated | 3.175 | 22000 | 466 | 235 | 1.911111 | — |
| 2.540000000 | Approximate | 3.175 | 22000 | 672 | 339 | 2.752000 | +0.15% |
| 2.886364636 | Close | 3.175 | 22000 | 954 | 481 | 3.909092 | +10.03% |
| 3.527777778 | Approximate | 3.175 | 19800 | 945 | 530 | 4.300000 | +0.00% |
| 4.233333333 | Interpolated | 3.175+6.35 | 19500 | 1239 | 651 | 4.444444 | +57.43% |
| 5.772728273 | Close | 6.35 | 17000 | 1472 | 736 | 2.545455 | -2.90% |
| 7.055555556 | Approximate | 6.35 | 15300 | 1458 | 810 | 2.800000 | +0.00% |
| 8.466666667 | Extrapolated | 6.35 | 12750 | 1215 | 810 | 2.800000 | +0.00% |
| 12.700001000 | Unavailable | none | — | — | — | — | — |
## doc10 / Normal

| Diameter mm | Quality | Source mm | RPM | Feed | Plunge | DOC mm | Δfeed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.000000000 | Unavailable | none | — | — | — | — | — |
| 2.116666667 | Extrapolated | 3.175 | 22000 | 622 | 311 | 2.844444 | — |
| 2.540000000 | Approximate | 3.175 | 22000 | 896 | 448 | 4.096000 | +0.11% |
| 2.886364636 | Close | 3.175 | 22000 | 1272 | 636 | 5.818184 | +9.94% |
| 3.527777778 | Approximate | 3.175 | 19800 | 1259 | 700 | 6.400000 | +0.00% |
| 4.233333333 | Interpolated | 3.175+6.35 | 19500 | 1653 | 862 | 6.600000 | +57.43% |
| 5.772728273 | Close | 6.35 | 17000 | 1963 | 981 | 3.727273 | -2.87% |
| 7.055555556 | Approximate | 6.35 | 15300 | 1943 | 1080 | 4.100000 | +0.00% |
| 8.466666667 | Extrapolated | 6.35 | 12750 | 1619 | 1080 | 4.100000 | +0.00% |
| 12.700001000 | Unavailable | none | — | — | — | — | — |
## doc10 / Rapid

| Diameter mm | Quality | Source mm | RPM | Feed | Plunge | DOC mm | Δfeed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.000000000 | Unavailable | none | — | — | — | — | — |
| 2.116666667 | Extrapolated | 3.175 | 22000 | 777 | 391 | 3.777778 | — |
| 2.540000000 | Approximate | 3.175 | 22000 | 1120 | 563 | 5.440000 | +0.09% |
| 2.886364636 | Close | 3.175 | 22000 | 1590 | 800 | 7.727275 | +9.96% |
| 3.527777778 | Approximate | 3.175 | 19800 | 1575 | 880 | 8.500000 | +0.00% |
| 4.233333333 | Interpolated | 3.175+6.35 | 19500 | 2066 | 1082 | 8.777778 | +57.47% |
| 5.772728273 | Close | 6.35 | 17000 | 2454 | 1227 | 5.000001 | -2.89% |
| 7.055555556 | Approximate | 6.35 | 15300 | 2430 | 1350 | 5.500000 | +0.00% |
| 8.466666667 | Extrapolated | 6.35 | 12750 | 2024 | 1350 | 5.500000 | +0.00% |
| 12.700001000 | Unavailable | none | — | — | — | — | — |
## limited / Prudent

| Diameter mm | Quality | Source mm | RPM | Feed | Plunge | DOC mm | Δfeed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.000000000 | Unavailable | none | — | — | — | — | — |
| 2.116666667 | Extrapolated | 3.175 | 16000 | 339 | 235 | 1.911111 | — |
| 2.540000000 | Approximate | 3.175 | 16000 | 488 | 300 | 2.752000 | +0.00% |
| 2.886364636 | Close | 3.175 | 16000 | 694 | 300 | 3.909092 | +9.98% |
| 3.527777778 | Approximate | 3.175 | 16000 | 700 | 300 | 4.300000 | +0.00% |
| 4.233333333 | Interpolated | 3.175+6.35 | 16000 | 700 | 300 | 4.444444 | +0.00% |
| 5.772728273 | Close | 6.35 | 16000 | 700 | 300 | 2.545455 | +0.00% |
| 7.055555556 | Approximate | 6.35 | 15300 | 700 | 300 | 2.800000 | +0.00% |
| 8.466666667 | Extrapolated | 6.35 | 12750 | 700 | 300 | 2.800000 | +0.00% |
| 12.700001000 | Unavailable | none | — | — | — | — | — |
## limited / Normal

| Diameter mm | Quality | Source mm | RPM | Feed | Plunge | DOC mm | Δfeed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.000000000 | Unavailable | none | — | — | — | — | — |
| 2.116666667 | Extrapolated | 3.175 | 16000 | 452 | 300 | 2.844444 | — |
| 2.540000000 | Approximate | 3.175 | 16000 | 651 | 300 | 4.096000 | +0.00% |
| 2.886364636 | Close | 3.175 | 16000 | 700 | 300 | 5.818184 | +0.00% |
| 3.527777778 | Approximate | 3.175 | 16000 | 700 | 300 | 6.400000 | +0.00% |
| 4.233333333 | Interpolated | 3.175+6.35 | 16000 | 700 | 300 | 6.600000 | +0.00% |
| 5.772728273 | Close | 6.35 | 16000 | 700 | 300 | 3.727273 | +0.00% |
| 7.055555556 | Approximate | 6.35 | 15300 | 700 | 300 | 4.100000 | +0.00% |
| 8.466666667 | Extrapolated | 6.35 | 12750 | 700 | 300 | 4.100000 | +0.00% |
| 12.700001000 | Unavailable | none | — | — | — | — | — |
## limited / Rapid

| Diameter mm | Quality | Source mm | RPM | Feed | Plunge | DOC mm | Δfeed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1.000000000 | Unavailable | none | — | — | — | — | — |
| 2.116666667 | Extrapolated | 3.175 | 16000 | 565 | 300 | 3.777778 | — |
| 2.540000000 | Approximate | 3.175 | 16000 | 700 | 300 | 5.440000 | +0.00% |
| 2.886364636 | Close | 3.175 | 16000 | 700 | 300 | 7.727275 | +0.00% |
| 3.527777778 | Approximate | 3.175 | 16000 | 700 | 300 | 8.500000 | +0.00% |
| 4.233333333 | Interpolated | 3.175+6.35 | 16000 | 700 | 300 | 8.777778 | +0.00% |
| 5.772728273 | Close | 6.35 | 16000 | 700 | 300 | 5.000001 | +0.00% |
| 7.055555556 | Approximate | 6.35 | 15300 | 700 | 300 | 5.500000 | +0.00% |
| 8.466666667 | Extrapolated | 6.35 | 12750 | 700 | 300 | 5.500000 | +0.00% |
| 12.700001000 | Unavailable | none | — | — | — | — | — |

## Interpretation

Unavailable below the smallest admissible anchor and above the largest admissible anchor follows the50% policy bound. The25% boundary changes the label, usually without a numeric jump beyond rounding. The10% small-tool boundary changes q² to q. At≈4.233333mm the second anchor becomes eligible and interpolation supersedes a single source; at≈5.772727mm close matching supersedes interpolation. These switches explain the larger changes. DOC1 and synthetic caps mask some changes but do not remove their cause.
