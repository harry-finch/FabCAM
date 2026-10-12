# FS3.6 — Adobe-inspired UI polish qualification

Date: 2026-10-12. Engineering issue: ivac-n23x. Human approval: ivac-vgu4.
FS3.5 functional acceptance is explicit in the user's FS3.6 request. FS3.6
engineering is ready for human visual review. FS4 is unstarted.

The initial report below records the first handoff. The user-requested review
revision and its additional API diagnostic scope are recorded in the final section.

## Visual problems identified

FS3.5 rates/status occupied separate lines at ordinary desktop widths; large
stale cards, nested fieldset/disclosure frames and uniform heading weights
competed with current cutting values. Operation name and cutter had similar
emphasis. Existing colors, radii, focus styles and panel patterns already supplied
a useful foundation. Machine tooling, Tool Library and top navigation were
inspected; their larger structural differences are deferred rather than redesigned.

## Shared style/design changes

Audited shared styles before component edits. Added only a small token layer to
frontend/src/app.css: 4/8/12/16/24 px spacing, 28 px inspector controls, field/meta
sizes and fixed value/unit/status widths. Existing theme, border, radius, accent,
warning and focus tokens are reused. Shared op_properties/op-properties.css
owns field grids, numeric suffix widths, compact actions and section/disclosure
separators. Below 260 px of inspector content, metadata follows its value and
shared legacy property rows reserve numeric/unit space. This is a desktop panel
fallback, not a mobile redesign.

## Components modified

- FeedsSpeedsSection: shared rows, quiet Modified/Suggested text, Pass depth
  label, small section headings, compact kept-values/stale/source group,
  neutral Calculate/Recalculate/Cancel and restrained primary Apply.
- OpPropertiesPanel: reduced embedded indentation using shared spacing.
- OperationsList: operation name primary, cutter secondary, existing warning/
  duplicate/delete actions retained; full names accessible through titles.
- Shared property CSS: controls, section separators and progressive disclosures
  reused across operation inspectors. English/German strings and generated keys
  updated for the three presentation labels.

Handlers, matching, ownership, persistence and geometry/output paths are unchanged.
No new recommendation data or engine behavior was introduced.

## Before/after layout summary

Before: repeated parameter/status lines, large state block and nested boxes.
After: at a 360 px sidebar, four aligned tracks; numeric controls and rows are
28 px with 4 px gaps. Units share the same x-coordinate and statuses do likewise.
At 240 px, values/units remain on their main row and statuses move beneath them.
Labels wrap at word boundaries where possible. Stale state uses icon/text plus
Recalculate and a small kept-values line; full explanation remains its tooltip.
Provenance is secondary and expandable; proposed values remain explicitly not
applied. Advanced depth and finishing controls remain disclosures.

[Before: FS3.5 stale inspector](../feeds-speeds/screenshots/FS3_5-manual-stale.jpg).
The comparison is an earlier retained checkpoint capture, not a recreated image.

| Final capture | Verified scenario |
| --- | --- |
| [Profile stale](../feeds-speeds/screenshots/FS3_6-profile-stale.jpg) | 360 px panel; Suggested spindle/plunge, Modified feed/pass depth, compact stale state |
| [Proposal](../feeds-speeds/screenshots/FS3_6-profile-proposal.jpg) | Suggested values not applied; clear Apply priority, quiet provenance/Cancel |
| [Pocket large values](../feeds-speeds/screenshots/FS3_6-pocket-large.jpg) | 250000 / 99999 / 9000 synthetic manual values; coherent stepover group |
| [Pocket narrow](../feeds-speeds/screenshots/FS3_6-pocket-narrow.jpg) | Minimum 240 px panel; readable values/units and metadata fallback |
| [Advanced depth](../feeds-speeds/screenshots/FS3_6-pocket-advanced.jpg) | Expanded depth/entry controls, units and focus |
| [Finishing](../feeds-speeds/screenshots/FS3_6-pocket-finishing.jpg) | Expanded finishing controls and long cutter description |
| [Operation header](../feeds-speeds/screenshots/FS3_6-operation-header.jpg) | Name/cutter hierarchy and ellipsis, full-name titles |

Pocket fixture: [FS3_6-visual-pocket.ivac-project.json](../feeds-speeds/FS3_6-visual-pocket.ivac-project.json).
This is software-only synthetic layout stress data, deliberately outside realistic
cutting rates. It is not a preset or machining instruction. It has manual origins
and no reference proposal. No Pocket reference transfer or evidence was fabricated.

## Accessibility impact

