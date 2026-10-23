# QUAL-1 limited UI review — 2026-10-23

Real in-app browser, fresh disposable origin
`http://qual1-review.localhost:5173/?api=wasm`. The maintainer restarted the existing
web server when requested; it is left running. Page startup identity is
`v0.4.0-111-g932ee4f-dirty`, with the original user's two blue-accent CSS changes.
The only QUAL-1 production delta is native CPS cancellation polling; core geometry,
WASM-default cutting behavior and frontend UX are unchanged. This is a dev-browser
review, not installed-native or three-platform acceptance.

No injected application state, fixture file upload, project download, permission
bypass, private collection mutation or native dialog claim. File transfers remain
NOT TESTED under the earlier explicit browser approval denial.

Synthetic project: built-in DejaVu `O`, 50 mm, plain source; MDF; T1 3 mm endmill;
Pocket `QUAL-1 SYNTHETIC Pocket`; Z -1 mm, pass 0.5 mm; rough 12000/600/120 and
finish 10000/450/90 (RPM/feed/plunge). These are software inputs, not recommendations.
Collection `QUAL-1 DISPOSABLE QA — NO MACHINING`; preset
`QUAL-1 SYNTHETIC — NOT PHYSICALLY TESTED`. The attestation checkbox is exercised
solely as an explicitly labelled synthetic test, not an assertion of real machining.

| Flow | Observation |
|---|---|
| New project / geometry | Fresh empty origin, new project and bundled text added through UI |
| Rough and finish | Separate selectors and independent values; report confirms both actual roles |
| Material prerequisite | Save preset initially refuses: select exact Stock material; MDF satisfies it |
| Attestation | Save disabled before explicit checkbox; separate software-validation disclosure |
| Collection save | New named collection created; saved preset shown |
| Reviewed load | Preview shows both role triplets and pass/overlap; Replace parameters copies values |
| Loaded / Modified | Loaded preset shown; setting rough feed to zero changes status to Modified |
| Work origin | Initial WCS outside geometry produces critical warning and disables machine export; suggested origin fixes it |
| Generation / simulation | 14829 moves / 14612 lines; full-program simulation complete; download becomes available, not clicked |
| Actual report | Rough 12000/600/120, finish 10000/450/90, generated min Z -1.000 mm, T1, MDF |
| Invalid parameters / stale result | Feed zero visibly rejected; generation fails, old output stale, download disabled |
| Undo | Restores 600, Loaded preset and a freshly qualified program |
| Drill / Spot / hole Chamfer | Temporary Drill inspector shows separate Drilling / Spot drilling / Hole chamfer roles. Added secondary roles have blank fields and explicit positive-value errors; endmill chamfer refuses invalid V-bit geometry; no silent substitution. This is authoring/refusal coverage, not successful three-tool generation |
| Preset details | `Author attestation: tested. Not proof of testing on the current workshop machine.` visible beside both role values. No actual import performed |
| Save/open v9 files; collection import/export | NOT TESTED; use MANUAL_ACCEPTANCE.md |

The temporary Drill operation was removed using the ordinary reversible UI action;
only the disposable Pocket and QA collection remain. No physical output was sent.
Full positive actual-tool Drill/Spot/Chamfer generation is covered by the fixed corpus
and Rust/frontend contract tests, not asserted from this limited UI walkthrough.

## Readability findings — future work only

Inspected desktop 1280×720 and requested narrow CSS viewport 760×900. Read-only DOM
measurement confirms width 760, height 900, document scrollWidth 760. The report
requires vertical scrolling; values wrap without document-wide horizontal overflow.
Mandatory warning text and invalid/stale states remain reachable. No presentation
change is justified under QUAL-1's warning-concealment exception.

Small proposals for a later approved UX milestone:

- Put each actual cutting role on its own row with aligned RPM/feed/plunge columns;
  the current single cell mixes both roles, units and long labels.
- Give long warning prose a short operator-facing lead and retain the full explanation
  in details. Keep critical warnings visible and never hide the required action.
- Move technical identifiers such as `CUTTING_LIMITS_UNKNOWN` into secondary details,
  preserving a copyable diagnostic code. Use machine/tool names ahead of long IDs.
- Remove duplicate role prefixes such as `Rough: Rough:` / `Spot: Spot:` in errors.
- At narrow widths use a role-labelled stacked value layout. Keep report warnings
  and export/freshness state visible while scrolling.

No report redesign was started. Screenshots: `ui/report-values.png`,
`ui/report-narrow.png`, `ui/blocked-origin.png`, `ui/invalid-stale-refusal.png`,
`ui/secondary-role-errors.png`, `ui/attestation-disclosure.png`.
