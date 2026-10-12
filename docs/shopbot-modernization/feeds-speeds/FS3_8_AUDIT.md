# FS3.8 pre-implementation audit

User explicitly authorizes FS3.8; FS4 remains unstarted. Baseline FS3.7 is the
uncommitted implementation described in ADR-0005.

| Existing boundary | Disposition | FS3.8 decision |
| --- | --- | --- |
| Pure Engine and FS3.7 rank | KEEP | No math or precedence rewrite |
| SourcePack/RecordOrigin and loader | EXTEND | Optional local authoring metadata and revision snapshots; old packs remain readable |
| Tauri app-data / browser IndexedDB | EXTEND | Immutable Sorbonne pack revisions, optimistic concurrency; same storage, no parallel database |
| Core cam_request/saved_rates | KEEP/EXTEND | Read-only capture adapter supplies actual resolved rates and exact context |
| Evidence enum | KEEP | Draft is authoring workflow, excluded from resolution, not a new cutting-evidence claim |
| ExactPreset validity | KEEP | Tested needs complete exact identities/entry/engagement and physical-success attestation; unknown engagement can be captured as inactive Draft |
| Settings source management | EXTEND | Compact searchable Sorbonne records, inspect/notes/promote/disable/delete with revision history and export |
| Project values / posts | KEEP | Capture never applies/recalculates or changes current operation |

Stable machine/material/tool FS identities must already exist. Missing identities
are not invented from display names; one-time Machine/Tool compatibility setup is
required before a reusable exact preset can be authored. The existing Machine/Tool
selector initially only listed public/catalog references; FS3.8 adds deliberate
local UUID creation using existing fsIdentity fields, solving the first-capture
bootstrap without deriving identity from names or making Save mutate equipment. Reliable DOC, thickness,
entry and nominal/full/unknown engagement come from the existing native adapter.
Pocket capture covers the primary roughing role, retaining secondary finish rates
as context only; it never labels roughing as a qualified finishing preset.

Policy sorbonne-local-validation-1: Tested requires a dated physical use with no
critical failure, exact context and resolved values. Validated requires later
explicit review: acceptable finish/behavior, no overheating/chatter/overload and
operator suitability for normal lab use, with review notes. No arbitrary test count
or numerical tolerance is invented. User attestation is evidence, not automatic
verification of machine use. Drafts cannot influence recommendations.

Every update creates a new immutable pack/profile revision, retaining prior profile
and authoring metadata in the exported current record history. Concurrent saves
against an old pack head are refused. Disabling and deletion are reversible flags;
delete requires deliberate confirmation and retains historical data. Export is
explicit, local, includes private notes/operator data, and does not publish rights.
No public license for new Sorbonne data is chosen.
