# PERF1-A raw-data summary

Native control release; cold process first call, warm n=5 after one warmup. Timings in ms.

| Case/post | Classification | Cold | Warm median [min, max] | Moves | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: |
| missing-layer-cps | empty-source-diagnostic | 14.25 | 14.22 [13.17, 14.58] | 0 | 36.4 |
| missing-layer-linuxcnc | empty-source-diagnostic | 0.11 | 0.02 [0.01, 0.02] | 1 | 8.3 |
| pocket-islands-cps | accepted | 406.71 | 404.82 [403.32, 409.52] | 1535 | 38.8 |
| pocket-islands-linuxcnc | accepted | 338.91 | 335.53 [334.02, 340.09] | 1538 | 14.8 |
| profile-contours-cps | accepted | 25.74 | 25.11 [24.92, 25.24] | 307 | 30.9 |
| profile-contours-linuxcnc | accepted | 0.23 | 0.05 [0.04, 0.06] | 246 | 8.9 |
| profile-many-segments-cps | accepted | 1271.34 | 1217.40 [1210.39, 1224.16] | 25215 | 126.2 |
| profile-many-segments-linuxcnc | accepted | 54.64 | 6.14 [5.66, 9.85] | 41602 | 90.9 |
| square-medial-cps | accepted | 271.24 | 269.10 [268.81, 298.73] | 5862 | 44.8 |
| square-medial-linuxcnc | accepted | 5.35 | 0.75 [0.72, 0.75] | 5865 | 14.2 |
| square-outline-cps | accepted | 28.02 | 18.78 [18.50, 19.83] | 130 | 30.9 |
| square-outline-linuxcnc | accepted | 1.59 | 0.09 [0.08, 0.10] | 133 | 8.7 |
| text-B8-medial-cps | refused | 7.70 | 5.84 [5.83, 5.88] | None | 12.4 |
| text-B8-medial-linuxcnc | refused | 7.63 | 5.84 [5.80, 5.89] | None | 14.2 |
| text-Test-medial-cps | accepted | 820.49 | 903.53 [818.46, 975.72] | 18874 | 83.1 |
| text-Test-medial-linuxcnc | accepted | 32.54 | 3.35 [3.34, 3.43] | 18877 | 31.5 |
| text-Test-outline-cps | refused | 0.45 | 0.25 [0.25, 0.26] | None | 11.7 |
| text-Test-outline-linuxcnc | refused | 1.02 | 0.78 [0.77, 0.81] | None | 13.7 |

Nested/inclusive native stages, exact counts, WASM binding boundaries and browser endpoint distributions are in SUMMARY.json. Do not sum child spans into parent elapsed time.
