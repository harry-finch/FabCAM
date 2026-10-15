# Geometry source & contour interpretation UX — 2026-10-14

Engineering ivac-uebs is ready for human review **ivac-xy3w**. This is the
requested shared-inspector refinement, not a new CAM milestone. The independent
V-Carve final acceptance **ivac-1b75** remains pending; its numerical qualification,
scope, inlay restrictions and physical-evidence limits are unchanged.

## Existing source-selection architecture

**KEEP:** imported segments, chaining/containment, selectors, booleans, machine-
independent toolpaths, origin handling and ShopBot output.
**EXTEND:** presentation in `GeometrySourceSection.svelte`, using shared inspector
CSS and existing `project.updateOperation` commands. `OpPropertiesPanel.svelte`
now delegates this section, falling from 1248 to 1110 lines. No new styles,
framework, wire type, schema or migration; no ADR needed for this local extraction.
**DEFER:** authoritative temporary highlighting (ivac-9oty).

An import entry is a drawing/file with its own layout transform. It produces flat
segments, string layer names, per-segment object IDs and object metadata. The
pipeline re-chains the sent segments into contour objects. One contour can consist
of many segments; a layer can contain many contours. These are not file/group IDs.
`combineImports` offsets imported object IDs and merges layers **by name across
files**. Editable text uses synthetic `__text_<id>` layers. `geometryView` also
injects the visible stock outline on `__stock_outline`; text is sent separately
through the existing project builder. Visibility is not the source selector.

Authorities: `state/file-transform.ts`, `state/stock-outline.ts`,
`state/project.svelte.ts`, `api/build-project.ts`, core `input.rs`,
`cam/chaining.rs`, `pipeline/selection.rs`, `pipeline/offset_builder.rs`,
`cam/source_combine.rs`, `pipeline/regions.rs` and the V-Carve/Halfpipe drivers.
The new helper only describes authored fields and produces explicit command
patches; it computes no contours, containment, regions, islands or boolean results.

## Actual source modes and persistence

| UI label | Existing wire meaning |
| --- | --- |
| All available geometry | `kind: all`. Every available chained object, including rendered text and the visible stock outline. Always Auto; a retained `sourceCombine` field has no wire effect here. |
| Specific layers | `kind: layers`, unchanged layer-name array. Matches names across drawings, rather than a selected file. |
| Selected geometry | `kind: objects`, unchanged ordered ID array. Saved binding, not live canvas selection. A nonempty ID list takes precedence over layers. |

Important legacy behavior: empty `sourceLayers`/`sourceObjects` fall through to
All in `buildSource`. A saved empty layer binding now states this explicitly; it
is neither silently corrected nor advertised as an empty machining selection.
Missing layer names remain selectable as “unavailable”; missing IDs remain saved
and are left to the existing generation diagnostics.

Choosing a mode with no remembered binding stages an **editor-only** choice;
the current operation remains unchanged, and the inline message identifies the
still-active source. Choosing a layer or using “Set sources from selection” then
binds explicitly. Previous nonempty layer/ID bindings are remembered while this
inspector instance remains mounted, allowing a switch back without discarding them.
This memory is not persisted, survives no reopen and is not a second project
model. Explicit All uses the original `null`/absent representation. Every effective
source change is one existing undoable command, replacing the prior two separate
field patches. Undo restores the full previous binding.

The compact layer picker still **authors one layer at a time**. Saved multiple-
layer arrays are displayed together and preserved until an explicit replacement;
it does not pretend to be a new multi-layer authoring widget. Its option values
are JSON-encoded presentation values to avoid collision between arbitrary layer
names and a “saved layers” sentinel; the persisted names remain unchanged.

## SVG/DXF layer representation

- SVG importer walks/transforms the `usvg` tree into one synthetic **svg** layer.
  SVG/Inkscape groups, element IDs and fill rules are not exposed as CAM layers.
  The UI says “SVG geometry · filename” or “SVG geometry · N drawings” when the
  contributors really are SVG imports; its tooltip preserves the raw layer ID
  and lists drawing sources.
- DXF preserves resolved named entity layers; import options can filter layers
  or form color-layer names. The picker displays the actual resolved name and
  contributing drawing, without changing it. A DXF layer also named `svg` does
  not get an SVG-only label when mixed with SVG imports.
- Equal layer names in separate imports are one shared source name. Filenames
  shown beside them provide provenance, not file-scoped selection.
- Text remains a named text source. The existing selectable stock outline now
  has a readable “Stock outline” option and its own synthetic-layer tooltip.

No importer, coordinates, transform, group hierarchy or layer ID changed.

## Actual contour interpretation

“Source” becomes **Geometry**, “Mode” becomes **Use geometry from**, and “Combine”
becomes **Contour interpretation**. All six persisted enum values remain available
where region interpretation is consumed. Labels and help are translated in EN/DE.

