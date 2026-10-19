# FabCAM V1 — consolidation UX, Phase 0

19 octobre 2026 · **Plan approuvé ; lots A/B/C acceptés, lot D livré pour revue humaine (ivac-he4lt).**
Suivi : ivac-0gc58 ; validation humaine **ivac-2tndr CLOSED**. Analyse du source `66f229ea21cccf73a76fd45d0e9f9fe3721a41a1`,
beta.3/pipeline61. Les candidats et captures proviennent de `db5bde5` ; les différences
commitées depuis ce candidat ne concernent que la documentation. Les modifications
MCP1/documentaires déjà présentes sont conservées, sans extension de leur périmètre.

Références détaillées conservées : [demande](../../../FabCAM%20—%20V1%20UX%20Audit%20Consolidation%20%26%20Polish.md),
[audit Argon complet](../../../audit-argon_UX.md),
[galerie native](../../../Captures-FabCAM-beta3-2026-10-18/GALERIE.html).
Les sept groupes couvrant01–62 et toutes les remarques Stéphane ont été lus.
[Empreintes et limites de l'analyse](ux-polish-2026-10-19/phase0-evidence.json).
Trois images originales07/27/54 ont aussi été inspectées directement ; pas de nouvelle
session native, ni qualification clavier effectuée pendant cette phase documentaire.

## Décision recommandée

Quatre lots bornés, chacun avec validation humaine avant le suivant. Corriger d'abord
les états trompeurs et les paramètres, puis les parcours de travail et enfin les
finitions communes. Conserver l'identité FabCAM, les tokens et composants existants,
Lucide, densité desktop et divulgation progressive du skill Professional Workshop UI.
Pas de refonte, changement de framework, nouvelle géométrie ou migration d'identités.

P0 désigne ici une ambiguïté dangereuse ou une intention ignorée, pas une preuve de
collision physique. Les P0 confirmés doivent être traités avant le gel RC ; les P1
réparent les parcours essentiels ; les P2 sont bornés aux composants déjà modifiés.
Les suggestions de moteur et capacités nouvelles ne sont pas absorbées dans le polish.

## Matrice consolidée

Les références donnent le groupe de captures puis la lettre de section de l'audit.
« À reproduire » signifie que la capture/observation motive le travail, sans prétendre
avoir exécuté aujourd'hui l'interaction. Les preuves de code sont détaillées plus bas.

| Groupe | Références | Constat vérifié / décision proposée | Classe · lot |
|---|---|---|---|
| États de génération et capacités |51–60 A/B,61 | `statusFor` retourne ✓ pour les contrôles programme avant les tests de fraîcheur. Homing/Probe/Marker sont ignorés avec warning CPS ; inclusion brute refusée par ShopBot. Montrer non généré/périmé/incompatible, aide selon la route réelle, aucune fausse erreur T0. | **P0 · A** |
| Paramètres réellement appliqués |41–50 A,51–60 C/D,11–20 D | Inspecteur commun et F&S différents selon le type. Construire la matrice des consommateurs, présenter Z absolu et épaisseur positive sans changer les données. **Conserver step/finish_step Chamfer et start_depth T-Slot/Dovetail.** | **P0 · A**, lisibilité P1 |
| Réglages machine/sécurité |21–30 C,31–40 B | Parking ShopBot déjà refusé ; ne pas le présenter comme inactif sans effet. Distinguer refus obligatoire ADR-0023, autres avertissements et préférence d'export. Enveloppe réelle inconnue ≠ dimensions de la démo. | **P0 · A**, politique distincte |
| Identité outil projet/bibliothèque |21–30 A/B/D | Synchronisation même ID existe mais revue préalable et historique existent aussi. Afficher nom, type, diamètre et impacts côte à côte ; ouvrir l'outil du projet dans son contexte. Ne pas changer le mécanisme de liaison sans ADR. | **P0 · B** |
| Géométrie et montage de l'outil |21–30 D,25–28 | Schéma SVG paramétrique demandé ; dimensions absentes marquées inconnues, jamais « Auto/Full » inventé. Clarifier longueur taillée, dépassement et porte-outil selon modèle Rust, valeurs conservées. | P1 · B |
| Repérage brut/origine |01–10 A,07–08 | Marqueur2D existe déjà ; visibilité/cadrage à améliorer, cohérence3D à qualifier. Distinguer déplacer le brut et choisir le zéro. Sélecteur de points doit garder une cible explicite brut/dessin et ne déplacer que XY. | P1 · C, erreur de repère confirmée → P0 |
| Inspection et navigation |11–20 A/B/C,43,47 | Contexte actif visible pendant le scroll, opérations regroupées. Frame dispose d'un disclosure mais pas de conversion inverse explicite ; distinguer replier et annuler Pocket Outside. Palette : ouvrir le propriétaire de l'action sans dépendre du panneau monté, sinon raison humaine. | P1 · A/C |
| Attaches et conversion Pocket Outside |16,47 | Réduire l'encombrement des attaches sur poche aveugle, garder accessibles les valeurs actives. Ne pas utiliser `abs(depth)<thickness` comme règle universelle ; tenir compte des Z réels. Conversion explicite, atomique et undoable, préserver sources/combinaison. | P1 · A/C |
| Recommandation matériau |17 | Éviter l'aller-retour vers Stock ; proposer choix dans la modale et recalcul sans application automatique des vitesses. Définir commit/Cancel du matériau et garder recommandations séparées des paramètres appliqués. | P1 · C |
| Texte |13,11–20 E | Trois polices embarquées + import de fichier confirmés. Outil compatible explicite ou état bloqué, aucun outil inventé ; position déterministe visible, ne pas centrer arbitrairement. Polices système demandées : qualification native dédiée. | P1 · C ; capacité polices séparée |
| Terminologie et alertes |01–10 B/C/E,34,44 | « Generate toolpaths », titre de programme d'après dialecte réellement généré ; alertes complètes, détails techniques secondaires copiables. Temps avec unités. Limite d'analyse expliquer sans conseiller d'augmenter les passes. | P1 · D, copie sécurité A |
| Aperçu et réglages |01–10 D,31–40 A/C,62 | **2D/3D seulement**, préférence Stéphane ; retirer Wire du parcours utilisateur, conserver fallback technique nécessaire et charger anciens réglages sans perte du projet. Regrouper réglages d'aperçu, texte obsolète, coût simulation et visibilité des trajets. | P1 · D |
| Fiche atelier |42 | Ajouter brut/matériau, repère auteur/programme explicite, Z réellement calculé et vitesses appliquées par opération. Jamais « Z0=haut du brut » supposé. Une fraise peut avoir plusieurs réglages de coupe ; ne pas afficher un unique feed trompeur. | P1 · C |
| Sélection géométrique |10,37,45 | Libellés de symétrie visuels sans changer axes/pivot ; accès Snap contextuel avec unité ; coordonnées coins/hover+focus plutôt que cardinal arbitraire sur contour tourné. Pas d'ajout automatique de coins incompatibles/îlots. | P1 · C |
| Finitions communes |01–10 E,21–30 D,31–40 C/D,54,56,61 | Noms longs accessibles, formulaires/textarea adaptés, casse, unités, nombres lisibles, import stylisé, aide tactile repliable, label STL/Source correct. Icône54 est un contrôle visible, pas preuve de glyph cassé. | P2 · D |
| Lecture simulation |02–03,62 | Afficher opération active ; temps uniquement si une correspondance segment/temps réelle existe. Progression actuelle fondée sur longueur, ne pas multiplier sa fraction par le temps total. | P1 borné · D ; calendrier de simulation différé |
| Moteur restant / commentaires post |04,44 | Dernière passe seule et translittération ne sont pas du CSS : qualifier séparément si retenus. Analyse filtre déjà à la profondeur cible ; garde budget distinct de TEST1. | Différé/qualification dédiée |

## Conclusions de l'audit à corriger ou déjà couvertes

- **Chamfer : hypothèse de passe unique fausse.** `pipeline/setup_resolver.rs:508–515`
  conserve le pas et le pas de finition pour la descente ; profondeur finale, départ,
  dépassement et liste explicite sont remplacés par la résolution du chanfrein.
  Montrer la largeur et la profondeur calculée, garder le contrôle de descente.
- **T-Slot/Dovetail :** le même résolveur:620–630 impose une passe au fond mais utilise
  `start_depth` pour le span ; `gcode/emit.rs` utilise ce départ pour l'entrée. Ne pas le
  supprimer. Pré-ébauche requise n'est pas un état physique que FabCAM peut certifier.
- **Thread :** `op_drivers/thread.rs:96–97` consomme départ et fond ; pitch ne remplace
  pas l'étendue Z. **Drill :** cycle/peck relèvent du driver et de `resolve_peck_step` ;
  ne pas confondre peck avec le pas de fraisage et ne pas effacer les valeurs enregistrées.
- **Origine2D :** `canvas/render/work-zero.ts` dessine cible+libellé WCS, appelé par
  `EntityCanvas2D.svelte:1781`. Constat de visibilité, pas fonctionnalité totalement absente.
  Contrat XY de SAFE-Z inchangé ; repère machine et repère programme restent distincts.
- **Outils :** `ToolLibraryDialog.svelte:460–480` offre revue avant Apply, token courant,
  liste avant/après et opérations ; `tool-library-review.ts` expose déjà les diamètres
  non rendus dans cette liste. Renforcer le contraste d'identité, pas inventer un écrasement
  silencieux ni une identité physique commune à deux outils de même numéro.
- **Safety :** ADR-0023 refuse déjà les collisions connues avec aperçu diagnostique,
  sans préférence de contournement. La proposition « seule la sauvegarde est bloquée »
  n'est donc plus exacte. Les avertissements restants ont une autre politique.
- **Parking :** `shopbot.cps:77` refuse `park_at_home` et les stations ; le regrouper parmi
  les paramètres « ignorés » donnerait une fausse promesse. Les dwell passent par les
  consommateurs communs/événements : auditer chaque champ, pas tous les CPS en bloc.
- **Remaining :** `pipeline/remaining.rs` borne `MAX_MOVES`, contrôle type/passes/stock,
  puis `at_depth` découpe les mouvements à la profondeur demandée. Le code ne manque pas
  simplement d'une projection2D. Augmenter pass depth/stepover peut changer la sécurité
  d'usinage : ne pas proposer cela comme solution par défaut à une limite de diagnostic.
- **Mirror :** `drawing/mod.rs:64–68` inverse Y pour mirror_x, X pour mirror_y ; l'actuel
  texte correspond à une symétrie autour de l'axe. Renommer humainement, ne pas permuter
  les champs JSON ni extrapoler une contradiction depuis le flip double-face.
- **Frame :** `PocketSection.svelte:45–49` contient déjà un `<details open>` repliable.
  Absence de retour Pocket visible confirmée, impossibilité réelle de fermer à reproduire.
- Génération asynchrone, fraîcheur/export, Undo/Redo, refus ShopBot, provenance/About,
  séparation recommandations/appliqué, vues Machine/Tools approuvées : **KEEP**.
  Les captures montrant des opérations temporaires ne qualifient pas leur support.

## Lots proposés et propriété des composants

**A — Vérité des états et inspecteurs (priorité P0).**
`OperationsList`, `OpPropertiesPanel`, sections d'opérations, `MachineDialog/Workspace`,
copie sécurité de Settings et contexte outil App. Une seule passe cohérente sur les
inspecteurs : regroupement des opérations, contexte actif visible, champs pertinents,
profondeur/épaisseur, vitesses effectives sans étendre artificiellement F&S à un type
non qualifié. Matrice par type×post ; distinguer indisponible et ignoré, ne pas accepter
vert une opération non générée. Invariants : sorties identiques hors changement de
refus expressément approuvé ; projets anciens lisibles, champs retenus, historique et
source bindings inchangés. Dépend de la décision CPS ci-dessous. Charge relative forte.

**B — Outil du projet, bibliothèque et montage.**
`ToolLibraryDialog`, `ToolRowExpandedEditor`, `ToolDetailBasics`, `ToolHolderEditor`,
`tool-library-review`, ouverture contextuelle depuis App. Comparaison explicite des
copies avant application, accès clair projet/local, schéma outil coté avec inconnues.
Garder brouillon, revue, copie distincte et Undo/Redo ; aucune renumérotation automatique,
aucun changement de liaison même-ID. Dépend du contexte A ; charge moyenne.

**C — Parcours de préparation et fiche atelier.**
`StockPanel`, `PocketSection`, `AddTextDialog`, modale F&S, `LayerList`, palette,
`CornerReliefPanel`/rendu, `ReportDialog` et modèle de rapport. Propriétaire unique des
changements d'origine/Frame/matériau ; groupage atomique d'actions et Cancel explicite.
Réutiliser l'import de police existant ; l'extension système suit sa décision dédiée.
Tests de sauvegarde/réouverture et reprise de projet ancien. Dépend de A/B ; charge forte,
mais sans nouveaux algorithmes CAM ni nouvelle fiche de données machine.

**D — Aperçu, vocabulaire et finitions partagées.**
`GenerateBar`, tiroir programme/App, `SettingsDialog`, `PlaybackBar`, composants de
warnings, styles/tokens/i18n communs et boutons d'import. 2D/3D stables, réglages
consolidés, alertes multi-lignes, aide/touches, typographie/nombres et labels STL.
Pas d'arrondi des valeurs persistées pour corriger seulement l'affichage. Retirer Wire
comme mode utilisateur sans supprimer une dégradation sûre du rendu ; ne pas supprimer
les anciens champs du format. Dépend de A/C, ferme la revue visuelle globale. Charge moyenne.

Chaque lot se termine par tests, captures rendues, inspection clavier et HARD STOP.
Les numéros A–D désignent cette campagne UX, pas de nouveaux jalons de release.
A/B/C/D ne sont pas lancés par la Phase0. Éviter de retoucher les mêmes sections par
petits tickets indépendants ; les composants partagés ont un propriétaire de lot.

## Décisions nécessitant accord spécifique

1. **Intention CPS ignorée (P0).** Recommandation : refus structuré avant résultat
   exécutable pour Homing/Probe actifs sur une route qui ne peut pas les représenter,
   en plus de la prévention UI, y compris projets enregistrés et API. Cela change le
   comportement warning-only et mérite une qualification moteur/adapters séparée dans A.
   Marker peut rester un avertissement explicite d'annotation non émise si accepté ;
   inclusion brute ShopBot reste refusée. Ne pas inventer des commandes G28/OpenSBP.
2. **Hors enveloppe.** Ne pas activer globalement un blocage sur des limites fictives.
   Proposer une politique fondée sur limites réellement renseignées et cadre connu,
   avec inconnues explicites, conservation des réglages existants et décision pour les
   nouveaux projets. L'activation par défaut souhaitée reste à approuver/qualifier.
3. **Polices système (préférence conservée).** Préparer un sous-checkpoint natif macOS,
   Windows et Ubuntu : énumération, lecture des octets par pont restreint, formats pris
   en charge, collections/variables, fallback et persistance portable des contours.
   Aucune promesse « toutes » avant inventaire technique ; ne pas introduire une
   dépendance silencieuse au nom d'une police installée ni un accès fichier arbitraire.
   Le navigateur garde une capacité distincte avec import explicite. Recommandation :
   qualification isolée avant engagement V1, pas reclassification automatique en polish.
4. **Matériau et Frame.** Recommandation : draft du matériau dans la modale, application
   explicite au job avant recommandation appliquée ; Cancel sans mutation cachée.
   Frame : conversion inverse explicite et undoable, ne pas remplacer `sourceCombine`
   par une valeur supposée « ancienne » non conservée. Valider ces comportements dans C.
5. **Moteur de matière restante / temps / identité globale.** DEFER : fusion/projection
   optimisée, temps de simulation exact et GUID outils nécessitent leurs propres contrats.
   La translittération des commentaires peut être un petit lot post séparé si prioritaire :
   ASCII, sauts de ligne/injection, commandes et carte des lignes restent inchangés.

## Tests et acceptation visuelle

Phase0 : revue statique + trois captures natives existantes, aucune batterie exécutée
pour une correction de code inexistante. Ne pas utiliser les anciens résultats comme
preuve d'une implémentation future. L'état de rendu « avant » est beta.3 identifié.

- A : tables de cas par opération/post et fraîcheur ; nouveau/édité/annulé/échec/cache ;
  tests positifs/négatifs CPS et sortie décodée si refus modifié ; Chamfer multipasse,
  Thread étendueZ, Drill peck, T-Slot départ, millimetres/pouces et valeurs héritées.
- B : même ID avec Ø3/Ø6, plusieurs opérations, annulation/revue devenue obsolète,
  Apply/Undo/Redo/save/reopen, copies locales/projet distinctes ; schéma avec dimensions
  absentes, nulles, invalides et holder complet. Un dessin n'est jamais un modèle mesuré.
- C : translations XY sans Z, ancien projet, attaches actives sur cas non trivial,
  Frame aller/retour/undo sans perte de binding, matériau Cancel/Apply et requête F&S
  tardive, texte sans outil/outil incompatible, rapport périmé explicitement signalé.
- D : i18n EN/DE, titres suivant sortie courante même si machine a changé, diagnostics
  bloquants longs/copier détails, anciens réglages Wire, budget solid atteint/fallback,
  aucune disparition des refus ou modification du programme pour du style.
- Chaque lot : screenshots réels desktop1280×800 et1920×1080, inspecteur étroit/large,
  zoom125–150%, noms longs et fenêtres déplacées ; thème sombre et contrôle clair.
  Tab/Shift-Tab/Enter/Espace/Escape, focus visible/restauré, disclosures, palette, labels,
  hover **et** focus ; sélection/alerte non fondées sur la couleur seule.
- Revue native sur les trois plateformes pour polices, dialogues et rendu concernés.
  Aucune validation visuelle uniquement par tests automatiques ; aucune simulation
  n'est présentée comme qualification machine. Documentation et preuves séparées par lot.

## Impact V1 et documentation

Recommandation : A et B précèdent le gel RC ; C corrige les impasses essentielles et
la préparation atelier ; D reste borné aux écrans audités. Les P2 non bloquants ne
retardent pas indéfiniment V1. Polices système et changement de politique d'enveloppe
sont des décisions explicites, pas un engagement de délai. V1 ShopBot, V1.5 ProtoMAX,
V2+ capacités avancées restent inchangés.

Les candidats beta.3 existent et les tests Windows/Ubuntu/clean-macOS peuvent fournir
un diagnostic utile maintenant. Après changements UX/guard, ils ne valideront pas le
nouveau source : reconstruire les candidats concernés avec autorisation distincte,
rejouer tests impactés puis qualification atelier. Ne pas reporter automatiquement
les observations physiques d'un programme antérieur sur une sortie changée.

STATUS/ROADMAP renvoient à ce plan sans dupliquer l'audit. DEVLOG enregistre la Phase0.
Le registre de limites/release reste autoritaire ; aucun ancien gate accepté n'est
rouvert simplement parce qu'un texte historique indique encore OPEN. Conserver le
rapport détaillé original, les captures et leurs empreintes ; documenter chaque lot
avec version/source, commandes, différences visuelles et limites restantes.

**HARD STOP — approbation du plan et des décisions de comportement avant implémentation.
Aucun commit, push, merge, publication ou changement de production.**

Lot A : [résultat et validation humaine séparée](UX_V1_LOT_A_2026_10_19.md). [Lot B livré pour revue](UX_V1_LOT_B_2026_10_19.md). C/D non lancés.
