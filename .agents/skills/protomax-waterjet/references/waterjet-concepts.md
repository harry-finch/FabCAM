# Waterjet concepts for future architecture

This is a design vocabulary, not a claim about a specific ProtoMAX file format.

## Shared with other processes

Potentially reusable: source geometry, contour topology, inside/outside determination, offset algorithms, operation ordering, lead geometry helpers, machine envelope, fixtures/keep-outs, project persistence, and visualization.

## Waterjet-specific intent

Likely concepts: traverse with jet off, pierce event, cut segment, quality level, kerf compensation, lead-in, lead-out, slit/no-compensation cut, and etch/marking.

## Architectural test

A good design should allow:

```text
Geometry
  ├── Milling operation → milling semantics → ShopBot post
  └── Waterjet operation → waterjet semantics → OMX/OMAX post
```

Before adding new generic IR concepts, inspect whether existing plasma/laser semantics already cover them cleanly.