| Value / UI | Actual behavior |
| --- | --- |
| `auto` / Automatic — nested contours | Shared region kernel ranks containment depth within selected closed contours: even depths are regions, odd depths holes. V-Carve and Halfpipe use this kernel. Normal Pocket Auto instead skips contours inside another selected contour and preserves selected nested closed contours as islands; its help is consumer-specific. |
| `union` / Union | Clipper2 closed-contour union with NonZero winding and recovered holes. |
| `difference` / Difference | First eligible closed contour minus the rest, with NonZero winding. Objects preserve saved ID order; layers walk engine object order, **not layer-array order**. The help exposes this distinction; object sources show the first saved ID with an explicit “if closed and present” qualifier. |
| `intersection` / Intersection | Successive closed-contour intersections, followed by hole recovery. |
| `xor` / Exclusive areas (XOR) | Successive symmetric differences. Existing implementation ends when an intermediate result is empty; help states that limitation rather than promising arbitrary N-way parity. |
| `none` / Separate contours | Each closed contour forms its own region, without holes. Nested interiors may also be machined. |

Region combination filters out open contours and invalid object indices. Profile
and Engrave can use open paths through their existing path drivers; this change
does not remove those sources or apply a new region interpretation to them.
Containment classification uses tessellated polygons, bounding-box rejection and
point-in-polygon tests on a representative contour probe. SVG fill/group metadata
does not decide Auto nesting. This is existing numerical behavior, not a new
whole-polygon containment guarantee for overlapping/self-intersecting inputs.

Two separately tracked findings are deliberately unchanged:

1. **ivac-dli7:** three-level nesting can differ between normal Pocket Auto's
   emitted geometry and the parity-based generated region preview. The ordinary
   outer/inner case is already regression-tested; deeper nesting needs engine
   investigation and independent qualification. No silent parity fix here.
2. **ivac-j2po:** XOR terminates on an empty intermediate result. Three identical
   contours therefore return empty. A new characterization test documents the
   current behavior without turning it into a mathematical correctness claim.

## Operation-specific behavior and contextual help

Region controls appear for Pocket (including Halfpipe) and V-Carve with explicit
layer/object bindings, even if only one closed contour is bound. All uses Auto and
does not display an ineffective combination selector. Pocket Outside fixes
frame-minus-selection in the driver, so it shows that explanation instead.

Profile, Engrave, DragKnife, Chamfer, Thread, T-slot and Dovetail retain individual
path semantics. Drill uses source objects individually for hole placement. Their
saved combination fields are preserved while the region control is hidden.
An existing Auto helix fit can still consult combined regions for entry fitting;
hiding the path-operation control does not rewrite an authored setting or claim
that it has no possible effect on entry fitting.

Planar Surfacing keeps its explicit rectangular zone UI. Relief/Waterline/Raster
keep their existing heightfield/STL/image controls. Program-only operations keep
their existing inspectors. No generic source section was added to these workflows.

The selected interpretation has one concise description linked by
`aria-describedby`. Source/layer provenance and saved ID order are in tooltips.
Labels use existing compact grid spacing; controls retain native select behavior.
Keyboard propagation stops at these selects so parent operation-list navigation
cannot collapse the inspector while editing. No new diagrams or explanatory
controls: a generic diagram would hide the consumer/order differences just audited.

## Highlighting decision

Existing `opIncludesObject` and Scene3D overlay coloring can identify imported
objects using authored selectors. They do not expose authoritative resolved
sources after re-chaining, text/stock insertion, patterns or frame synthesis.
Generated Pocket region overlays are area previews, not a fresh cross-operation
source API; the deep-nesting finding also prevents treating them as a universal
source truth. No temporary hover highlight or frontend boolean/source resolver was
introduced. **ivac-9oty** records the separate follow-up. Existing canvas selection,
overlay visibility and operation colors remain unchanged.

No resolved contour/region/island counts were invented. Displayed ID count means
**saved binding entries**, not validated resolved geometry; generation diagnostics
remain authoritative for invalid sources.

## Validation and compatibility evidence

- Frontend: **1260 passed, 1 skipped, 138 files**; 11 new source-presentation tests
  exercise wire precedence/fallback, source switching, all six combine values,
  missing references, SVG/DXF/multi-import name matching, saved multi-layer arrays,
  ordered IDs, one-command Undo/Redo, JSON save/duplicate preservation and consumer
  applicability. Existing operation-template and source/history tests also pass.
- `npm --prefix frontend run check`: **0 errors / 0 warnings**.
  Lint and build pass; existing build chunk/dynamic-import notices remain.
- Core: **7 new self-contained audit fixtures** pass, including actual SVG group
  flattening and actual ASCII DXF named-layer/filter imports (no optional corpus
  dependency), Difference order, selected-depth nesting, None holes, open/missing
  filtering and the unchanged XOR limitation.
- Existing core suites: source_combine **9**, selection **9**, offset_builder
  **54** (including Profile/Pocket/open/island/frame regressions), input **41
  reported passes**. Two DXF corpus tests return early because optional
  `refs/viaconstructor/tests/data/{simple,all}.dxf` is absent; the self-contained
  DXF test provides real layer-import coverage rather than counting those skips
  as exercised corpus cases.
