Open-source CAM software for digital fabrication.

<!--
  This file is the source for the About dialog's prose. It is read at
  build time by the `ivac-about-md` plugin in `frontend/vite.config.ts`,
  which substitutes the tokens below and exposes the result as the
  `virtual:about` module. Only a subset of Markdown is rendered (headings,
  bold, italic, inline code, links, bullet lists, paragraphs) — see
  `frontend/src/lib/components/markdown-lite.ts`. Tokens:
    %%VERSION%%      git describe --always --dirty (e.g. v0.1.0-3-gabc1234)
    %%PKG_VERSION%%  configured frontend version, or Tauri version for this unversioned frontend
    %%DATE%%         ISO-8601 UTC build timestamp
-->

### Project origins

FabCAM is derived from the open-source [ivaCAM project](https://github.com/aalarchiv/ivacam) by Soenke J. Peters. The fork develops workshop-oriented milling workflows, machine-aware preparation and transparent cutting recommendations. Original authorship and upstream contributions remain credited.

### Open source

FabCAM retains [GNU General Public License v3.0 or later](https://www.gnu.org/licenses/gpl-3.0.html). Source and development records are at [harry-finch/FabCAM](https://github.com/harry-finch/FabCAM). See the repository's [LICENSE](https://github.com/harry-finch/FabCAM/blob/main/LICENSE) and [technical status](https://github.com/harry-finch/FabCAM/blob/main/docs/shopbot-modernization/STATUS.md) for qualification limits.

Third-party dependencies and reference-data packs retain their own notices and licenses. Local/private manufacturer packs are not part of the public distribution. The original GPL text remains in the repository as `LICENSE`.

### Acknowledgements

- **[viaConstructor](https://github.com/multigcs/viaconstructor)** — a Python CAM tool of similar scope that inspired the idea and the name (besides being a funny acronym for "I vibe-coded a CAM"). ivaCAM ported some calculations, so it may or may not be some kind of derivative work.
- **Estlcam** — its feature catalogue inspired the CAM primitives ivaCAM implements. No Estlcam code is used; algorithms are implemented from public literature. Estlcam is not free, but it is great software at a reasonable price. Buy it!
- **The CNC and maker community** — bug reports, test geometries, and the machine quirks that turned synthetic test suites into real shop-floor coverage.
