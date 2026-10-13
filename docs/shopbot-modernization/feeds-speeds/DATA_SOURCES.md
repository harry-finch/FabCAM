# Cutting-data licensing matrix — FS3.7

Reviewed 2026-10-12. Public readability and application code licenses are not
redistribution rights for third-party data. Only the explicit bundled registry
is shipped; imported manifests cannot add themselves to it. Local-only storage
is a distribution policy, not a statement of legal permission.

| Source | Data available | Data license / evidence | Redistribution | Transformation | Attribution / share alike | Status | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Sienci Labs, pinned LongMill metric PDF, six single-flute UC rows | Six existing metric reference rows | [Pinned Resources LICENSE](https://github.com/Sienci-Labs/Resources/blob/060e2291a1cf17bf9be4f151d7aa539a42ed2b0c/LICENSE): CC BY-SA 4.0 | Permitted subject to license | Permitted subject to license | Sienci Labs, pinned source, adaptation notice, CC BY-SA text/link; adapted data stays CC BY-SA | APPROVED_PUBLIC | Only this reviewed subset; application GPL is separate; no endorsement or local validation |
| User/lab original measurements and presets | User-supplied only; initially no production local records | Actual local rights and provenance required in each manifest | Not distributed by FabCAM | User responsibility / actual terms | Retain authored record permissions and trial metadata | LOCAL_ONLY | Shared projects carry applied values and pack/record IDs, not the local dataset |
| FabCAM / Sorbonne original public records | No cutting dataset created in FS3.7 | Public license NOT selected | Not authorized yet | Not authorized yet | Requires explicit owner/user license decision | LICENSING_REVIEW_REQUIRED | Do not infer GPL for data or pick a license without approval |
| LMT Onsrud | [Official cutting-data resources](https://onsrud.com/) | Data-specific redistribution grant not established | Unverified; do not bundle | Unverified | Permission review required | LICENSING_REVIEW_REQUIRED | No table copied or scraped |
| Amana Tool | [Official product reference](https://www.amanatool.com/products/router-bits/straight-plunge-router-bits/straight-plunge-router-bits/45210-carbide-tipped-straight-plunge-high-production-1-4-dia-x-1-inch-x-1-4-shk.html); some charts carry [All Rights Reserved notice](https://www.amanatool.com/pub/media/productattachments/SCFD-material-speed-and-feed-chart.pdf) | No redistribution grant established | Do not bundle | Not established | Explicit data permission required | LICENSING_REVIEW_REQUIRED | Manufacturer availability does not grant copy rights; no data imported |
| DATRON | [Official cutting guide](https://www.datron.com/wp-content/uploads/2022/11/DATRON-High-Speed-Cutting-Guide.pdf) | Data-specific redistribution grant not established | Do not bundle | Unverified | Explicit data permission required | LICENSING_REVIEW_REQUIRED | DATRON Dynamics is the machining source; unrelated communications-company terms are not evidence |
| Sorotec | [Official downloads](https://webseite.sorotec.de/service/downloads/) | Data-specific redistribution grant not established | Do not bundle | Unverified | Explicit data permission required | LICENSING_REVIEW_REQUIRED | No table copied or scraped |

## Exact Sienci verification

The pinned [handbook](https://github.com/Sienci-Labs/Resources/blob/060e2291a1cf17bf9be4f151d7aa539a42ed2b0c/cnc-fun/handbook/cnc-feeds-speeds.md)
links the Sienci metric table. Re-fetched the exact repository LICENSE, handbook
and `_downloads/FeedsSpeedsMetric.pdf` via GitHub's contents API at
`060e2291a1cf17bf9be4f151d7aa539a42ed2b0c`. License identifies
Attribution-ShareAlike 4.0; PDF SHA-256 matches the prior source qualification:
`e7c4519025ce1a285733138d1295423995617a80e2b4f2b5c135d4607a6167d8`.
[CC BY-SA 4.0 terms](https://creativecommons.org/licenses/by-sa/4.0/legalcode.en)
permit sharing/adaptation with attribution and applicable share-alike conditions.
The migrated pack preserves the original numerical dataset, record permissions,
source notes/hash and attribution. Existing license text accompanies the bundle.

These statuses are scoped engineering distribution decisions, not blanket legal
permission for all material on a vendor site. New files, rights exclusions or new
sources require their own review. No proprietary extended pack was populated.

## FS3.9 distribution delta — 2026-10-13

Reviewed bundled revision 2026-10-13.1 adds FabCAM adaptation declarations, not
cutting rows. The same six pinned Sienci profiles/source notes/numbers remain
unchanged. Original and derived data remain separate under the existing CC BY-SA
metadata; prior version is retained for regression. No manufacturer permission
status changed, no private records populated and no public Sorbonne license chosen.
