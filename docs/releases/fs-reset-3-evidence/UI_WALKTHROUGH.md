# FS-RESET-3 disposable software UX walkthrough

2026-10-22, in-app browser, existing local Vite server at
`http://fsreset-source.localhost:5173/?api=wasm`. Real current WASM adapter, no native
installation and no hardware. Main walkthrough used `ded2348`; the only subsequent
software change, `64d7584`, corrects three machine-limit tooltips (English/German).
Machine-limit walkthrough repeated on that final software source. The dev-server title
contains its earlier startup build label, not an installed final-source identity claim.

Main project: built-in DejaVu text “O”, 50 mm, plain geometry, then a Pocket operation
named `FS-RESET-3 QA SYNTHETIC Pocket`; MDF; depth -1 mm/pass 0.5 mm; T1 3 mm.
Rough 12000 RPM / 600 then 610 mm/min / plunge 120; Finish 10000 / 450 / 90.
These values are synthetic software inputs, **not machining recommendations**.
Collection: `FS3 DISPOSABLE SYNTHETIC QA — NO MACHINING`.
Entry names and notes explicitly state `SYNTHETIC QA` / `NOT PHYSICALLY TESTED`.
The explicit attestation UI was exercised only within the user-authorized disposable
synthetic QA scenario; this does not assert any genuine workshop test.

| Requested step | Observed result | Evidence/limit |
|---|---|---|
| 1 Create operation | PASS: built-in text then Pocket; second project PlanarSurface | No external import needed |
| 2 Direct RPM/feed/plunge | PASS: saved values shown in actual generated report | `ui/report-rates.png` |
| 3 Separate rough/finish | PASS: 12000/610/120 and 10000/450/90 | Actual-role report, no global block |
| 4 Save and reopen file | NOT TESTED in browser | Browser file-transfer approval denied; automated v9/lifecycle round trips pass |
| 5 Named collection | PASS: new explicitly synthetic collection | `ui/synthetic-attestation.png` |
| 6 Explicit attestation | PASS within synthetic-only test | Missing material initially blocked with precise reason; exact MDF enabled capture |
| 7 Inspect/load compatible | PASS: reviewed both role values, then Replace parameters | Loaded preset observed |
| 8 Modify/status | PASS: Rough 600→610 produced Modified from preset | `ui/modified-preset.png` |
| 9 Undo/Redo | PASS: Undo restored 600 + Loaded; Redo 610 + Modified | Existing command history |
| 10 Edit/duplicate/delete | PASS on disposable entry only | Renamed, added synthetic-only notes, duplicated, deleted original; QA copy remains |
| 11 Deleted entry isolation | PASS: operation retained 610 and 10000/450/90 | Snapshot retained original name/values |
| 12 Tool/material/machine context | PASS for tool diameter 3→3.2 and MDF→Birch: preserved authored rates, refused preset copy; Undo restored context. Machine limit changed on second project: saved rates preserved and refusal updated | `ui/context-refused.png`; machine edit did not exercise loaded-preset status itself |
| 13 Warnings/blocking | PASS: unknown-rating disclosure; zero feed rejected; known max feed 500 rejected authored 600 without reducing it | `ui/machine-limit-refused.png` |
| 14 Generate/qualify | PASS: 14829 moves, 14612 lines, full-program simulation complete, download enabled | `ui/qualified-program.png`; no download or machining performed |
| 15 Change/stale export | PASS: feed edit invalidated qualification and disabled export; zero feed then refused generation | `ui/stale-export-refused.png` |
| 16 Machining report | PASS: actual distinct role rates and -1 mm minimum depth | `ui/report-rates.png` |

Desktop and narrow layouts were inspected, not every 16-step flow repeated twice.
At requested viewport 760×900, responsive drawers exposed finish selection/status and
manual values; the final limit screenshot shows 12000/600/120 and refusal legibly.
`ui/narrow-finish.png` records drawer/modified/stale state, not a full visible rate table.
`ui/narrow-limit-refused.png` is the directly inspected narrow parameter capture.
Viewport was reset afterward. No redesign was undertaken.

A development HMR reload after the tooltip correction discarded the disposable unsaved
project; a second blank project used built-in PlanarSurface to finish known-limit checks.
This is not evidence of saved-file corruption or a successful reopen. Browser numeric
`fill` alone did not commit onchange fields; native keyboard changes/blur and the engine's
reported values were used to confirm the final 600/120 refusal. DOM field text alone was
not counted as persisted-value evidence.

Automatic browser approval review rejected the attempted local SVG file upload with an
explicit user-declined decision. No alternate upload, injected state or browser bypass
was used. Saved-project browser transfer and collection import/export remain NOT TESTED.
The prior browser download limitation also remains open. Static ACL/mock file tests do
not replace installed native dialogs. The original local web server was left running;
no app-data reset or other user's data deletion occurred.
