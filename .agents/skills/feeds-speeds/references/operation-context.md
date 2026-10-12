# Operation context

The CAM integration should provide operation context automatically where it already knows it.

## Profile

Useful context may include:

- cutter side,
- DOC,
- number/depth of passes,
- whether geometry implies full-width cutting,
- finishing allowance/pass where relevant.

Do not assume every profile is full-width slotting.

## Pocket

Useful context may include:

- configured stepover,
- DOC,
- pocket strategy,
- finishing pass,
- whether the path contains locally higher engagement than nominal stepover.

For the MVP, use configured stepover conservatively rather than claiming exact instantaneous engagement.

## Slotting

Slotting should be represented explicitly when the CAM knows the cutter is engaged approximately full width.

Typical trigger examples may include:

- slot operation,
- centerline cut where slot width approximately equals tool diameter,
- explicitly classified full-width engagement.

Do not classify arbitrary profile operations as slotting without evidence.

## Finishing

Finishing context may include:

- small radial allowance,
- shallow axial allowance,
- finish-pass flag.

Keep the policy understandable and conservative.

## Surfacing

Useful context may include:

- surfacing tool family,
- tool diameter,
- stepover,
- DOC,
- large radial cutter diameter relative to machine capability.

Surfacing may need separate tool-family data rather than ordinary end-mill assumptions.

## Engraving / V tools

Do not force conventional end-mill chipload rules onto V-bits or engraving tools without appropriate tool-family data.

If the initial dataset cannot support the tool type reliably, return an explicit unsupported/low-evidence result instead of inventing one.

## Unknown engagement

When actual engagement is unknown:

- do not assume low engagement,
- use a documented conservative default or fallback,
- expose the assumption in recommendation provenance/warnings.

The engine should distinguish:

```text
known engagement
estimated engagement
unknown engagement
```

if that distinction proves useful during FS0.
