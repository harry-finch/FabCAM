# NUMSAFE3 specialized planning budgets

Approved by the maintainer on 2026-10-20. These are initial product limits, not
universal memory or elapsed-time guarantees. They do not qualify additional
machine/post combinations.

| Resource | Per-operation ceiling |
|---|---:|
| Waterline depth levels, including final floor | 1,000 |
| Heightgrid-generated mesh triangles | 2,000,000 |
| Relief footprint entries | 1,000,000 |
| Relief cells × footprint entries | 100,000,000 |
| Waterline / Relief / Thread retained XYZ path points | 1,000,000 |

`cam/resource_limits.rs` owns the non-depth constants; the shared
`cam::MAX_DEPTH_PASSES` owns the depth policy. Counts/products are checked before
large generated allocations where they can be known in advance. Dynamic Waterline
clearing and Thread whirling check growth before appending points; a failure
discards the operation rather than returning its completed prefix.

No step, resolution, pitch or valid point is changed to make an oversized request
fit. Thread counts include radial passes, selected circles and lead-in geometry.
Relief work is conservatively assessed without relying on pruning. Shared pocket
callers retain their existing size policy; only Waterline supplies the specialized
point budget to the shared raster helper.

Checked planner entry points return errors. Legacy infallible convenience APIs
may panic on an excessive request; production drivers use the checked interfaces.
This preserves source call shapes without silently treating refusal as an empty
successful operation. Cancellation remains distinct in the pipeline.

Polling covers mesh construction, thread passes/helix/whirling points, relief
sampling and bounded clearing scanlines. Sorting, contour stitching and Boolean
library calls still contain noninterruptible spans. No universal 200ms latency
guarantee is made; `ivac-42ylx` remains open.

Existing laser streaming raster semantics and other operation-specific policies
are unchanged. Their inherited resource risks are not certified away by this
checkpoint. Physical and installed-platform qualification remain separate.
