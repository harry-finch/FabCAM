# FabCAM

[English](README.md) | **Deutsch**

**Quelloffene CAM-Software für praktische digitale Fertigung.**

![FabCAM A1 Optical Balance](assets/branding/fabcam/source/fabcam-a1-app-icon.svg)

## Überblick und Funktionen

FabCAM bereitet 2D-Zeichnungen für CNC-Bearbeitung vor: Geometrie, Rohmaterial, Werkzeuge, Operationen, Schnittwerte, Vorschau und Maschinenausgabe. Der aktuelle Schwerpunkt liegt auf Fräsabläufen in Werkstätten und Fablabs einschließlich eines begrenzten ShopBot-OpenSBP-Umfangs. FabCAM ist eine Weiterentwicklung von [ivaCAM](https://github.com/aalarchiv/ivacam), keine pauschale Produktionsfreigabe.

- SVG/DXF, Ebenen, Transformationen und Projektdateien.
- Profil, Tasche und Bohren; Anbindungen und Ein-/Ausfahrten mit maschinenabhängigen Prüfungen. Weitere geerbte Operationen sind vorhanden, aber nicht für jede Maschine/Kombination qualifiziert.
- Rohmaterial, Arbeitsnullpunkt, Maschinenprofile, Werkzeugbibliotheken, Spannmittel und Kollisionswarnungen.
- Frässpezifische Schnittwertempfehlungen mit Quellenangaben, expliziter Übernahme, manuellen Werten und privaten lokalen Voreinstellungen.
- 2D/3D-Vorschau und Materialabtrag-Simulation; begrenzte Restmaterialanalyse je Tasche/Tiefe, keine implementierte Restbearbeitung.
- Native CPS-Posts, begrenzte ShopBot-Ausgabe und kompakte Atelier-Oberfläche mit Startbildschirm, Dateien, letzten Projekten und Maschinen-/Werkzeugkontext.

![FabCAM-Arbeitsbereich](docs/shopbot-modernization/ux/A1-branding-project.jpg)

## Plattformen und Einstieg

Ein lokaler macOS-Tauri-Build wurde gestartet und visuell geprüft. Windows/Linux sind Build-Ziele mit geprüften Icon-Ressourcen; native Pakete/Laufzeit wurden in diesem Checkpoint nicht getestet. Browser-Entwicklung und Produktionsbuild sind geprüft. Das aktuelle WASM-Paket unterstützt keine CPS-Posts; ShopBot benötigt Desktop oder einen nativen Server. Geerbte Android-Werkzeuge liegen außerhalb der aktuellen Desktop-Qualifikation.

Voraussetzungen: [gepinntes Rust](rust-toolchain.toml), Node 20.19+ oder 22.12+, pnpm 10+, wasm-pack und für Desktop Tauri-CLI 2 samt Systembibliotheken. [Build-Anleitung](docs/BUILDING.md), [Windows](docs/BUILDING_WINDOWS.md).

```sh
git clone https://github.com/harry-finch/FabCAM.git
cd FabCAM
cargo build --locked
wasm-pack build crates/ivac-wasm --target web --release
pnpm --dir frontend install --frozen-lockfile
pnpm --dir frontend dev
```

Für Desktop anschließend `cd crates/ivac-tauri` und `cargo tauri build`. Technische Binär-/Installernamen können aus Kompatibilitätsgründen weiterhin `ivac`/`ivaCAM` enthalten. Es wird kein Release veröffentlicht.

## Arbeitsablauf

Projekt erstellen/öffnen → Geometrie importieren → Rohmaterial, Nullpunkt, Maschine und Spannmittel einrichten → Werkzeuge/Operationen wählen → Schnittwerte prüfen → Werkzeugwege erzeugen → Vorschau/Warnungen prüfen → unterstützten Code exportieren. Maschinenbezogene Einrichtung und physische Prüfungen erfolgen anschließend durch den Bediener. [Schnellstart](docs/QUICKSTART.md).

## Status, Grenzen und Planung

Softwareprüfung ersetzt keine physische Schnittprüfung. Die dokumentierte M4-ShopBot-Quadratprüfung qualifiziert nicht alle Fräsoperationen. Helix-Einstieg, bestimmte zweiseitige CPS-/Registrierungskombinationen und ein Arbeitsnullpunkt-Z ungleich null bleiben eingeschränkt/abgelehnt. Der Wechsel der ShopBot-Drehzahl zwischen Schruppen/Schlichten ist eine bekannte Exportgrenze. Referenzdaten sind keine physisch validierten Laborwerte. **FS4 bleibt zurückgestellt.** Menschliche CAM/FS/UX-Freigaben sind weiterhin offen.

Eckfreistellungen (Dogbone/T-Bone), Planfräsen, Vorlagen und fortgeschrittene Strategien sind spätere Arbeit; FabSBP ist Planung, Wasserstrahl/Vinyl sind zurückgestellt. Maßgeblich sind [STATUS](docs/shopbot-modernization/STATUS.md), [ROADMAP](docs/shopbot-modernization/ROADMAP.md) und Qualifikationsnachweise.

## Herkunft und Lizenz

FabCAM basiert auf **ivaCAM von Soenke J. Peters**: [aalarchiv/ivacam](https://github.com/aalarchiv/ivacam). Ursprüngliche Autorenschaft und Beiträge bleiben gewürdigt. ivaCAMs Kern enthält teilweise Rust-Portierungen von [viaConstructor](https://github.com/multigcs/viaconstructor); [Estlcam](https://www.estlcam.de/) inspirierte Terminologie, steuert aber keinen Code bei. Danksagungen und Copyright-Hinweise bleiben, einschließlich dxf-rs, cavalier_contours, clipper2-rust, usvg, Svelte, Three.js und Tauri.

**GPL-3.0-or-later**, siehe unveränderte [LICENSE](LICENSE). Abhängigkeiten und Referenzdaten behalten ihre Lizenzbedingungen. Private Herstellerpakete werden nicht automatisch weiterverteilbar oder Teil der öffentlichen Distribution. [Datenquellen/Lizenzen](docs/shopbot-modernization/feeds-speeds/DATA_SOURCES.md).

## Mitwirken

[Beitragsrichtlinien](docs/CONTRIBUTING.md), [Architektur](docs/ARCHITECTURE.md) und aktuellen Status lesen. Projekt: [harry-finch/FabCAM](https://github.com/harry-finch/FabCAM). [Identitätsentscheidung](docs/shopbot-modernization/adr/ADR-0012-fabcam-product-identity.md).
