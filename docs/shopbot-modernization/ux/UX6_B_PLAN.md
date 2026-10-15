# UX6-B — Tool Library UX Redesign

2026-10-15 · **IMPLEMENTED — human review ivac-y2aln pending**.
User accepted UX6-A and authorized B; Phase 0 contracts retained.
[Implementation, tests, actual desktop evidence and limits](UX6_B_RESULTS.md).
The original acceptance specification follows; optional schematic omitted in favor
of explicit dimensions and unknown-data labels.

## Browse, inspect and edit

Searchable compact list (kind, diameter, flutes, relevant type metadata and filters)
plus contextual selected-tool editor. Primary workflow Find → Inspect → Edit if
needed; avoid permanently expanded technical rows and irrelevant n/a columns.
Retain direct editing only where Phase 0 usage justifies it. Type changes must not
silently discard hidden data or fabricate dimensions.

Provisional detail groups, confirmed against actual consumers:

- Cutting geometry: family, diameter/flutes/flute length, V angle/tip and supported
  type-specific dimensions. Distinguish usable/effective geometry accurately.
- Mounting/clearance: modeled shank/body/overall or usable length, stickout and holder;
  do not invent equivalent fields or promise exact clearance from missing data.
- Availability: library identity, machine association/loadout and applicable process
  with evidence/compatibility limits. Not proof of installation or physical verification.
- Notes/Advanced/Legacy: descriptions, calibration and specialized fields classified
  by real effect, not blindly hidden.

Optional compact 2D technical silhouette with dimension annotations from actual data:
diameter, flute length, shank/stickout, V included angle/tip and holder where available.
Unknown stays visibly unknown; schematic is not a measured model. No new 3D engine.

## Physical tools, legacy values and associations

Physical tool, saved operation CuttingParameters and Recommendation stay separate.
Do not embed universal recipes or change FS context/math/apply/provenance. Existing
“Fallback cutting defaults” move to secondary Legacy compatibility presentation,
explaining applicability to older operations without saved values; no deletion/migration
or silent application to modern operations. Effective fallback origin remains explainable
in the existing operation workflow, not a new rate-precedence model.

Audit Z shift/warmup/spindle direction/whirling/wear/reference compatibility/holder:
type/process/post effects determine context and prominence. Preserve consequential
settings, stored unsupported values and safeguards even when normally hidden.

Add to machine / Remove from this machine / Open details / Delete from library must
map to real existing ownership. Shared inventory, stocked copies, profile mirroring
and operation references are distinct; association removal is not global deletion.
Preserve project-local tools and explicit assignment/history. Show which operations/
loadouts are affected before destructive actions; no new association schema assumed.

Apply/Revert/import replace/add/export retain their audited draft/storage scope and
format IDs. No autosave change or promise of workspace-wide Undo without support.
Editing physical geometry never silently recalculates stored cutting parameters.
All relevant tool changes invalidate output through existing authority.

## Future acceptance criteria and evidence

1. Efficient search/filter and compact essential characteristics.
2. Tool essentials readable without a large editor.
3. Geometry fields relevant to tool type, preserved hidden data.
4. Consistent contextual editor and keyboard/focus access.
5. Unknown dimensions not fabricated; schematic limitations clear.
6. Physical geometry separate from applied/recommended cutting values.
7. Legacy fallbacks retained/explained without dominating normal use.
8. Library/machine/operation states and associations clear.
9. Removing association distinct from deleting globally, with dependency safeguards.
10. Import/add/replace/export and Apply/Revert scope explicit.
11. Existing tools/operations/projects/formats and manual values compatible.
12. Diameter/type/V-bit/association/delete/import/history/save changes preserve
    compatibility, generation invalidation and export preflight.
13. Relevant automated regressions and actual desktop visual/accessibility review pass.

Capture actual list/search, flat end mill/V-bit, geometry details, unknown dimensions,
legacy expanded, machine association and replace-import confirmation. Include narrow
desktop/long names and hidden incompatible values. Test identity/copy propagation,
referenced deletion, workspace versus project history, manual-rate save/reopen and
stale export; no test-only physical compatibility or validation claim.

Phase 0 resolved ownership and transaction scope; B implements the existing inventory-wide
draft and copy model with explicit consequences. No domain/schema change was required.

**HARD STOP — human software/visual acceptance ivac-y2aln before another milestone.**
