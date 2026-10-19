# UX A–D — consolidation sur l'arbre courant

> 2026-10-19 — [Follow-up implementation](UX_SAFETY_LOAD_RESULTS_2026_10_19.md) corrects ivac-3xqtb / ivac-52tpd after explicit ADR-0024 approval. Fresh tests/browser evidence and compatibility limits are recorded there; original findings below remain historical. Human review ivac-3i00w is separate from consolidation ivac-amibd and UX D ivac-he4lt. No new candidates or publication.

> Follow-up — 2026-10-19: user approved bounded P0/P1 investigation/correction with an architecture gate. [Stage A](UX_SAFETY_LOAD_STAGE_A_2026_10_19.md) reproduces both defects and identifies an actual-export bypass in addition to the UI defect. ADR-0024 proposes independent background verification; implementation awaits that execution-boundary decision. Original evidence below remains historical; neither defect is resolved.

19 octobre 2026 · **Contrôles terminés, préparation des candidats BLOQUÉE.**
Travail `ivac-v5doi`, revue humaine **ivac-amibd OPEN**. Revue D `ivac-he4lt`
conservée séparément ; A/B/C déjà acceptés. **HARD STOP pour validation.**

## Périmètre et préservation

HEAD `66f229ea21cccf73a76fd45d0e9f9fe3721a41a1`, arbre local modifié,
FabCAM **0.9.0-beta.3 / pipeline62**, sans changement de version.
Les lots [A](UX_V1_LOT_A_2026_10_19.md), [B](UX_V1_LOT_B_2026_10_19.md),
[C](UX_V1_LOT_C_2026_10_19.md), [D](UX_V1_LOT_D_2026_10_19.md), les travaux MCP
et les autres changements préexistants ont été conservés. Le manifeste d'entrée
empreinte les 2 530 fichiers présents avant cette consolidation ; le contrôle final
identifie précisément les ajouts et modifications propres à ce travail.

Un seul correctif de production : une garde dans `OperationsList.svelte`, limitant
la navigation clavier de la liste à la ligne d'opération focalisée. Les flèches dans
Final Z remontaient au gestionnaire de liste, qui resélectionnait la même opération
et refermait son inspecteur. Après correction, ArrowDown modifie −1 en −1,1 mm et
Tab laisse le champ et l'inspecteur présents. Aucun paramètre CAM, algorithme,
garde de sécurité, schéma ou format de programme modifié. Ticket `ivac-uedp3`.

Aucun commit, push, tag, workflow distant, paquet de distribution ou publication.
Les builds frontend/backend servent seulement à cette vérification locale.

## Défauts soumis, sans correction

### Bloquant sécurité — ivac-3xqtb

Fixture `ux-c-2026-10-19-evidence/saved-preparation.ivac-project.json`, stock haut Z=2,
origine XY=(60,60), outil T2. Elle est volontairement diagnostique, pas prête à usiner.
Le réglage existant **Exact 3D rewind on backstep est activé** ; il n'a pas été modifié.

1. Ouvrir et générer : 431 mouvements, estimation 13 min 9 s.
2. Attendre la simulation à 431/431 : rapide dans la matière au segment0,
   2 alertes dont1 critique ; Download .ngc désactivé.
3. Home sur le curseur de lecture : 1/431, seule l'alerte pipeline hors enveloppe
   subsiste ; le bouton Download .ngc redevient actif.
4. End : l'alerte critique et le blocage reviennent. Le phénomène a été rejoué.
   Le rapport ouvert à Home omet également la critique ; à End, son Markdown
   contient bien « 1 critical · 1 non-critical ».

Chaîne établie par lecture du code : le rewind de `sim/driver.ts` réinitialise les
diagnostics au préfixe lu ; `Scene3D.svelte` les transmet à `project.setSimDiagnostics` ;
`GenerateBar.svelte` et `ReportDialog.svelte` consomment ce même état courant.
Le service d'export vérifie fraîcheur/collision pipeline mais ne remplace pas cette
vérification complète des alertes de simulation. **Aucun programme téléchargé pour
ce test ; aucun crash machine ni comportement matériel inféré.** L'origine historique
du défaut n'est pas attribuée au seul lot D.

