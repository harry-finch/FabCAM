# FabCAM A1 — UX2.2 branding integration, 2026-10-14

Engineering complete locally: ivac-7cdj. Human review remains under UX2.2
ivac-ein1. User explicitly authorized integration of the finalized production
assets; prior asset gate ivac-8k3o closed for canonical assets/outlined wordmark.
The micro candidate has not been explicitly approved and is not integrated.
No new CAM milestone, commit, merge, push, release or binary publication.

## Approved assets located

Authoritative masters remain in `assets/branding/fabcam/source/`: exact original
A1 symbol/app tile/editable lockup, plus the accurately outlined Inter lockup.
Concept 01 — Offset Pocket / Variant A — Balanced Offset / A1 Optical Balance.
Palette, geometry, transforms, corners and margins are unchanged. Full source
and export evidence: [brand specification](../../../assets/branding/fabcam/README.md).

## Assets integrated and policy

| Context | Runtime asset | Authoritative source |
| --- | --- | --- |
| Header / About | `frontend/public/branding/fabcam-symbol.svg` | `source/fabcam-a1-symbol.svg` |
| Welcome | `frontend/public/branding/fabcam-lockup.svg` | `source/fabcam-a1-lockup-outlined.svg` |
| SVG favicon | `frontend/public/favicon.svg` | Canonical symbol |
| PNG favicons | `frontend/public/favicon-{16,32,64,128,256}.png` | Canonical symbol exports |
| Apple touch icon | `frontend/public/apple-touch-icon.png` | Canonical app tile, 180 px |
| Tauri desktop resources | `crates/ivac-tauri/icons/` | Canonical app tile and reviewed native exports |

The public/Tauri files are generated runtime copies, not a second independent
branding system. Canonical symbol for normal UI/SVG, app tile for desktop icons,
outlined horizontal lockup for Welcome; monochromes remain available but unused
because the current dark surfaces suit the canonical artwork. No micro assets,
font files or new runtime graphics dependencies are shipped.

## Application header

24 × 24 px canonical symbol beside the existing translated FabCAM name; decorative
image has empty alt and aria-hidden, so the name is announced once. Shared font,
spacing and surface styles retained. Browser measurement at 1280 × 800:
navigation height **30.2890625 px before and after**. The image loads at 24 × 24,
centered with the name and existing Project/Machine/Tools navigation. No header
height increase or full-width lockup. Existing narrow navigation is untouched.

## Welcome screen

Existing h1 now contains the production outlined lockup, accessible name FabCAM.
258 px nominal width and 64.09375 px measured height preserve the 322:80 aspect
ratio. Intro, New/Open/Import, recent projects, browser guidance, example/help
actions and their handlers are unchanged. No duplicate h1 name or decorative hero.

## Browser favicon

`frontend/index.html` references SVG plus PNG 16/32/64/128/256 fallbacks and the
existing Apple touch path. SVG appears after raster fallback declarations.
Nine branding/favicon URLs returned HTTP 200 with correct MIME types and bytes
matching `frontend/public` in development and production. Evidence:
`assets/branding/fabcam/previews/http-validation.json`. Initial HTML title now
matches the existing ivaCAM application identity rather than the Vite template
title; App.svelte's runtime/version title is unchanged.

## Tauri Windows icons

Existing `icons/icon.ico` now contains the reviewed seven-entry A1 tile ICO
(16/24/32/48/64/128/256). Tauri's existing executable/bundle/installer/window
resource pipeline uses the same configured icon paths. Existing Store/Square
PNG names and dimensions were regenerated directly from the approved tile.
Container decoding/source-pixel fidelity and configuration paths verified;
no Windows compilation, installer, taskbar or runtime trial was performed.

## Tauri macOS icons

`icons/icon.icns` contains the reviewed A1 tile. Local optimized Tauri build
completed with `--bundles app -- --locked`; artifact:
`target/release/bundle/macos/ivaCAM.app`, approximately 21 MiB. Bundle Info.plist
confirms name ivaCAM, identifier org.opcenter.ivacam, version 0.5.0 and
CFBundleIconFile icon.icns. Bundled ICNS bytes exactly match the input asset.

