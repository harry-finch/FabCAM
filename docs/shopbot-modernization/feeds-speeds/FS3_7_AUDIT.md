# FS3.7 Phase A audit

Baseline: 8271dd3, feature/shopbot-mvp. User explicitly authorized FS3.7 phases A–G.

| Boundary | Existing implementation | Decision |
| --- | --- | --- |
| Mathematics | ivac-feeds-speeds/engine.rs pure chipload, limits and scoped modes | KEEP |
| Data validation | dataset.rs version 1 catalogs/profiles, no I/O | EXTEND pack envelope |
| Applicability | immutable identities, explicit families and strict subset | EXTEND explicit taxonomy/multi-pack resolver |
| Built-in source | sienciEnabled + fixed resource URL | REPLACE with reviewed manifest registry |
| Local source | custom.contents inside localStorage preferences | REPLACE with app-data files/IndexedDB and references |
| Source application | full Profile in appliedCutting.proposal.provenance | EXTEND private-safe identity summary; no private profile in projects |
| Generation | only saved cutting rates; no engine calls | KEEP |
| UX | compact material/engagement/mode, collapsed context/provenance | KEEP; source count and advanced explanation |
| Other sources | no licensed datasets imported | DEFER data population pending licensing |

The Sienci category combines soft stock classes. Do not equate its ID to all MDF,
plywood or softwood. Existing machine context is LongMill, not ShopBot; exact DOC
and known slotting scope remain unchanged. Source identities describe references,
not lab inventory. Applied/manual values remain independent of source availability.

Phase B: ADR-0005 resolves the storage/privacy and resolution decisions before code.
Local Tested/Validated import declarations require existing trial/criteria metadata;
they are not physical qualification performed by this checkpoint.
