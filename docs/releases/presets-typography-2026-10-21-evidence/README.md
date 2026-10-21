# PRESET-3 typography follow-up — 2026-10-21

Branch FabCAM-v1.1, HEAD5ca5632 plus preserved existing local work; task ivac-pnpih.
User requested matching dialog typography to the compact interface.
Only CSS changed in PresetBrowser, CuttingParameterComparison, PresetOperationPicker
and FeedsSpeedsSection: shared field/meta font tokens, moderate label weight,
tabular numerals and shared spacing. Templates, data and application logic unchanged.

Isolated browser port18813 / WASM, synthetic project derived from existing software
fixture, no Apply or toolpath generation. Both Recommended and Browse inspected.
Computed styles: values/labels12.48px; table headings/filter labels/meta11.2px.
[Recommended](recommended.png) · [Browse](browse.png).

Frontend1498 PASS / one existing locale skip, 170 files; WASM guards10 PASS.
Typecheck0errors/0warnings; scoped ESLint/Prettier PASS. Component-size guard PASS,
FeedsSpeedsSection985/1000; no budget changed. Receipts alongside this file.
No new tests for reversible typography. No build/release or physical qualification
claim. No commit/push for this follow-up; existing implementation/evidence preserved.