The generated bundle was actually launched. Welcome and About rendered the A1
artwork; existing recent-project/reopen data remained visible. A genuine Finder
icon-view screenshot shows the new app icon. Dock rendering, icon-cache behavior,
all small OS scaling modes, signed distribution and DMG installation are not
qualified by this check. The known iconutil 16/32 px edge-extraction differences
remain documented in the brand specification; canonical encoded PNGs are lossless.

## Tauri Linux icons

Existing 32/64/128/256/512 PNG icon resources now contain A1 tile rasters.
Configured bundle icon paths are unchanged and resolve. Master PNG exports and
Linux links remain available for desktop packaging. Dimensions/alpha validated;
no Linux build, desktop launcher, compositor/task-switcher or package trial.

## About and identity preservation

Only the old image reference and its EN/DE alt text changed. Image remains 80 px.
Existing About title, author, version/build/date, GPL license, acknowledgements,
library attributions and project links are preserved and visible in browser and
native screenshots. The OS application still identifies as **ivaCAM** by design:
productName, window title, application/bundle identifier, version, permissions,
security, updater and file associations were not changed. This is logo integration,
not an application rename or a legal attribution rewrite.

## Obsolete branding cleanup and reproducibility

Removed only the now-unreferenced `frontend/public/ivacam-logo.png` after a
repository reference search. Existing favicons and desktop resource names were
replaced in place. Historical artwork, documentation/fixtures and Android's
separate launcher-generation pipeline remain; Android branding is out of scope.
Active desktop/frontend image paths no longer reference the old logo.

Run from repository root:

```sh
python3 assets/branding/fabcam/tools/build_assets.py --integrate
```

Requires Pillow and Inkscape for build-time generation only; no font required.
This command copies approved SVG/ICO/ICNS exports, directly rasterizes missing
180 px/Store sizes, validates dimensions/alpha/config paths, and emits
`previews/integration-validation.json` plus `app-icon-size-review.png`.
Full master-export rebuilding remains a separate default command requiring
temporary genuine Inter; see the brand README. Do not regenerate using an
unreviewed concept or independently edit the runtime copies.

## Build and validation results

| Check | Result / limit |
| --- | --- |
| Frontend tests | 124 files; 1145 pass, 1 optional skip |
| Svelte/TypeScript check | 0 errors, 0 warnings |
| ESLint / Prettier | Pass after formatting the Welcome markup |
| Production frontend build | Pass through Tauri beforeBuildCommand |
| Development and production resources | 9 URLs each: HTTP 200, correct MIME, source bytes match |
| Source SVG / native format checks | Prior reviewed source validation retained; integrated copies match |
| Runtime raster sizes / alpha | 21 PNG resources checked; exact dimensions and transparent corners |
| Tauri paths / protected configuration | All icon paths resolve; tauri.conf.json unchanged |
| macOS compilation and packaging | Optimized build and .app bundle completed; icon bytes verified |
| macOS runtime / Finder | Actual Welcome/About and Finder icon view verified |
| Windows / Linux | Structural asset validation only; no native package/runtime claim |
| CAM/FS/post/schema/identity | No source/config changes |

No new Rust algorithm test run was necessary for this graphics-only integration.
The successful native build is compilation evidence, not CAM or machine qualification.
Native Save As/picker/reopen/German-layout checks in ivac-ein1 remain independent.

## Screenshots and review

All application screenshots are genuine JPEG captures, not simulated mockups.

- [Browser Welcome, 1280 × 800](A1-branding-welcome.jpg)
- [Browser project loaded, 1280 × 800](A1-branding-project.jpg)
- [Browser smaller window, 1024 × 640](A1-branding-small-window.jpg)
- [Browser About](A1-branding-about.jpg)
- [macOS native Welcome](A1-branding-native-welcome.jpg)
- [macOS native About](A1-branding-native-about.jpg)
- [Actual Finder icon view](A1-branding-finder.jpg)
- [App icon raster sheet, 16/24/32/64/128/256 px](../../../assets/branding/fabcam/previews/app-icon-size-review.png)

Human review of this UX2.2 continuation is next. Do not automatically publish,
release, rename the application, integrate the micro candidate or advance CAM work.
