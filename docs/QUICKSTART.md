# FabCAM — quickstart

Use [STATUS](shopbot-modernization/STATUS.md) to select the accepted source, then
[BUILDING](BUILDING.md) / [Windows setup](BUILDING_WINDOWS.md). Historical builds can
lack later features. Current software qualification does not establish installed-native
file workflow or physical machining acceptance. Frontend languages are EN/DE.

## 1. Create or open a project

Welcome offers New/Open/Import and an example. Save/open a **ProjectFile version 10**;
legacy `.ivac-project.json` / `.vc-project.json` suffixes do not make V9 content
compatible. Older/future versions refuse without rewriting originals. Keep backups;
there is no implicit machining-data migration.

## 2. Import and place geometry

Import SVG/DXF and review units, scale, dimensions, layers and warnings. Unitless DXF
or non-physical SVG prompts for the drawing unit; physical declarations retain their
true scale. Source unit choice is saved and reused on reload. Do not double-convert
physical units. Tilted DXF/MINSERT or inputs exceeding parser/expansion limits refuse.

Mouse drawing placement and proportional dimensions are available. Original source
coordinates, explicit transforms, manual stock and work origin are separate. Import
must not silently rebase manually configured stock/zero. Cleanup Analyze/Preview/Apply
has bounded continuity/topology checks; local repair acceptance does not certify CAM.
Crossings/touching/ambiguous topology may require CAD editing; no universal repair.

## 3. Configure stock and work origin

Set stock dimensions/placement and work origin; inspect their relation to the drawing
and fixtures. These are job coordinates, not physical machine zero. Nonzero work-origin
Z remains outside the bounded qualified setup. Establish physical work zero separately.

## 4. Configure machine and tools

Choose the actual machine/tool and check diameter, usable flute length, limits and
process suitability. SOFTWARE ONLY/unvalidated labels matter. Substrate/coating is
optional descriptive metadata, not an automatic cutting calculation or machine proof.

## 5. Author cutting values or review a preset

Profile/Pocket require Main Cutting; optional Finishing is OFF by default. Add explicit
finishing tool/rates/DOC and applicable reserves. Scoped Pocket offers Walls / Floor /
Walls and floor within supported geometry/entry/proof budgets. Other families retain
their actual roles. Enter RPM/feed/plunge/DOC; the app validates known limits and
reports missing/unknown information. It does not calculate rates from chipload.

[TESTED_PRESETS](TESTED_PRESETS.md) explains named collections, exact tested loading
and the distinct reviewed starting-value copy. The latter can copy selected numbers
across contexts without transferring testing or stepover. A new target attestation
requires a real target trial. FabSpeed is independent; optional exchange is planned.

## 6. Generate and inspect

Generate, then inspect full-program simulation, provenance, work frame, stock,
clearances and diagnostics. Changes stale the old output. Bounded remaining-material
diagnostics are not optimized rest machining or full machine simulation. Restricted
Helix, legacy Profile overcut and affected two-sided setups refuse. Never bypass a
refusal by relabelling a source or using an old fixture/installer.

## 7. Export a supported program

ShopBot OpenSBP requires a CPS-enabled native adapter; default standalone WASM has no
CPS. Posts and same-tool RPM transitions have explicit limitations. Native selection
grants file access dynamically; a recent/reloaded source can require selecting it
again. Export checks are separate from raw FS plugin authorization. Per-file
transactional output is not pair-atomic. See [cutting/export contract](CUTTING_CONTRACT.md).

## 8. Set up and validate on the machine

Separately review actual controller, tool/spindle, stock, holding, work zero, travel
and fixtures, then conduct authorized supervised air/material/measurement checks.
FabCAM does not operate the machine. Historical M4 evidence is not physical
qualification of the current software/job.

## More information

[STATUS](shopbot-modernization/STATUS.md), [technical roadmap](shopbot-modernization/ROADMAP.md),
[release gates](releases/RELEASE_ROADMAP.md), [architecture](ARCHITECTURE.md),
[origins/license](../README.md#project-origins). Preserve original attribution and evidence.