Décision demandée : autoriser un chantier sécurité distinct définissant une preuve
de vérification complète indépendante du curseur, son invalidation et sa consommation
par toutes les entrées d'export et par le rapport. Ajouter alors les régressions
génération/rewind/retour en fin/régénération et refus de publication. Pas de contournement
par désactivation d'une protection ni modification automatique de politique.

### Réserve chargement/format — ivac-52tpd

La sauvegarde initiale après outil Undo et Pocket→Frame→Pocket est sémantiquement
identique au fichier C. Après passage par la fixture CPS puis réouverture du même
projet LinuxCNC, une sauvegarde ajoute `machine.cpsPost` (ShopBot bundled), absent
du fichier d'entrée. Les autres données, notamment matériau et opérations, sont
identiques. Les deux fichiers réellement téléchargés sont conservés.

`project-file-ops.ts:249` fusionne la machine précédente avec les champs du fichier,
ce qui explique la survie d'une clé optionnelle absente. Ce constat ne démontre pas
une corruption des bytes générés LinuxCNC. La politique attendue pour clés optionnelles
et compatibilité des anciens fichiers doit être décidée avant correction.
Le test matériau Cancel ne doit donc pas être présenté comme une égalité intégrale
du fichier : matériau, opérations, outils, stock et origine sont identiques ; cette
métadonnée machine diffère. Aucune modification de modèle/format effectuée.

## Régressions exécutées

macOS26.5.2 arm64, Node24, Cargo offline/locked. Journaux complets dans
[les preuves](ux-consolidation-2026-10-19-evidence/README.md).

| Contrôle | Résultat frais |
|---|---|
| `npm --prefix frontend test` final | 153 fichiers, **1387 PASS / 1 SKIP existant**, +10 tests Node WASM PASS |
| Frontend check | Svelte **0 erreur / 0 warning**, TypeScript Node PASS |
| Frontend lint / format / build | PASS ; empreinte paquet WASM vérifiée ; avertissements Vite taille/import mixte/timings conservés |
| Core lib `cps,profiling`, mono-thread | **1204 PASS** |
| `ux_inspector_contracts` | **2 PASS**, refus CPS, natif conservé, Marker/Pause |
| `safe_z_extended` / `shopbot_m2_post` | **15 / 27 PASS** |
| `cps_posts` / `vcarve_qualification` | **5 / 10 PASS** ; oracle FANUC externe absent, son test retourne sans oracle |
| `test1_properties` direct | 4 tests oracle PASS ; worker ignoré comme prévu, exécuté séparément ci-dessous |
| TEST1 optimisé, protocole 32 cas/famille +32 frontend | **256/256 PASS en27,42 s**, aucun échec/timeout |
| Collecteur de candidats, fixtures uniquement | **4 PASS**, aucun installateur produit |
| Rust fmt / clippy core `-D warnings` | PASS |
| `git diff --check` et préservation | Voir manifeste final ; aucun changement Git d'état intentionnel |

La première tentative TEST1 utilisait le worker debug : 137 PASS, 4 timeouts et33
stage_timeout, 174 résultats sur256 planifiés au plafond120 s. Résultat incomplet
conservé ; il n'est pas compté comme qualification. Le protocole exige le worker
release construit hors budget : cette relance passe, sans augmenter délais ni
réduire domaine. Full TEST1/SAFE-Z reste une porte distincte.

Une première version du correctif clavier ajoutait quatre lignes et dépassait le
budget du composant (1003>1000) ; la garde finale tient sur une ligne, sans relever
le seuil ni enlever de code préexistant. La suite complète repasse ensuite.

## Interactions et parcours rejoués

Production Vite18803 + serveur Rust fraîchement compilé18804 ; `/version` confirme
`rust-server`, beta.3, CPS disponible. Navigateur intégré, pas une application Tauri
installée. Captures et états DOM frais, sauvegardes par les vrais contrôles UI.