- V-Carve qualification **10**, Halfpipe **15**, ShopBot guards **7** pass.
- **120 protected production files are byte-identical** to the start-of-checkpoint
  snapshot: core/CPS Rust, bundled posts, project-wire builder and file-transform
  authority. [Hashes](geometry-source/CORE_HASHES.json),
  [comparison](geometry-source/CORE_COMPARISON.json).
- Actual browser-generated OpenSBP programs are **byte-identical** to the preceding
  strategy qualification for both V-Carve strategies: Outline 130 moves,
  variable-depth 5862 moves. [Exact output comparison](geometry-source/OUTPUT_COMPARISON.json).
  No golden changes or numerical geometry/parameter changes.

Browser actions also verify an unbound mode leaves the current source unchanged,
All → remembered layers, source Undo/Redo, selecting the DXF layer, undoing a
single-layer replacement back to two layers, duplication, real Save/download and
reopen. The [saved fixture](GEOMETRY-software-sources-saved.ivac-project.json)
retains all five original operations exactly and the duplicate's two layers and
saved Difference value. Template instantiation continues to explicitly rebind
geometry, with existing automated coverage; a separate new template popup exercise
was not performed here.

Enabling the unavailable-layer test operation yields `op_source_missing_layer` and
critical `op_source_empty`; Download is refused by the existing export guard.
The test operation was disabled again; no safety settings were changed. The valid
Pocket reference generates 1535 moves with only the existing CPS/operator warning.

Commands: frontend `test`, `check`, `lint`, `build`; offline Cargo core with
`--features cps`, `--test geometry_source_ux`, `source_combine --lib`,
`pipeline::selection --lib`, `pipeline::offset_builder --lib`, `input:: --lib`,
`--test vcarve_qualification`, `halfpipe --lib`; CPS `--test shopbot_guards`.

## Visual evidence and remaining human checks

Actual screenshots at **1280×720** and **1366×768**, with approximately **280/360 px**
sidebars, use the existing software-only fixtures. Narrow DOM measurements confirm
no Geometry fieldset horizontal overflow or label/control overlap. Long select
values can still truncate on narrow panels; full choices and provenance remain
available in the native menu/tooltip.

| Capture | Evidence |
| --- | --- |
| [A](ux/GEOMETRY-A-vcarve.png), [B](ux/GEOMETRY-B-unbound.png) | V-Carve source/help and unbound mode with preserved active source |
| [C](ux/GEOMETRY-C-pocket-nested.png), [M](ux/GEOMETRY-M-pocket-preview.png), [N](ux/GEOMETRY-N-desktop.png) | Pocket nested contours, generated existing region overlay and desktop layout |
| [D](ux/GEOMETRY-D-source-focused.png), [E](ux/GEOMETRY-E-contour-focused.png) | Focused native selects; DOM verifies all options, **popup window is not captured** |
| [F](ux/GEOMETRY-F-difference-layers.png), [H](ux/GEOMETRY-H-difference-objects.png) | Layer versus ordered-object Difference help |
| [G](ux/GEOMETRY-G-profile-multiple.png), [P](ux/GEOMETRY-P-dxf-layer.png) | Saved multi-layer Profile and actual DXF layer choice |
| [I](ux/GEOMETRY-I-unavailable.png), [J](ux/GEOMETRY-J-empty-binding.png) | Missing layer identity and truthful legacy empty-selector fallback |
| [K](ux/GEOMETRY-K-source-diagnostics.png), [L](ux/GEOMETRY-L-export-refused.png) | Existing missing-source diagnostics and critical export refusal |
| [O](ux/GEOMETRY-O-narrow.png) | Narrow inspector without overlapping controls |

Tab traversal reaches the layer control and keeps the inspector open. **Human
review must still check actual expanded native menus, native tooltip readability
and arrow-key option navigation**, because the IAB screenshot does not include
the native popup. These focused captures do not claim expanded-dropdown visual
acceptance. No physical machining or new native-app qualification was performed.

Documentation updated: this operation-source/CAM UX report, STATUS, DEVLOG and
`ux/DESIGN_DIRECTION.md`. ROADMAP and V-Carve qualification scope are unchanged.
No commit, push, merge, release or next milestone.

**STOP — waiting for human review ivac-xy3w; V-Carve gate ivac-1b75 remains open.**


## 2026-10-15 — Imported drawings / layers presentation continuation

[Drawings and layers clarification](ux/DRAWINGS_LAYERS_CLARIFICATION.md) records
the pre-edit control/model audit, same-name union provenance, view-only visibility
versus authoritative CAM sources, read-only ACI swatches and actual before/after
checks. Imported geometry, selection/source bindings and persistence rules are
unchanged. Seven targeted regressions, 1270 frontend passes/1skip, check/lint/build
pass. Human visual gate ivac-ena7m remains open; no new major CAM milestone or
functional roadmap/previous qualification acceptance is inferred.
