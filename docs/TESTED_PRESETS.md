# Cutting values and tested preset collections — V1.1

Enter cutting values in each operation's **Cutting parameters** inspector. Select the
actual pass role when several are shown. RPM, feed and plunge are operation-owned;
changing a library default later does not change them. New operations may be seeded
once from the chosen tool's defaults. Review every value before generation.

Profile's contour uses **Finishing**; its stored main values remain separate. Pocket
can use different roughing and finishing tools. Drill can have Drill, Spot and Chamfer
roles. An incomplete operation can be saved and edited; generation refuses missing,
non-positive or mismatched values and known machine/tool-limit violations. Values
are never reduced silently to fit a rating. Unknown limits are disclosed, not inferred;
specialized operations can still require specific ratings or geometry.

## Save a tested configuration

Set the workshop machine and tool identities in their libraries, choose the exact
Stock material, and enter complete operation values. **Save preset** explains missing
context. Give the configuration a meaningful name and select or create a named
collection. Review the context and explicitly attest workshop testing only when it
really occurred. Software validation is a separate check and cannot establish that
physical test. Synthetic QA entries must be clearly marked and never presented as
workshop recommendations.

Collections contain tested configurations; they have no Draft/Trial promotion,
manufacturer ranking, automatic interpolation or enable-source switch. FabSpeed is
an independent possible future service; no connection is implemented here.

## Load and review

**Load preset** shows the compatible context and the values that will replace the
operation's current values. Confirm the reviewed copy. The tool, machine and material
are not silently replaced. A matching local tool number alone is not matching physical
tool identity or geometry. Different context is refused; there is no automatic transfer.

A refusal titled **Preset context needs review** preserves the original record,
signature and author claim. Open **Recorded compatibility context** in the load
review or library details to inspect the saved machine configuration, physical
cutter geometry/roles, material revision and operation strategy. This read-only
view uses the saved record, never the current operation. Unknown or unsafe-to-display
contexts get an explanation; they are not reconstructed or made compatible.

You can inspect the recorded rates after a refusal and explicitly enter chosen
values in the operation editor. This remains manual authoring, not workshop testing.
Create a new tested preset only after testing the exact current configuration and
providing a fresh explicit attestation. Even a small context difference still refuses;
there is no numeric normalization, automatic migration or renewed test claim.

After copying, the operation shows **Loaded preset**. Editing values shows **Modified
from preset**. Undo/Redo restores the authored state. Editing, duplicating or deleting
the library entry does not change values already copied into a project. A context
change preserves authored values while refreshing validation and generation status.
Generate and complete the required simulation qualification again before export.

## Manage and transfer collections

Open **Presets** to inspect named collections and edit, duplicate or delete entries.
Import/export uses the current `fabcam-preset-collection`, version `1`, JSON format;
maximum size is 2 MiB and maximum count is 256 presets per collection. Import collision
choices are explicit and stored updates detect stale writes. An imported attestation
records somebody's statement; it is not proof of testing on this workshop's machine.
Review the identity, geometry, operation and values before copying.

Project files use version 9. Versions 1–8, old source packs and experimental recipes
are not migrated. Original user files remain untouched. Keep them for historical
reference; they are not current collection imports.

Installed native dialogs and real browser JSON transfer still need the manual
acceptance recorded in [FS-RESET-3 results](releases/FS_RESET_3_RESULTS.md). The
[developer contract](CUTTING_CONTRACT.md) distinguishes saved files from generation
requests and documents safety/export limitations.
