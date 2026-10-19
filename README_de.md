# FabCAM

Aktuelle Entwicklungsversion: siehe [Versionspolitik und Release-Status](docs/releases/VERSIONING.md) (Beta zur V1-Stabilisierung); Desktop-Installation und Werkstattvalidierung stehen noch aus.

[English](./README.md) | **Deutsch**

**Quelloffene CAM-Software für praktische digitale Fertigung.**

![FabCAM A1 Optical Balance](assets/branding/fabcam/source/fabcam-a1-app-icon.svg)

## Überblick

FabCAM bereitet 2D-Zeichnungen für CNC-Bearbeitung vor: Geometrie, Rohmaterial, Werkzeuge, Operationen, Schnittwerte, Vorschau und Maschinenausgabe. Der aktuelle Schwerpunkt liegt auf Fräsabläufen in Werkstätten und Fablabs einschließlich eines begrenzten ShopBot-OpenSBP-Umfangs. FabCAM ist eine aktiv entwickelte Weiterentwicklung von [ivaCAM](https://github.com/aalarchiv/ivacam), keine pauschale Produktionsfreigabe.

## Hauptfunktionen

- SVG/DXF-Import, Ebenen, Zeichnungstransformationen sowie Speichern/Wiederöffnen von Projekten.
- Profil, Tasche und Bohren; Haltestege, Ein-/Ausfahrten und Eintauchsteuerung mit maschinen-/postspezifischen Prüfungen. Geerbte Operationen umfassen außerdem Gravur/Text, V-Carve, Fase, Gewinde, Schwalbenschwanz und T-Nut; ihr Vorhandensein qualifiziert nicht jede Maschine oder Kombination.
- Rohmaterial- und Arbeitsnullpunkt-Konfiguration, Maschinenprofile, Werkzeugbibliotheken, Spannmittel-Erstellung und Kollisionswarnungen für erzeugte Werkzeugwege.
- Frässpezifische Schnittwertempfehlungen (Feeds & Speeds) mit Quellenangaben, begrenztem Abgleich mit Referenzdaten, expliziter Übernahme, maßgeblichen manuellen Werten und privaten lokalen Voreinstellungen.
- 2D/3D-Vorschau und Materialabtrag-Simulation. Eine begrenzte Restmaterialanalyse je Tasche/Tiefe ist implementiert; Restbearbeitung (Rest Machining) nicht.
- Eingebaute G-Code-/HPGL-Dialekte und native CPS-Posts, einschließlich eines eingeschränkten ShopBot-OpenSBP-Teilumfangs. Nicht unterstützte Ausgabekombinationen werden abgelehnt.
- Kompakte Atelier-Oberfläche mit Startbildschirm, Neu/Öffnen/Import/Speichern/Schließen, letzten Projekten und Maschinen-/Werkzeugkontext.

Der Rust-CAM-Kern, die unabhängige Schnittwert-Engine und die Svelte-Oberfläche bleiben getrennt. Siehe [Architektur](docs/ARCHITECTURE.md) und [Fork-Architektur](docs/shopbot-modernization/ARCHITECTURE.md).

## Screenshots

Tatsächliche UX2.2-Oberfläche mit dem freigegebenen A1-Branding:

![FabCAM-Arbeitsbereich](docs/shopbot-modernization/ux/A1-branding-project.jpg)

[Prüfung von Startbildschirm/Icon](docs/shopbot-modernization/ux/A1_BRANDING_INTEGRATION.md) und [Prüfung von Produktidentität/Info-Dialog](docs/shopbot-modernization/ux/PRODUCT_IDENTITY.md).

## Unterstützte Plattformen

| Ziel | Aktuelle Verifikation |
| --- | --- |
| macOS | Lokaler optimierter Tauri-Build gebaut und gestartet; Branding/Anzeigename geprüft. Signierung und Installer-Upgrades erfordern eine separate Qualifikation. |
| Windows | Build-Ziel und Icon-Ressourcen vorhanden; dieser Checkpoint enthält keinen Windows-Paket-/Laufzeittest. |
| Linux | Build-Ziel und Icon-Ressourcen vorhanden; dieser Checkpoint enthält keinen Linux-Paket-/Laufzeittest. |
| Browser | Entwicklungs- und Produktionsoberfläche geprüft. Das aktuelle eigenständige WASM-Paket unterstützt keine CPS-Posts; ShopBot benötigt einen nativen Server oder die Desktop-App. |

Geerbte Android-Werkzeuge liegen außerhalb der aktuellen Desktop-Qualifikation. Diese Identitätsaktualisierung erzeugt keine Veröffentlichung und keine Binärdateien.

## Einstieg

Voraussetzungen: [gepinntes Rust](rust-toolchain.toml), Node 20.19+ oder 22.12+, pnpm 10+, wasm-pack und für das Desktop-Paket Tauri-CLI 2. Systembibliotheken, sccache und passende WASM-Werkzeuge stehen in [BUILDING.md](docs/BUILDING.md); [Windows-Anleitung](docs/BUILDING_WINDOWS.md).

```sh
git clone https://github.com/harry-finch/FabCAM.git
cd FabCAM
cargo build --locked
wasm-pack build crates/ivac-wasm --target web --release
pnpm --dir frontend install --frozen-lockfile
pnpm --dir frontend dev
```

Für ein Desktop-Bundle, nachdem die Plattformvoraussetzungen erfüllt sind:

```sh
cd crates/ivac-tauri
cargo tauri build
```

Für CPS-gestützte Browser-Entwicklung `cargo run --locked -p ivac-server` im Wurzelverzeichnis parallel zu Vite starten; die Auswahl des Transports steht in der Build-Anleitung. Namen von Programm-/Installerdateien können aus Kompatibilitätsgründen weiterhin `ivac`/`ivaCAM` enthalten; die [Identitätsentscheidung](docs/shopbot-modernization/adr/ADR-0012-fabcam-product-identity.md) erklärt den Grund.

## Arbeitsablauf

1. Projekt erstellen/öffnen und SVG/DXF-Geometrie importieren.
2. Rohmaterial, Arbeitsnullpunkt, Maschine und Spannmittel konfigurieren.
3. Werkzeuge wählen und Operationen konfigurieren.
4. Schnittwerte prüfen; eine Empfehlung bewusst übernehmen oder manuelle Werte verwenden.
5. Werkzeugwege erzeugen und Vorschau sowie Warnungen prüfen.
6. Ein unterstütztes Programm exportieren, anschließend maschinenspezifische Einrichtung sowie Steuerungs-/physische Prüfungen durchführen.

[Schnellstart](docs/QUICKSTART.md). FabCAM legt den physischen Maschinennullpunkt nicht fest und steuert die Maschine nicht direkt.

## Status und Einschränkungen

- Softwarequalifikation ist von physischen Nachweisen getrennt. M4 dokumentiert eine begrenzte, vom Bediener berichtete ShopBot-Quadratprüfung samt Vergleich, keine pauschale Fräsvalidierung.
- ShopBot-CPS ist eingeschränkt: Helix-Einstieg, nicht qualifizierte zweiseitige/verschobene Registrierung und ein Arbeitsnullpunkt-Z ungleich null bleiben begrenzt bzw. werden abgelehnt. Der Wechsel der Spindeldrehzahl zwischen Schruppen/Schlichten ist eine bekannte Exportgrenze.
- Spannmittel-Diagnosen und Restmaterial-Überlagerungen haben begrenzte geometrische Geltungsbereiche; sie belegen nicht alle physischen Freiräume.
- Herstellerreferenzdaten und daraus abgeleitete Empfehlungen sind keine physisch validierten Laborwerte. **Die physische FS4-Validierung der Schnittwerte bleibt zurückgestellt.**
- Menschliche CAM/FS/UX-Übergabeprüfungen sind weiterhin offen. Ein älterer CPS-Test, der bei zweiseitiger Bearbeitung Erfolg erwartet, muss an die explizite Ablehnung angepasst werden.

[STATUS](docs/shopbot-modernization/STATUS.md) und [CAM-Qualifikation](docs/shopbot-modernization/CAM_VALIDATION.md) sind maßgeblich für unterstützte Kombinationen und Nachweisgrenzen.

## Planung

Vorrangige Verbesserungen von Fräsablauf und -abdeckung erfolgen weiterhin innerhalb menschlicher Freigabeschritte. Eckfreistellungen (Dogbone/T-Bone) und Planfräsen haben begrenzte lokale Software-Übergaben. Wiederverwendbare Vorlagen für Profil/Tasche/Planfläche haben eine akzeptierte, begrenzte Software-Übergabe ([UX4.1-Qualifikation](docs/shopbot-modernization/UX4_1_QUALIFICATION.md)); fortgeschrittene Strategien bleiben spätere Arbeit. FabSBP ist reine Planung; ProtoMAX/Wasserstrahl und Vinyl sind zurückgestellt. Siehe die maßgebliche [ROADMAP](docs/shopbot-modernization/ROADMAP.md); geplante Arbeit ist keine implementierte Funktionalität.

## Herkunft

FabCAM basiert auf **ivaCAM von Soenke J. Peters**: [aalarchiv/ivacam](https://github.com/aalarchiv/ivacam). Der Fork entwickelt ShopBot-Vorbereitung, Schnittwertempfehlungen, Rohmaterial-/Nullpunkt-/Spannmittel-Abläufe und eine eigene Werkstatt-Oberfläche. Ursprüngliche Autorenschaft und Beiträge bleiben gewürdigt; FabCAM ist ein eigenständiges abgeleitetes Projekt.

Der CAM-Kern von ivaCAM ist teilweise eine Rust-Portierung der Geometrie-/Werkzeugweg-Routinen von [viaConstructor](https://github.com/multigcs/viaconstructor) (`calc.py`, `machine_cmd.py`, `setupdefaults.py`). [Estlcam](https://www.estlcam.de/) inspirierte die Funktionsterminologie, steuert aber keinen Code bei. Bestehende Quell-Copyright-Hinweise und Danksagungen an Abhängigkeiten bleiben erhalten, darunter dxf-rs, cavalier_contours, clipper2-rust, usvg, Svelte, Three.js und Tauri.

## Lizenz

**GPL-3.0-or-later**, siehe unveränderte [LICENSE](LICENSE). Abhängigkeiten, importierte Referenzdaten und separat lizenzierte Ressourcen behalten ihre eigenen Hinweise. [Lizenzangaben der Datenquellen](docs/shopbot-modernization/feeds-speeds/DATA_SOURCES.md). Lokale/private Herstellerpakete sind nicht automatisch weiterverteilbar und werden durch diese Aktualisierung nicht Teil der öffentlichen Distribution.

## Mitwirken

[Beitragsrichtlinien](docs/CONTRIBUTING.md), Architektur sowie aktuellen STATUS/ROADMAP lesen, bevor Änderungen vorgeschlagen werden. Das Repository nutzt Beads zur lokalen Nachverfolgung. Kompatibilität und Urheberangaben bewahren und für CAM-Änderungen angemessene Nachweise beifügen. Quelle: [harry-finch/FabCAM](https://github.com/harry-finch/FabCAM).