Associated native labels/inputs remain; status is readable text, stale state has
icon/text and role=status. Units remain visible. Existing 2 px blue keyboard
focus outline retained; Tab from Feed reaches Plunge as expected. Primary action
uses darkened existing accent with strong text. Muted metadata retains existing
#A0A0A0 dark-theme text color. Controls are 28 px; disclosure targets remain at
least 24 px. Long header names truncate visually but retain full titles/accessibility
text. No new tab-order interception or modal flow. This is a focused keyboard/
DOM/visual review, not a full assistive-technology or all-theme certification.

## Tests performed

- Frontend Vitest: 110 files, 1,065 tests pass, one existing optional skip.
- Svelte/TypeScript: zero errors and zero warnings.
- Targeted ESLint on changed Svelte components and Prettier on changed styles,
  components and English/German/generated translations: pass.
- Production frontend build with existing fresh WASM: pass. Git whitespace: pass.
- Real browser at the existing 1280 × 720 desktop viewport: Profile and Pocket
  loaded; sidebar resized through its real splitter to 240 and reset to 360 px.
  DOM confirms no horizontal inspector overflow at 240 px, including expanded
  depth. Ordinary cutting rows measure 28 px and identical suffix positions.
- Real calculation at DOC 2.8 mm still proposes 17000 rpm / 1620 mm/min /
  810 mm/min. Current manually edited Feed stays 1500 until explicit Apply.
  Source/provenance disclosure exposes the original license/link/snapshot.
- SHA-256 comparison of 437 files under crates, data, schema and frontend
  state/API/services against the pre-FS3.6 snapshot: zero changes. Durable
  [snapshot manifest](FS3_6-protected-sha256.json) retained; paths are relative to
  repository root. This preserves the already-qualified FS3.5 implementation.

No new mirror/CSS unit tests were added; real browser inspection addresses layout
where existing unit tests cannot. No new native bundle, physical machining,
Rust rerun or Windows/Linux qualification claimed for this visual-only checkpoint.

## Documentation updated

STATUS records current gate, DEVLOG records changes/evidence, ROADMAP inserts
FS3.6 without renumbering milestones. [DESIGN_DIRECTION](DESIGN_DIRECTION.md)
records professional Adobe Creative Cloud desktop principles as design-language
reference, without branding, proprietary icons or exact layout cloning.

## cam-ux-ui changes

Preserved the existing Adobe reference and extended concise skill guidance to
avoid dashboard/mobile/large-card styling, excessive rounding/spacing and
ornamental borders. Added shared-token reuse and preservation of units, labels,
focus and text/icon states. Linked the detailed persistent direction document.

## Remaining UX debt

- ivac-dqax: broader contextual inspector unification and incremental shared
  density across Machine/Tool Library/stock/geometry workspaces.
- ivac-703e: clarify stock offset anchor versus absolute corner and WCS; user
  feedback belongs to a separately approved semantic UX change.
- ivac-evl0: explain no-match reasons, especially the exact 2.8 mm DOC reference;
  preserve strict compatibility rules.
- Existing ivac-xz59 generation/export wording and ivac-xf9w source isolation
  remain separate follow-ups.

## FS4 readiness and human review

Review the screenshots or run the existing preview and load the FS3.5 software
fixture. Confirm normal panel density, field alignment, stale action visibility,
proposal/current distinction and narrow fallback. Human visual acceptance is
pending; no physical validation or calibrated preset is implied by FS3.6.

**STOP — waiting for human approval before starting FS4.**
Changes remain uncommitted; no new publication was authorized.

## Review revision — stock frame, rejection explanation and origin vignettes

User explicitly authorizes stock-frame clarification, precise no-recommendation
reasons and style revisions while retaining the FS3.6 gate (ivac-vgu4).
Engineering: ivac-703e, ivac-evl0, ivac-3d53.

Stock fields now say **Stock translation**. Mode-specific help explains their
initial placement anchor; live readouts show the lower-left drawing coordinates
and those relative to WCS zero. The existing computeFootprint result is reused,
with no new placement formula or change to WCS/stock persistence. For the
100 mm square at bounds 10..110, manual stock 500 mm with zero translation has
corner -190,-190. Native UI edits verified this; shifting WCS X by 0.5 changes
only the work-relative X to -190.5.

The Advanced depth and entry summary now uses the shared 11.2 px metadata size,
8 px top separation and a 24 px keyboard/click target. Suggested/Modified use a
small tinted rectangular vignette, respectively blue/star and violet/pencil;
text remains visible and tooltips explain saved origin without implying physical
validation. Dedicated light/dark tokens preserve readable contrast. The shared
status track is 80 px; real 240 px panel has no horizontal overflow, and Enter
still expands/collapses the disclosure.

For no-match, the native scope predicate now supplies the same refused criteria
it uses to decide compatibility. This small additive response extension was
reviewed with ivacam-architecture and cam-validation: FsCamResponse optionally
includes scope_rejections, omitted when empty and defaulted for older responses.
The pure engine explains up to three closest rejected scopes (minimum mismatch
count, identity-sorted), explicitly not recommendation candidates. This ranking
is diagnostic only. Matching, precedence, numerical calculation, source data,
applied/project formats and posts are unchanged; the frontend does not duplicate
matching rules. No significant boundary decision requiring a new ADR.

