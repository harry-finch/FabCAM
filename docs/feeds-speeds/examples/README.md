# Synthetic source-pack examples

[Authoring guide](../CUSTOM_DATA_SOURCES.md) · [Validation record](../VALIDATION_2026_10_20.md).

These original GPL-3.0-or-later documentation examples use invented workshop,
material, cutter and cutting values. They demonstrate the schema and resolver,
**not** machining advice or manufacturer/physical validation. License:
[repository LICENSE](../../../LICENSE). Attribution: FabCAM contributors.

| File | Purpose |
|---|---|
| [workshop-demo-v1.json](workshop-demo-v1.json) | Schema-1 pack; three synthetic records: Prudent Pocket/reference, Prudent slot/reference, Normal Pocket/theoretical |
| [workshop-demo-v2.json](workshop-demo-v2.json) | Same pack/catalog IDs, new immutable revision; one Pocket feed changed 600→660 with its record revision |
| [validate_examples.py](validate_examples.py) | Offline temporary Rust harness against the actual local loader and source resolver |

No profile is Tested or Validated. No trial references, operator records or real
machine/tool maxima are invented. The `reference_data` category in two rows is
explicitly synthetic: it demonstrates serialization, not reference credibility.
The packs are private-visibility/local-only for safe saved-provenance handling;
GPL redistribution permission does not add them to the application's bundled registry.
They are not placed under `frontend/public` or automatically activated.

Run from the repository root with Python 3 and the documented Rust toolchain:

```sh
python3 docs/feeds-speeds/examples/validate_examples.py
```

The harness loads both complete JSON files with `load_source_pack_json_checked`,
checks three matching request contexts per revision, rejects unknown manifest fields,
unsupported evidence promotion and missing origin metadata, and checks stable
catalog/record identities across the revision pair. Dependencies are seeded from
this checkout's Cargo.lock; Cargo works offline using its existing cache. The harness
and build output are in temporary directories; repository source/lockfile and
application storage remain untouched. A missing dependency/toolchain requires the
normal build setup, not a schema relaxation.

For your own source, author genuine data/provenance in a new ID/revision and use
the real application import validation described in the guide. The helper checks
these two demonstration files specifically; it is not a generic import CLI or a
proof that any claimed physical trial really occurred.
