# Local Sorbonne preset authoring

FS3.8 provides the capture workflow. It does not authorize FS4 or certify a cut.

## One-time context setup

Select job material in Stock. Use a precise existing taxonomy identity or create a
custom material; a broad plywood name is not automatically birch plywood.
Machine → Settings and Tool library → expanded cutter → Cutting reference
compatibility allow selection of a genuinely compatible reference or **Create local
identity** for equipment with no reference. Local IDs are generated UUIDs in the
existing project fields. Keep them when renaming; a different machine/setup or
physical cutter needs its own identity. Replacing reference identity is explicit
and can change future source compatibility; existing rates remain authoritative.
No display string is used as identity. Shared presets require these identities to
travel with the project/tool setup, rather than recreating them on each workstation.

## Capture a cut

1. Select an operation and review/apply a recommendation or edit cutting values.
2. After a physical test, return to that same operation.
3. Choose **Save as lab preset…** in Cutting parameters.
4. Review prefilled name/context. Destination is Sorbonne Fablab, local only.
5. Select Tested only after actual machine use; enter date and attest physical use
   and no critical failure. Brief observations are optional. Optional trial details
   contain operator, job reference and detailed validation observations.
6. Save. The operation and its actual rates remain unchanged.

Draft preserves an untested/unknown-context capture and is excluded from all
recommendations. There is no direct Validated option. Unknown entry/engagement
cannot become Tested: explicitly confirm reliable context and recapture later.
Dates must be valid calendar dates, not in the future. Core uses the same resolved
rates as generation: manual overrides win over old recommendations and fallbacks.

Capture includes exact machine/material/tool identities, cutter family, diameter,
flutes, thickness where available, operation, maximum scheduled DOC, entry,
known full-width or nominal engagement, RPM/feed/plunge. Nominal pocket engagement
is the existing CAM overlap-derived context, not a physical geometric guarantee.
Unknown helix/coating/shape are not inferred from cutter names. Pocket capture
records primary roughing; secondary tool/rates are notes only. Separate finishing
qualification remains unsupported until its engagement is reliably modeled.
A public applied proposal supplies its recorded mode; absent that (including a
private safe summary) capture scopes to Prudent. No unrecorded Normal/Rapid claim
is added. The exact cutting values are never altered by that scope decision.

If context changes while the dialog is open, Save is blocked; reopen. If another
window writes the pack, the old-head save is refused; reopen to refresh knowledge.

## Duplicates, revisions and management

Similar context offers **Save as new** or **Update existing**. Similar means exact
identity/version, operation, engagement, entry and thickness, with DOC within
5% or 0.01 mm for the duplicate hint only. This is not a machining tolerance or
resolver qualification: saved scopes retain exact captured DOC.
Updating preserves preset ID and creation date, creates a new revision and retains
prior values/evidence/notes. An old Validated preset is not silently overwritten:
new captured values need their own Tested/Draft declaration and later review.

Settings → Feeds & Speeds → Sorbonne Fablab → Manage lab presets provides search,
selection, context/trial/history disclosures, observations editing, disable,
confirmed reversible deletion and restore. Draft cannot be enabled or promoted.
Tested can be promoted through the deliberate review described in
[validation policy](LOCAL_VALIDATION_POLICY.md). Editing notes keeps evidence and
creates history; changing cutting values requires recapture and renewed review.

Eligible local presets enter the existing resolver: exact Validated, then Tested,
then compatible reference data. No UI ranking, averaging or generalization of
machine/material/operation specificity. Application still requires explicit
Recalculate followed by Apply; Save never applies a recommendation.

## Private storage and backups

Desktop: Tauri app_data_dir()/feeds-speeds/sources, OS-resolved app directory,
immutable ID@revision.json files, Unix mode 0600. Browser: origin-local IndexedDB
ivac-cutting-source-packs, packs store. No parallel DB, repository write, automatic
upload, bundling, publication or sync. Export is a deliberate exception for backup.
Changing origin/port/browser profile changes browser storage; use export first.

**Export Sorbonne source pack…** saves current schema-2 pack with all retained
preset history. It includes private observations/operator/job details; keep the
file in a location you control. **Import source pack…** validates bounded JSON,
identities, scope/numeric values and evidence metadata. On a fresh installation,
restores the pack; enable it explicitly as needed. Duplicate/equal/older revisions
are rejected if newer knowledge exists. No automatic merge or destructive rollback.
Only the current Sorbonne head is active; older immutable files remain for recovery.
Limits: 2 MiB current pack, 32 distinct IDs, 512 revisions total and 32 non-lab
revision files. Export before reaching limits; large-history cleanup is not a UI
feature in this checkpoint. If a native crash leaves .sorbonne-write-lock, close
all app instances before removing that marker; do not remove it during a save.

Portable projects retain safe knowledge identities and actual rates, never the
private pack/observations. Missing backup/source availability does not block
opening a project or generation from saved values.