UI translates engine reason kinds and displays requested/reference values and
units for each failed criterion. DOC 1 mm now reports exactly **Requested: 1 mm /
Reference requires: 2.8 mm**, followed by kept-values/no-extrapolation text.
Old refusal explanations are hidden when the calculation context becomes stale.
The source catalog captured with the response supplies readable identity labels.

Validation of this revision:

- Frontend: **1,068 pass / one optional skip**, 111 files; three presentation
  tests cover exact/ranged values, identity revision lookup and unknown engagement.
- Pure engine/data: **50 tests + one doctest** pass, including native DOC,
  machine and operation rejection reasons; existing 38 policy/math cases pass.
- Core adapter: **11** targeted tests pass, including transient rejection
  immutability and old-response deserialization; ShopBot M2 goldens **8** pass,
  CPS ShopBot guards **5** pass.
- Svelte/TypeScript zero errors/warnings; targeted ESLint/Prettier, cargo fmt,
  library Clippy with warnings denied, schema-check and final WASM/frontend
  production build pass. Cached matching wasm-bindgen/wasm-opt were used for
  the final offline rebuild after the toolchain's partial-output guard issue.
- Against the earlier 437-file protected snapshot, only the five intended engine/
  adapter/test files, generated schema and generated API types changed. Stock
  footprint, cutting data, persistence, geometry and post files remain identical.
  The earlier zero-change statement applies to the initial visual handoff only.
- Real browser verifies exact DOC refusal, 360/240 px vignettes, disclosure
  keyboard/spacing and actual stock/WCS readouts. No native bundle/physical run.

Captures: [styles](../feeds-speeds/screenshots/FS3_6r-statuses.jpg),
[precise rejection](../feeds-speeds/screenshots/FS3_6r-no-match.jpg),
[stock frame](../feeds-speeds/screenshots/FS3_6r-stock-origin.jpg),
[narrow](../feeds-speeds/screenshots/FS3_6r-narrow.jpg).

Stock-frame and no-match debt ivac-703e/ivac-evl0 is resolved within this authorized
revision. Broader contextual inspector ivac-dqax remains deferred. No commit or
push; **STOP at FS3.6 human approval, before FS4**.


## Focused review — progressive calculation context (2026-10-12)

User-authorized UI-only refinement (ivac-rncu): the default assistant shows
material, Profile full-width confirmation or Pocket nominal radial engagement,
cutting mode, compact source/evidence and an explicit calculation action.
Machine, physical cutter geometry/flutes, operation and effective maximum pass
depth move into the closed-by-default **Calculation context** disclosure.
The top-level editable pass depth remains authoritative; proposed pass depth is
still visible in the unapplied result because it is part of the proposed change.

Existing recommendations use **Recalculate cutting parameters / Recalculate**;
new recommendations use **Cutting recommendation / Calculate recommendation**.
The review opener disappears while the assistant is open. Accepted source
metadata is labeled **Previous source** until a fresh compatible result provides
its source/evidence. Before an initial calculation, enabled source names are
shown with **Evidence after calculation**, rather than implying an applicable
profile has already been selected. Detailed provenance remains secondary and
collapsed for accepted/proposed recommendations. Multiple-source results retain
source labels to identify each Apply action. Mode uses two inspector columns.

Validation: frontend **1,068 pass / one optional skip**, 111 files;
Svelte/TypeScript zero errors/warnings; full ESLint/Prettier and production build
pass. Browser verifies default collapse, keyboard expansion, new Pocket heading
and 50% nominal engagement without redundant confirmation, and 360/240 px layouts
(no horizontal overflow: narrow region clientWidth = scrollWidth = 174 px).
Compatible recalculation retains manual feed **1,500 mm/min** while proposing
**17,000 rpm / 1,620 mm/min / 810 mm/min**; Apply remains a separate action.
Protected-file audit still has only the seven previously documented changes:
no additional engine, data, adapter, domain or persistence changes in this review.
No native rebuild or physical trial; FS4 remains unstarted.

Captures: [default](../feeds-speeds/screenshots/FS3_6-context-collapsed.png),
[expanded](../feeds-speeds/screenshots/FS3_6-context-expanded.png),
[narrow](../feeds-speeds/screenshots/FS3_6-context-narrow.png),
[new Pocket state](../feeds-speeds/screenshots/FS3_6-context-pocket.png).
The Pocket capture precedes the final two-column mode alignment; the other three
show the final alignment. STOP at the existing FS3.6 human gate (ivac-vgu4).
