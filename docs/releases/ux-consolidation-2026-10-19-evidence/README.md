# Preuves fraîches — consolidation UX A–D, 2026-10-19

[Bilan, réserves et portée](../UX_V1_CONSOLIDATION_2026_10_19.md).
Le dossier contient les fichiers d'entrée/sortie et les journaux complets ; TEST1
conserve volontairement ses requêtes/réponses, seeds et sommes de contrôle.
Pas de fichier destiné à une machine ni de paquet de distribution.

## Parcours visuels

| Fichier | Ce qu'il démontre |
|---|---|
| 01-3d-stock.png | Stock présent après masquage des parcours, 3D stable |
| 02-full-playback.png | 431/431, critique rapide dans stock, Download désactivé |
| 03-home-export-enabled.png + DOM | 1/431, critique absente, Download actif : **défaut soumis ivac-3xqtb** |
| 04-report-1280.png + DOM | Rapport courant et footer accessibles ; résultat complet repris à End |
| current-report.md | Véritable téléchargement UI ; minZ−1, origine60/60/0, MDF, rates et 1 critique |
| 05-stale-tool-review.png | Comparaison Ø3→4,01, opération impactée, Apply désactivé après modification |
| 06-tool-applied-dom.txt | Application de la copie projet et état de génération périmé |
| 07-origin-stale-dom.txt | Origine120/0 et génération périmée, avant Undo |
| roundtrip.ivac-project.json | Vraie sauvegarde après outil Undo et Pocket→Frame→Pocket ; identique au fichier C parsé |
| 08-cps-refusal.png | Homing CPS refusé sans supprimer l'opération |
| 09-cps-generated.png + DOM | Après désactivation Homing :137 mouvements, .sbp, panneau OpenSBP168 lignes |
| 10-preview-settings-dom.txt / 11-performance-dom.txt | Réglages lus, Exact rewind activé ; aucune préférence modifiée |
| 12-material-draft.png | Brouillon matériau non appliqué avant Cancel |
| after-material-cancel.ivac-project.json | MDF/rates conservés ; ajout de clé machine CPS après passage entre projets, **ivac-52tpd** |
| 13-stale-report-dom.txt | Rapport OUTDATED, stats/temps/minZ précédents omis |
| 14-modal-resize.json / 14-stale-report-1280.png | Bornes à1920 puis1280 et footer accessible ; fenêtre déjà mémorisée petite, aucune extrapolation à un OS natif |
| 15-keyboard-fixed.png + DOM | Après correctif/rebuild : ArrowDown−1→−1,1 et Tab, inspecteur toujours ouvert |
| browser-errors.json | Deux notifications ResizeObserver consignées ; pas de blocage observé |

Captures sombres en navigateur intégré. Les lots possèdent leurs autres captures
historiques, explicitement distinctes de cette campagne. Aucun essai d'impression,
de machine réelle, ni qualification système de zoom.

## Commandes et journaux

Racine `ivacam`, PATH Node24/Cargo/Homebrew, `RUSTC_WRAPPER=''`, Cargo offline.

```sh
npm --prefix frontend test
npm --prefix frontend run check
npm --prefix frontend run lint
npm --prefix frontend run build
cargo test --offline --locked -p ivac-core --features cps,profiling --lib -- --test-threads=1
cargo test --offline --locked -p ivac-core --features cps,profiling --test ux_inspector_contracts --test safe_z_extended --test shopbot_m2_post --test cps_posts --test vcarve_qualification
cargo build --offline --locked -p ivac-server
cargo fmt --all -- --check
cargo clippy --offline --locked -p ivac-core --features cps,profiling --lib -- -D warnings
node --test scripts/collect-desktop-candidate.test.mjs
cargo test --offline --locked -p ivac-core --features cps,profiling --test test1_properties
python3 scripts/test1/build.py
python3 scripts/test1/run.py --binary target/release/deps/test1_properties-a28bea547eac3aa1 --output docs/releases/ux-consolidation-2026-10-19-evidence/test1-release --cases 32 --budget 120 --frontend
git diff --check
```

Les journaux `*-verified.log` sont ceux du correctif final ; `frontend-final.log`
conserve l'échec intermédiaire du budget de taille1003>1000. Aucun seuil relevé.
`test1/` conserve l'essai debug incomplet ; seul `test1-release/` est256/256 PASS.
Le worker direct a4 PASS/1 IGNORE attendu ; le worker ignoré est exécuté par le
harness avec son budget, pas compté par le simple `cargo test`.
FANUC externe absent : le test conditionnel retourne, sans oracle réel.

Serveur18804 compilé sur cet arbre, capabilities/version lus via HTTP ; frontend
production preview18803 avec query api locale. Les processus/onglet de revue ont
été fermés. `incoming-files.json` représente les fichiers **avant** nos changements.
`preservation.json` compare leurs empreintes après documentation/correctif et conserve
les préfixes/suffixes historiques des documents. `SHA256SUMS.txt` porte les preuves
finales, sans s'inclure lui-même.
