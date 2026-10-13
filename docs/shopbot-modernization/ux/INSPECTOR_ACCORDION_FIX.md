# Inspector accordion follow-up — 2026-10-13

User reported Layers closing into Stock, Operations closing into Layers, and
short Layers/Text content stretching to the bottom. Cause: togglePane swapped
active/previous sections and the active grid row consumed all remaining height.

The inspector now allows one expanded section or none. Closing the active
section does not open another; explicit contextual reveals still open the
requested section and remain idempotent. View state only, no project/schema change.
Sidebar hosts use natural content height and shrink/scroll for long content.
Collapse tooltips no longer promise a return to the previous panel.

Actual desktop browser evidence with the existing square software fixture:
- Layers close → all sections closed; Operations close → all sections closed.
- Short Layers height 101 px, next Text top 210 px, Operations top 251 px.
- Empty Text height 95 px, Operations follows at 246 px.
- Short Operations height 95 px; expanded operation scrolls 1017 px inside a
  503 px viewport. Long Stock scrolls 1259 px inside 507 px; sibling headers
  remain accessible.
- Enter closes Text. Canvas fixture selection from the closed inspector reveals
  Stock and its selected fixture through the existing contextual flow.

Six transition regressions cover each section closing/reopening, cross-section
switching, no-open-section state, and repeated explicit reveals. Full frontend:
119 files, 1107 passes / 1 optional skipped; check zero errors/warnings, lint and
build pass. No native CAM, collision/export, FS or persistence behavior changed.

[Compact inspector screenshot](UX2_1-sidebar-compact.png). Current browser preview
serves the updated build; save the current project before reloading to use it.
CAM3 remains unstarted; UX2.1 human gate remains ivac-svnf. Engineering ivac-16yy.
