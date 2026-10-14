# FabCAM — A1 Optical Balance

Finalization of **Concept 01 — Offset Pocket / Variant A — Balanced Offset /
A1 Optical Balance**, prepared on 2026-10-14 (ivac-79bq). The subsequent explicitly
authorized UX2.2 integration is implemented locally (ivac-7cdj), awaiting human
review under ivac-ein1. Canonical source geometry remains unchanged.

## Canonical assets

| Asset | File | Notes |
| --- | --- | --- |
| Symbol | `source/fabcam-a1-symbol.svg` | Transparent, no font dependency; 64 × 64 viewBox |
| App tile | `source/fabcam-a1-app-icon.svg` | Graphite tile with transparent outer corners; 128 × 128 viewBox |
| Editable lockup | `source/fabcam-a1-lockup.svg` | Original live text; 322 × 80 viewBox |
| Production lockup | `source/fabcam-a1-lockup-outlined.svg` | Genuine Inter glyph outlines; no font dependency |

The first three files are byte-for-byte copies of the supplied approved SVGs.
Names have been normalized; geometry, colors, viewBoxes, spacing, stroke widths,
rounded corners and transforms have not been optimized or reconstructed.
The outer stroke remains 4.6 units, the inner stroke 4.3 units, and the central
square 7.2 units. Keep these deliberate optical differences.

## Colors and backgrounds

| Role | Exact color |
| --- | --- |
| Symbol | `#58B5F5` |
| App tile | `#171E28` |
| Tile border | `#354250` |
| Wordmark Fab / light monochrome | `#EEF3F8` |
| Wordmark CAM | `#75C8FA` |
| Dark monochrome | `#171E28` |

Canonical blue and the two-color wordmark are intended for dark neutral
backgrounds. On white/light backgrounds use the dark monochrome lockup or symbol;
the canonical light wordmark is unsuitable there. Each monochrome SVG uses
one explicit foreground color, with no grayscale filter or opacity simulation.
The app tile supplies its own dark surface; its exterior corners are transparent.
No palette adjustment has been made.

## Clear space and icon safety

Keep at least one outer-stroke width (4.6 units on the 64-unit symbol, about 7.2%
of its nominal size) of clear space around the visible mark, outside the asset
box when possible. This is a usage recommendation, not a change to the approved
internal margins. Retain the lockup's existing symbol/text gap and letter spacing.

App symbol ink occupies x/y 19.29–108.71 within the 128-unit tile: equal 15.1%
margins on all four sides, centered at (64, 64). This gives the important symbol
substantial clearance from normal platform masks; the tile edge/border may be
masked by the OS. Do not add another shadow, gradient, or OS-specific frame.
Visual balance was reviewed on neutral surfaces, without an installed Dock/
Start-menu trial. Platform installation remains an integration check.

## Optical review

Assessment uses direct Inkscape rasterization at each target size, no resampling.
Inspect `previews/logo-review.png` at 100% image scale; Retina/CSS scaling can
otherwise disguise the actual pixel count. `symbol-size-review.png` includes all
eight requested sizes. `micro-comparison.png` puts both variants side by side,
with unscaled samples and separate nearest-neighbor 4× pixel inspection.

| Symbol size | Assessment |
| --- | --- |
| 16 px | Recognizable, but the central square is only 1.8 px and the inner opening 1.525 px; antialiasing softens both |
| 20 px | Better contour separation; the opening remains only 1.906 px |
| 24 px | Practical canonical minimum; opening 2.288 px, center 2.7 px |
| 32 px | Clear frame, opening and center; rounded corners remain distinct |
| 48 px | Stable contour spacing and stroke treatment |
| 64 px | Full intended construction is clearly legible |
| 128 px | Crisp geometry with normal rounded-edge antialiasing |
| 256 px | Crisp geometry; no clipping or unexpected contour artifacts |

The separation between outer and inner ink is 7.55 SVG units (1.888 px at 16 px).
Fractional stroke widths are intentionally preserved; no per-size snapping or
stroke rounding has been applied. Recommend **24 px minimum for the canonical
symbol and 32 px for the app tile** when the opening is important. Use a lockup
at least 40 px high to retain a 32 px symbol. The 16/20 px
app exports are technically valid, but optically less informative.

### Micro candidate policy

`variants/fabcam-symbol-micro.svg` is an optional, unapproved **16–20 px candidate**.
Only the inner path start changes from y=28.6 to 26.6 and the end from y=39 to 41.
The visible opening grows from 6.1 to 10.1 units (2.525 px at 16 px); the outer
frame, strokes, central square, palette and all other coordinates remain intact.
It improves the toolpath opening but cannot enlarge the tiny center without
another design change. A 24 px micro sample is provided only for comparison.
Never substitute it silently for the canonical symbol or app tile. The app icon,
ICO and ICNS exports all use the approved canonical construction.

## Wordmark and licensing

The editable original keeps `Inter, IBM Plex Sans, sans-serif`, size 39,
letter-spacing −1.35, Fab weight 500 and CAM weight 700 at x=86 / baseline y=52.
The draft does not identify the font actually used by its author. Neither Inter
nor IBM Plex Sans was installed here; fontconfig initially fell back to Verdana.

