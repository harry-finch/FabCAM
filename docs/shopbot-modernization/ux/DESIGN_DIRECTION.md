# Desktop CAM design direction

The primary visual reference for the CAM is professional Adobe Creative Cloud
desktop software: compact density, contextual inspectors, restrained dark UI,
strong typography hierarchy, subtle separators and progressive disclosure.
This is a design-language reference, not a request to clone Adobe branding or
proprietary components. No Adobe assets, logos or proprietary icons are used.

## Working rules

Reuse app.css theme tokens and op_properties/op-properties.css before local
styling. Keep numeric fields in label / value / unit / status tracks; use tabular
numbers and fixed suffix widths. Ordinary desktop widths keep metadata in the
same row. Below 260 px of inspector content width, move secondary status below
its value without moving or hiding units. Keep full text/tool names accessible
through titles when a compact header truncates them.

The shared rhythm is 4 / 8 / 12 / 16 / 24 px with approximately 28 px controls.
Use small semibold section labels, normal field text and smaller muted metadata.
Keep existing theme colors and visible keyboard focus. Reserve strong accent
for selection, focus and Apply; warnings retain icon/text plus amber. Use subtle
section separators instead of repeated framed cards. A collapsed disclosure
keeps its label and keyboard affordance.

Applied values, proposed values and manual edits must remain distinguishable.
A stale recommendation is compact, actionable and explicitly says the cutting
values were kept; its full explanation remains available as a tooltip. Origin metadata may use a small tinted rectangle with icon and text, as in the
user-requested blue-star Suggested/violet-pencil Modified direction; avoid large
pills or success colors that suggest physical validation. Source
and evidence are secondary; full provenance stays in disclosure. No visual
polish changes matching, rates, persistence, sources or machine output.

Avoid dashboard/card layouts, mobile-size controls, oversized spacing, excessive
rounding, decorative borders, copied Adobe branding or exact proprietary layouts.
A contextual inspector should evolve within existing selection/workspace seams;
full geometry/machine/operation inspector unification is later UX work.
