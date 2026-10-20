# FabCAM — quickstart

A milling-oriented introduction to the current Atelier shell. Build the app using [BUILDING](BUILDING.md) or [Windows instructions](BUILDING_WINDOWS.md). Target platforms are Windows, macOS and Linux; the current fork's local desktop qualification is macOS. This checkpoint publishes no installer or release.

## 1. Create or open a project

Welcome offers New project, Open project, Import SVG/DXF, recent projects and an example. Save projects through the existing File/Save actions. Legacy `.ivac-project.json` / `.vc-project.json` naming and serialization stay supported; the FabCAM name does not migrate files or storage.

## 2. Import and select geometry

Import SVG or DXF, inspect units/scale and drawing bounds, and select the required geometry/layers. Existing import and editing behavior is unchanged. Do not assume imported artwork has machining-safe dimensions.

When a file does not say what one drawing unit measures (DXF without `$INSUNITS`,
SVG sized in `px` or without a size), or the importer skipped something, an
**Import summary** opens before anything is added: it shows the units found in
the file, the resulting drawing size in mm and grouped import warnings. Choose
the drawing units (1 unit = 1 mm by default, 96 or 72 dpi pixels, inch or a
custom mm-per-unit value), check the size, then import. The choice is saved with
the project and reused when the source is reloaded. Files with declared or
physical units (DXF `$INSUNITS`, SVG in mm/cm/in/pt) import at their true size
without a choice. A warning badge next to an imported file reopens its summary.
Tilted 3D DXF entities and MINSERT arrays are refused; flatten or explode them
in the CAD tool first.

## 3. Configure stock and work origin

Set stock dimensions and placement, work-origin reference and fixture obstacles. These declare job intent; they do not establish physical machine zero. Nonzero work-origin Z is currently rejected in the bounded qualified setup. Work-origin/stock/geometry dragging remains deferred; fixture interaction has its own supported controls.

## 4. Configure machine and tools

Use the active-machine area and tool library. Machine profiles declare capabilities and limits; SOFTWARE ONLY/unvalidated qualifiers are meaningful. Select the operation's actual tool and check its diameter/type and machine suitability. A profile is not proof of physical machine state.

## 5. Add operations and review cutting values

Select geometry and configure profile, pocket or drilling operations. Inspect depth, compensation, allowances and supported leads/tabs/entry. Additional inherited operation types require their own machine/post qualification.

Manual cutting values remain authoritative. Feeds & Speeds recommends from traceable data; Apply is explicit. Imported ReferenceData and derived matches are not physical lab evidence. Local preset capture starts as Draft unless actual trial evidence is supplied. FS4 physical validation remains deferred.

Build your own local presets or import a versioned JSON source using the
[custom cutting-data guide](feeds-speeds/CUSTOM_DATA_SOURCES.md), including synthetic
examples, evidence levels, backups and current UI limitations.

## 6. Generate and inspect

Generate the program, inspect 2D/3D preview and warnings, and verify work frame, stock, clearances and fixtures. Critical errors can block export; changes can make the preview stale. Bounded remaining-material analysis is a per-pocket/depth diagnostic, not an implemented cleanup operation or complete machine simulation.

## 7. Export a supported program

ShopBot uses the native CPS runtime; the current standalone browser WASM package cannot run CPS. Use the Tauri desktop app or a CPS-enabled native server for ShopBot review. Helix, unqualified two-sided CPS/translated registration and some RPM-transition combinations reject. Consult [STATUS](shopbot-modernization/STATUS.md) and the operation/post qualification records before relying on a combination.

## 8. Set up and validate on the machine

Review controller preview, tool, stock, work zero, safe travel, fixture clearance and spindle assumptions. Conduct appropriate supervised air-cut/material/measurement checks. FabCAM does not directly operate the machine. The historical M4 square evidence does not qualify every future job or operation.

## More information

[Current limitations](shopbot-modernization/STATUS.md), [roadmap](shopbot-modernization/ROADMAP.md), [CAM validation](shopbot-modernization/CAM_VALIDATION.md), [project origins and license](../README.md#project-origins). FabCAM is derived from ivaCAM; legal attribution and internal compatibility identifiers are retained.