The outlined version deliberately resolves the **first declared family, Inter**,
using official Inter 4.1 Medium and Bold OTF files and Inkscape text-to-path.
No manual glyph tracing, substitution, spacing adjustment, or font files are
included. Confirm the Inter rendering against your approved wordmark before
integration; an original screenshot alone would not identify its actual font.

Source: [Inter official repository](https://github.com/rsms/inter),
[release 4.1](https://github.com/rsms/inter/releases/tag/v4.1),
[SIL Open Font License](https://github.com/rsms/inter/blob/v4.1/LICENSE.txt).
Fonts were used temporarily for conversion and not installed globally.
Medium SHA-256: `6c222ed2c3660524dd0c14fb745e00fee65c5361ab28f60fba6730bf4e648763`.
Bold SHA-256: `9fc6261e817d0b1b2bf1953d8799bb1c837d11aeebbf83ae352e7b4d6de6425d`.

At 4× rendering, text and outlines have identical ink bounds. Only 300 of
412,160 raster pixels differ (0.073%), in glyph-edge rasterization; mean absolute
channel difference is 0.000728 on a 0–255 scale. This is not a byte-identical
render, and the report records the difference rather than hiding it.

## Exports and structure

```text
source/         3 exact originals + outlined production lockup
variants/       4 monochromes + separate micro candidate
exports/png/    10 app PNGs, 8 symbol PNGs, 3 micro PNGs, outlined lockup PNG
exports/macos/  fabcam.icns
exports/windows/fabcam.ico
exports/linux/  relative links to the app PNGs, avoiding duplicate pixel data
previews/       review sheets + machine-readable validation.json
tools/          reproducible generation/validation script
```

App PNGs: 16, 20, 24, 32, 48, 64, 128, 256, 512, 1024 px.
Symbol PNGs: 16, 20, 24, 32, 48, 64, 128, 256 px.
Windows ICO contains seven decoded entries: 16, 24, 32, 48, 64, 128, 256 px.
macOS ICNS starts from native `iconutil` output, with the legacy 16/32 px slots
replaced by supported lossless PNG `icp4`/`icp5` entries. All encoded sizes decode
pixel-identically to their source PNGs in Pillow. Native `iconutil` accepts the
container and extracts ten 1×/2× PNG slots for 16, 32, 128, 256 and 512 logical pixels.
Its 16/32 px extraction changes antialiased RGB edges (maximum visible channel
differences 34/50 on white) while preserving alpha; the other eight slots remain
pixel-identical. These native extraction differences are recorded in the validation
report. Canonical source colors are unchanged; check small macOS display behavior
during integration before treating the ICNS as visually qualified on macOS.
Linux links resolve within this directory; copy their target PNG bytes when
installing into a desktop theme/package. No Linux desktop entry was installed.
These are validated file formats; runtime OS icon appearance is not yet qualified.

## Rebuild and checks

Requires Python with Pillow 12.3, Inkscape, fontconfig, and temporary genuine Inter
Medium/Bold available to fontconfig. On macOS, `iconutil` builds the ICNS; when
unavailable, retain the source PNG sizes for later native conversion. Do not bundle
the font files in this asset directory. Review-sheet labels use macOS Arial.

Run `python3 tools/build_assets.py` from this directory with the desired
`FONTCONFIG_FILE` set; optional `INKSCAPE` selects the renderer executable.
Use a temporary fontconfig file which includes the normal system configuration,
adds a temporary Inter font directory, and places its cache in a temporary folder.
The generator refuses to outline if fontconfig resolves Inter to another family.

`previews/validation.json` records SVG parsing/rendering, palette checks,
consistent symbol geometry, transparent corners, unclipped ink bounds, PNG sizes,
native format decoding and text/outline comparison. All production SVGs have
no raster images, scripts, external URLs or font dependencies. Only the explicitly
editable original lockup requires a locally available declared font.

## Application integration checkpoint

User approved production-asset integration on 2026-10-14; ivac-8k3o is closed for
the canonical assets and outlined wordmark. Micro remains an unapproved candidate.
Header and About use the canonical symbol, Welcome the outlined lockup, browser
favicons the canonical symbol and desktop resources the app tile. Generated
copies live at existing frontend public/Tauri icon paths; the masters here remain
the single authoritative asset system.

Refresh runtime copies from the repository root, with Pillow/Inkscape available:

```sh
python3 assets/branding/fabcam/tools/build_assets.py --integrate
```

This command needs no font and writes `previews/integration-validation.json` and
the 16/24/32/64/128/256 px app-tile sheet. HTTP resource evidence is recorded in
`previews/http-validation.json`. Local macOS .app compilation, runtime Welcome/
About and Finder icon view were verified; Windows/Linux packaging/runtime and
Dock/small-size OS scaling remain untested. Tauri name/identifier/security and
application behavior are unchanged in that branding checkpoint. Subsequent
[product identity work](../../../docs/shopbot-modernization/ux/PRODUCT_IDENTITY.md)
updates UI/window/macOS display names to FabCAM while retaining installation,
executable, identifier and storage compatibility. A1 artwork is unchanged.
No release, commit or push.

See [the integration handoff](../../../docs/shopbot-modernization/ux/A1_BRANDING_INTEGRATION.md)
for screenshots, exact runtime paths and platform limits. Await human UX2.2 review
under ivac-ein1 before publishing or proceeding further.
