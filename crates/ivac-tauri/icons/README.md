# Tauri bundle icons — FabCAM A1

`tauri.conf.json` references `32x32.png`, `128x128.png`, `128x128@2x.png`,
`icon.icns`, and `icon.ico`. These existing resource paths now contain the
approved A1 Optical Balance app tile. `icon.svg` is a generated exact copy of
the authoritative master `assets/branding/fabcam/source/fabcam-a1-app-icon.svg`.
Do not edit it independently. To refresh browser and desktop resources, run
from the repository root with Python/Pillow and Inkscape available:

```sh
python3 assets/branding/fabcam/tools/build_assets.py --integrate
```

This reuses the reviewed ICO/ICNS containers and rasterizes the existing
Windows Store sizes directly from the canonical SVG. No font is required.
The full asset regeneration process and ICNS native extraction limitation are
documented in [the brand specification](../../../assets/branding/fabcam/README.md).

The public productName and display identity are FabCAM. Compatibility identifier
`org.opcenter.ivacam`, internal executable and data paths remain unchanged.
See the REL1-A transition record for prior test installers. Native executable/bundle/package
icons remain driven by the established `bundle.icon` resources. Android's
separate launcher-generation script and historical artwork remain untouched;
this checkpoint targets Windows, macOS and Linux desktop only.