| Croisement | Observation |
|---|---|
| A × B : outil de l'opération | Accès à la copie projet T2 Ø3 ; diagramme et montage conservés ; revue Ø4,01 et opération touchée visibles ; édition après revue invalide Apply ; annuler/revoir/appliquer rend le programme périmé et bloque le bouton ; Undo restaure Ø3 |
| A × C : géométrie et état | Pocket→Outside padding9→Pocket conserve Union et sources ; origine XY120,0 rend le résultat périmé ; Undo restaure60,60 ; Z0 et stock haut2 inchangés |
| C : matériau | Plywood reste un brouillon, message Apply material to job explicite, recommandation désactivée ; Cancel conserve MDF et valeurs rough1100/finish1200 |
| A × D : CPS | Homing activé refuse sans programme et conserve3 opérations ; désactivation explicite permet137 mouvements, sortie .sbp et panneau **OpenSBP168 lignes**, sans faux T0 pour les opérations programme |
| C × D : rapport | Rapport courant : stock120×120×10, origineG54/60/60/0, minZ−1, rates manuelles, alertes pipeline+simulation ; Markdown réellement enregistré. Après changement d'origine : OUTDATED, statistiques/temps/minZ précédents omis |
| D : aperçu et lecture | Deux clics3D conservent3D ; masquer Toolpaths conserve le stock ; Home/End donnent1/431 et431/431, opération identifiée par les segments ; **anomalie sécurité ci-dessus** |
| B × D : fenêtres/clavier | Revue outil avec pied accessible ; rapport testé1280×800 puis1920×1080→1280×800, bornes dans le viewport et footer visible ; Échap ferme ; saisie ArrowDown/Tab de Final Z vérifiée après rebuild |
| Persistance | Vraie sauvegarde puis réouverture : stock, origine, outil, matériau, rates et Union conservés ; réserve option machine après passage CPS documentée séparément |

Les chemins FS sans référence compatible refusent d'inventer une recommandation.
Les tests automatisés couvrent aussi les sources, profondeurs spécialisées, texte sans
outil compatible, fraîcheur, cache, identité export, simulation et traduction des alertes.
Les captures claires/sombres exhaustives propres à chaque lot restent historiques :
cette consolidation ne prétend pas avoir rejoué chaque écran ni chaque combinaison.
Deux notifications ResizeObserver sont consignées ; les fenêtres restent utilisables.
Thème Auto inchangé, viewport restauré, onglet dédié et deux serveurs de revue fermés.

## Bilan candidats et décision

| Porte | État actuel |
|---|---|
| Régression logicielle intégrée A–D | PASS dans le périmètre ci-dessus, avec deux constats soumis |
| Défaut local clavier | Corrigé et vérifié, `ivac-uedp3` |
| Sécurité simulation/lecture/publication | **BLOQUANT `ivac-3xqtb`**, aucune correction autorisée ici |
| Chargement des clés machine optionnelles | **À décider `ivac-52tpd`**, sans correction implicite |
| Acceptation UX D / consolidation | **ivac-he4lt / ivac-amibd OPEN** |
| Candidats actuels | Beta.3/pipeline61, source **db5bde5d05b25328f29e27d210b7efeb8c0f429a**, preuve10/18 préservée ; ne contiennent pas A–D pipeline62 |
| Nouveaux candidats correspondants | **NON PRODUITS / non autorisés**, préparation suspendue aux décisions ci-dessus |
| Runtime natif / installation propre | Non rejoué ici sur macOS/Windows11/Ubuntu26.10 ; preuves antérieures séparées |
| Zoom système125–150 %, impression papier, atelier/physique, full SAFE-Z | Non qualifiés par ce lot |
| Publication | Interdite dans cette demande ; aucune action distante |

Prochaine étape proposée : valider cette consolidation et statuer sur les deux
constats ; seulement après résolution/acceptation des réserves, autoriser des candidats
correspondant à l'arbre figé et leurs essais natifs. Rien de cette suite n'est lancé.
