# V-Carve text demonstration — 2026-10-14

User-requested example, software only; existing human acceptance gates remain open.

- Project: `VCARVE-texte-Test.ivac-project.json`, saved through the UI, reopened and regenerated successfully.
- Toolset: `ShopBot-Vbit-60-reference.ivac-toolset.json`; use Tool library → Load (add) → Apply in another browser installation. Inventory storage is browser-local.
- Generic reference V-bit: included angle 60°, diameter 6 mm, ideal sharp tip 0 mm, two flutes, declared cutting length 5 mm. These are demonstration assumptions, not measurements of the user's cutter.
- Demonstration defaults: 12,000 RPM, feed 600 mm/min, plunge 150 mm/min; not material-qualified cutting recommendations.
- Editable text: Test, bundled DejaVu Sans with embedded font, size 20 mm, origin (10,12).
- Stock 100 × 50 × 10 mm. V-Carve full medial axis, containment-aware source, maximum depth 1 mm, step 0.25 mm.
- Current IAB installation at port 18781: reference tool added to existing inventory without removing its previous 90° tool, stocked as T2 on a new `VCARVE SOFTWARE ONLY — ShopBot post` profile. Project embeds machine and tool for portability.

## Software verification

Current backend port 18792: 18,874 resolved preview moves; generation succeeded before and after saving/reopening. Exported OpenSBP has 18,981 split lines and 18,863 M3 records. Independently parsed M3 bounds: X [9.9911,52.2561], Y [12.047,26.5313], Z [-1,0] mm, within stock XY and maximum depth. Output names tool 2, diameter 6, manual 12,000 RPM and expected 10 mm/s feed / 2.5 mm/s plunge. Diagnostic export retained in `/private/tmp/VCARVE-texte-Test-software-only.sbp`, not supplied as a machine-ready program.

Expected warnings: capped target (material remains), sampled medial spine scope (not complete-area clearing), ShopBot manual spindle / unqualified hardware. No guards weakened, no CAM or post code changed, no physical test.

UX observations: text-only project generates and appears in 3D, but its 2D canvas after reopen does not show the text in the observed narrow viewport; operation warning also misleadingly says no drawing imported. These are separate display/diagnostic issues, not proof of failed text toolpath generation. Follow-up required; not concealed by adding an unrelated SVG.

Evidence: `ux/VCARVE-text-tool.jpg`, `ux/VCARVE-text-preview.jpg`.

## Current strategy wording

Choose **Variable-depth engraving (Full medial axis)** for internal letter/motif
paths whose depth follows local width; Maximum carving depth caps those targets,
which may remain shallower. This is not complete pocket clearing. **Outline groove
(Perimeter)** follows contours at constant final depth and leaves the interior.
Both use the same depth and Step/pass values; switching does not reset them.
Negative Z cuts below stock top;0 retains geometry-limited automatic depth.
[Full strategy mapping and qualified limits](VCARVE_STRATEGY_UX.md).
