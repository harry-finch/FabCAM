# Protocole exact — candidat V1 2a3fe56

Tous les résultats commencent à **NON TESTÉ**. Exécuter chaque ligne S sur les trois OS, conserver ses preuves et indiquer PASS, FAIL ou NON TESTÉ. Une ligne non testée n'est pas acceptée par défaut. Les revues humaines ne se déduisent pas de tests automatiques.

## Préparation et réglage logiciel commun

Hôtes prévus : Mac Apple Silicon (MacBook M3/macOS 14.6 déclaré, version/build exact à relever), Windows 11 x64 et Ubuntu 26.10 amd64. Relever OS, CPU/RAM, écran, version, session Wayland/X11, état propre ou profil existant, testeur et date.

Pour les nouveaux dessins, partir d'un projet neuf. Unités mm, mode fraisage, post **ShopBot fourni/CPS natif**, outil plat **D6**, 2 dents, **12 000 tr/min**, avance **600 mm/min**, plongée **150 mm/min**, Z sécurité **+5 mm**, origine travail XY=(0,0), Z0 dessus du brut ; brut 140×140×10 mm en (0,0). Profil machine de démonstration aux courses suffisantes (140×140, débattement Z couvrant -10 à +5) ; vérifier les limites affichées, ne pas maquiller un avertissement critique. Passes 1 mm, profondeur finale -3, entrée Direct, surépaisseur 0, recouvrement 50 %, compensation précisée dans le cas. Choisir sens conventionnel puis avalant pour S08. Valeurs uniquement logicielles, sans prescription de coupe réelle. Les projets enregistrés conservent leurs paramètres propres ; ne pas les remplacer par ce réglage commun.

Les intervalles mathématiques ci-dessous décrivent les géométries et le centre outil idéal. Le moteur applique des marges numériques documentées : relever les valeurs réelles, utiliser les assertions automatiques liées en références pour qualifier un écart, ne pas transformer 0,01 mm en tolérance physique générale. La simple capture 3D ne prouve pas l'absence d'une gouge : conserver le programme et inspecter ses mouvements indépendamment dans le simulateur/aperçu contrôleur.

## Installation, identité et persistance

**S01 — Intégrité.** Vérifier SHA256SUMS.txt dans le sous-dossier de l'artefact et lire candidate.json : commit 2a3fe56 complet ci-dessus, `dirty=false`, beta.2, plateforme/architecture correctes, empreinte WASM présente. macOS/Linux : `shasum -a 256 -c SHA256SUMS.txt` / `sha256sum -c SHA256SUMS.txt`. Windows : `Get-FileHash -Algorithm SHA256` pour chacun des fichiers listés et comparer les valeurs. Attendu : toutes identiques ; sinon FAIL et ne pas installer.

**S02 — Installation propre.** Utiliser un hôte/VM/compte de test sans ancien profil ; ne pas supprimer les données existantes pour simuler un état propre. macOS : télécharger le DMG via le navigateur sur le Mac propre, noter URL/quarantaine/avertissement, copier dans Applications ; vérifier `codesign --verify --deep --strict /Applications/FabCAM.app`. Build communautaire ad-hoc, non notarisé : passage normal par Confidentialité et sécurité → Ouvrir quand même si proposé, sans désactiver Gatekeeper ni retirer la quarantaine. Un simple transfert local ne couvre pas le test de téléchargement propre. Windows : NSIS, relever SmartScreen et WebView2 (installation initiale peut nécessiter Internet). Ubuntu : `sudo apt install ./<fichier>.deb`, relever dépendances et session. Attendu : nom FabCAM, beta.2, lancement puis relancement. Conserver les éventuels refus OS exacts.

**S03 — Mise à niveau.** Sur un second profil/snapshot avec ancienne version, sauvegarder un projet, un outil privé et un profil machine personnalisé. Sauvegarder les données, installer beta.2 sans les effacer. Attendu : projets et valeurs intacts, absence de confusion entre ancien nom ivaCAM et FabCAM, chemin de données stable. Faire la désinstallation/réinstallation dans l'environnement jetable uniquement ; consigner la conservation ou suppression proposée des données.

**S04 — Fichiers, hors ligne et interface.** Après prérequis installés, couper le réseau et ne lancer aucun serveur preview. Ouvrir `projets/01-carre-reference.ivac-project.json`, Generate, 2D/3D, exporter `.sbp`, enregistrer sous un chemin avec espaces/accents, fermer et rouvrir. Attendu : génération CPS native autonome, carré visible, programme non vide, pas de dépendance à localhost, dialogues natifs fonctionnels, outils/machine conservés. Tester fenêtre normale et largeur 960 px, polices, zoom, sélection, navigation 3D et messages lisibles. Conserver le programme actuel ; le témoin historique offline-square.sbp n'est pas un oracle courant aveugle.

## Import, géométrie et parcours

**S05 — Parité SVG/DXF.** Importer séparément chaque paire de `dessins/reference/` dans deux projets neufs : carré 100×100 aux coordonnées (10,10)–(110,110), rectangle 100×50, cercle centre (60,60) D50, L concave, deux rectangles séparés, contours imbriqués et polyligne ouverte. Attendu : mêmes dimensions et topologie, pas de multiplication par 25,4, cercle sans facettes grossières, ouverte non refermée (longueur 80). Afficher/cacher les calques du DXF et vérifier la sélection des seules sources voulues. Les noms de calques SVG/DXF peuvent différer.

**S06 — GEOM1 et historique.** Ouvrir projets 04 puis 05, comparer le gap et la réparation enregistrée. Importer `regressions/09-geometrie-gap-0p05.dxf` : gap 0,05 mm entre (10,10) et (10,10.05). Demander la jonction avec tolérance explicite 0,1 mm si l'outil l'offre. Prévisualiser puis appliquer, Undo/Redo, sauvegarder une copie et rouvrir. Attendu : contour fermé après réparation, géométrie originale conservée avant Apply et après Undo, aucune fermeture tacite avant réparation. Vérifier qu'une opération liée à une source supprimée/introuvable est signalée et refuse la génération, sans réaffectation silencieuse.

**S07 — Origines et brut.** Sur rectangle 100×50, générer puis déplacer l'origine XY de (+10,+20), régénérer : positions exportées relatives diminuent de (10,20), forme inchangée. Revenir à (0,0). Vérifier Z sécurité positif, profondeur négative. Demander un décalage numérique Z non nul/deux faces non qualifié : refus explicite, sans programme exploitable. Ne pas déplacer physiquement une machine pour ce test.

**S08 — Profile outside/inside/on-line.** Carré100, D6, -3 par passes1, Direct. Outside : centre outil idéal (7,7)–(113,113), pièce visée100×100. Inside : (13,13)–(107,107), On-line : (10,10)–(110,110). Z de coupe -1/-2/-3 ; Z voyage +5. Répéter avalant/conventionnel : sens change, dimensions finales non. Importer le cercle D50 : centres outil aux rayons28/22/25 suivant outside/inside/on-line ; pas de saut au raccord. Vérifier l'export et la simulation avec compensation comprise.

**S09 — Concavité et poche avec îlot.** `rectangle_with_island` : carré extérieur100×100, îlot20×20 (50,50)–(70,70). Pocket Auto, -3, Cascade puis Spiral puis Zigzag supportés. Attendu : cavité intérieure au grand contour, îlot intact, fond -3, pas de coupe de l'îlot ni traversée rapide sous le brut. Refaire sur le L concave : parcours reste dans le matériau à enlever, pas de pont coupant dans le vide concave. Conserver un export par stratégie.

**S10 — Poche à trois niveaux.** `regressions/01-poche-imbrication-3` : carrés60,40,20 emboîtés. Source All/Auto : couronne extérieure à vider et carré central20×20 à vider, anneau intermédiaire conservé. Aire géométrique cible 3600−1600+400=2400 mm² (pas l'aire balayée exacte du centre outil). Vérifier présence des deux régions, absence de coupe dans l'anneau, accord aperçu/export, -3 final. Répéter les trois stratégies de S09.

**S11 — XOR et sources multiples.** `02-xor-trois-identiques` contient trois contours40×40 superposés mais l'import actuel les chaîne en **un seul objet** (constaté avec le CLI du candidat). Ce fichier seul ne démontre donc pas la parité XOR à trois opérandes. Vérifier d'abord le nombre d'objets : un seul après cet import est le résultat caractérisé, pas une preuve du test XOR. Pour qualifier la correction du noyau, depuis le dépôt exécuter `RUSTC_WRAPPER='' cargo test --offline -p ivac-core --features cps --test geometry_source_ux xor_preserves_parity_after_empty_intermediate` : cas2/3/4 objets et vide intermédiaire, PASS requis (déjà couvert par les suites enregistrées). La revue humaine examine cette limite d'exposition UI, sans prétendre avoir exercé trois sources avec cet import. Pour l'usage UI, `05-union-disjointe-attaches` en Union : deux régions20×20 séparées, aucune coupe de liaison entre elles ; tester aussi sa combinaison XOR disjointe, qui conserve ces deux régions.

**S12 — Trait ouvert dans une poche.** `07-poche-cercle-et-trait` : carré40, cercle centre(20,20) R5 et trait(5,5)–(10,5). Pocket All/Auto, même réglage que S09 : le cercle reste îlot. Désactiver le trait, régénérer : même poche et même îlot. Répéter Cascade/Spiral/Zigzag. Un trait ouvert ne doit pas faire disparaître l'îlot.

**S13 — Perçage/peck.** DXF `03-percage-quatre-points` : centres (20,20),(60,20),(20,60),(60,60). Outil Drill adapté, -4, peck1, rétract sûr +5 (logiciel). Attendu : quatre positions exactes, plongées par étapes vers -1/-2/-3/-4, remontées conformes au cycle choisi, aucune coupe de liaison entre trous. `10-cercle-arc-rayon2.dxf`, outil D2 : centre de référence(10,20) malgré arcs120°/240°. Comparer au SVG cercle. `08-reference-percage-concave-refus` : centre bbox(5,5) hors contour ; Drill sur ce contour doit refuser et demander un point explicite.

**S14 — Attaches ordinaires, leads et entrée.** Carré100, Profile outside, -4, attaches rectangulaires largeur6/hauteur1 sur travées droites ; répéter Ramp tabs. Attendu : matière conservée jusqu'à Z=-3 aux attaches, -4 hors attaches. Leads Off puis Straight puis Arc compatibles, les deux sens, Direct : aucune traversée de pièce/îlot/bride. Une combinaison non prise en charge doit refuser, jamais changer de stratégie tacitement. Tester une Ramp d'entrée supportée sans attaches/leads ; Ramp incompatible avec ces modificateurs et Helix non supportée restent refusées.

**S15 — Attaches et géométrie combinée (lot8).** Pour réserver la place du cadre hors des contours à (0,0), placer le brut à (-20,-20), largeur/hauteur140 et origine travail XY=(-20,-20) ; les coordonnées exportées sont donc translatées de +20. D2, carré20 de `05`, poche Outside avec cadre rectangulaire marge8, -4 et attache manuelle largeur4/hauteur1 au milieu d'un côté : parcours final conserve le pont à Z>=-3. Puis Union des deux carrés disjoints, attache sur le second : elle reste présente. `06-union-intersection-attaches-refus` : Union de deux carrés se chevauchant et attache sur une source modifiée par la fusion doit refuser explicitement (contour complet non conservé). Ne pas déplacer l'attache pour transformer ce refus attendu en succès.

**S16 — Phase d'attaches automatiques.** Conserver brut/origine décalés de S15 pour laisser la place à la compensation extérieure. `04-attaches-rectangle-30x20`, Profile, D2, 4 attaches de largeur10/hauteur1 : phase uniforme espacée de25 mm sur périmètre100, centres au moins5 mm des coins, attaches préservées. Choisir ensuite 4 attaches de largeur26 mm (espacement25 mm, donc dégagement uniforme impossible) : avertissement de dégagement des coins, pas de suppression silencieuse des attaches. Mode Manual : position choisie inchangée. Relever le comportement réel si l'UI limite les valeurs.

**S17 — Usure outil et aperçu.** Carré20, outil nominal D6, usure0,2 → effectif5,8 ; Profile outside décalage idéal2,9 et non3. Cadre Pocket : limites de l'aperçu et parcours concordent avec cet effectif. Export CPS identifie diamètre5,8 quand le post expose la donnée ; la valeur nominale enregistrée reste6. Vérifier les champs si l'UI les expose ; sinon garder cette vérification au niveau automatique et la signaler dans la revue.

## Valeurs, refus et cycle utilisateur

**S18 — Identité d'outil, historique, texte.** Ouvrir `02-collision-outil` : copie projet T2 D12. Créer une copie inventaire T2 D6,35 distincte si nécessaire. Engrenage de l'opération : éditer la copie projet, Cancel ne change rien, Apply D10 change uniquement le projet. Undo/Redo, save/reopen : D10 retenu, inventaire D6,35 intact. Ouvrir `03-texte-outil-enregistre` : texte Test visible après Fit, fenêtres normale/960 px, T2 D10 conservé, génération/3D sans faux « dessin absent ». V-Carve limité : ne pas interpréter des spines comme un évidement complet.

**S19 — Valeurs manuelles / FS.** Régler avance600/plongée150/RPM12000 manuellement, ouvrir suggestion FS, Cancel ; valeurs inchangées. Apply explicite d'une suggestion choisie, override manuel, historique et save/reopen : valeurs décidées conservées. Changer matériau/RPM et vérifier diagnostics pertinents sans réécriture silencieuse des overrides ni promotion automatique de Reference à Tested/Validated. Revoir les sources/licences et la portée du conseil avec H02.

**S20 — RPM de finition.** Pocket avec même outil, RPM ébauche17000 et finition12000. Generate ShopBot : refus explicite du changement spindleSpeed non pris en charge, aucun programme de succès partiel. Revenir aux mêmes RPM : génération possible si tous autres paramètres sont supportés. Les deux valeurs enregistrées ne doivent pas être unifiées pour masquer le refus.

**S21 — Fraîcheur, erreurs et export.** Après génération, modifier séparément diamètre, profondeur, source, origine, bride puis Undo/Redo. Sortie marquée périmée et export bloqué jusqu'à régénération (ou réutilisation de cache prouvée identique). Ajouter une bride traversant le trajet y compris voyage ; un avertissement critique bloque l'export, pas de promesse d'évitement automatique. Tester source vide et fichier texte non SVG/DXF : message compréhensible, pas de plantage ni export de l'ancien résultat.

**S22 — Annuler, relancer et erreurs.** Sur un travail assez long (dupliquer la géométrie ou texte long), démarrer, annuler avant fin, modifier un paramètre et relancer. Attendu : interface utilisable, temps/progression lisibles, zéro sortie partielle exportable et pas de résultat ancien qui remplace le nouveau. Refaire20 cycles ; noter temps, mémoire et anomalies. Si tâche finit avant Cancel, noter « annulation non exercée » et utiliser une charge plus longue.

**S23 — Sortie et parité entre plateformes.** Pour les mêmes projets sauvegardés S08/S09/S13, exporter depuis chaque OS avec mêmes options/post. Archiver fichiers et hash ; comparer mouvements, unités, origines, profondeur, vitesses coupe/plongée, retraits et ordre d'opérations dans un décodeur/aperçu indépendant. Les fichiers identiques constituent une preuve forte ; différences de commentaires/chemins doivent être isolées et expliquées, écarts de mouvements non expliqués = FAIL. Contrôler que l'aperçu affiché correspond bien au programme exporté.

**S24 — Performances et UX.** Relever lancement à froid, première génération, cinq générations chaudes avec le même projet et20 cycles pour mémoire. Même machine, fixture et cache pour comparer. Référence PERF1 copiée dans references/perf1-requests (requêtes API, pas des projets ouvrables directement), distincte du corpus de projets : ne pas prétendre valider ses neuf cas avec un carré. Proposition de régression >20 % sur médianes comparables à investiguer ; seuils PERF1 liés à leurs hôtes, pas une promesse sur chaque PC. Relire les observations UX, 3D, clavier, focus et champs de S04/S18/S22, sans clôturer les gates sur un chrono seul.

**S25 — Capacités limitées optionnelles.** Si incluses dans les revendications V1 : surfaçage raster X/Y avec marge/overtravel explicite sur rectangle100×50 (couverture et retraits), dogbones sur L concave (géométrie et diamètre cohérents, Undo), V-Carve Perimeter et Full sur projet texte (profondeur plafonnée et aperçu lisible). Conserver les refus d'associations non qualifiées et désigner explicitement ces fonctions « support limité/logiciel » si elles ne passent pas d'essai atelier. Deux faces, WJ, VINYL, inlays/rest avancés et V-Carve intégral arbitraire ne font pas partie des promesses V1.

## Revues humaines nécessaires avant go/no-go

**H01 — GEOM1** : examiner S05/S06/S11 et les sources originales, lineage, undo/reopen ; gate ivac-e2jnb ouverte tant que non acceptée.

**H02 — FS/UX** : revue des sources/licences et des recommandations FS, S18/S19, lisibilité et annulation S22 ; conserver les gates existantes, dont les limites de latence. Le seuil proposé200 ms n'est pas atteint par l'ancienne mesure p95216/218 ms ; trancher la portée sans falsifier les mesures.

**H03 — Audits récents** : relire les rapports des lots4–8, leurs défauts confirmés, limites et refus ; lot7 ivac-rnf0l, lot8 ivac-v103z et autres gates restent ouverts jusqu'à acceptation. S15 ne qualifie pas la résistance physique universelle d'une attache.

**H04 — Distribution et acceptation finale** : toutes plateformes requises S01–S24 satisfaites, écarts et S25 explicitement tranchés, documentation et risques cohérents, campagne atelier validée. Aucun build vert, version beta ou case logicielle ne donne automatiquement le feu vert V1.

## Campagne atelier — à faire par l'opérateur après validation logicielle

Les fichiers logiciels servent à préparer les coupons ; aucun .sbp de démonstration n'est un programme prêt à usiner. Remplacer profil et vitesses par des paramètres approuvés, confirmer brut/outil/origines/bridage et régénérer. **Avant coupe**, écrire dans le CSV atelier les tolérances acceptables XY, profondeur, planéité/jeu si concernés, instruments et paramètres. Aucun seuil physique inventé par ce kit.

**A01 — Dossier machine.** Relever contrôleur/version, machine et courses réelles, outils mesurés/saillie, matériau/épaisseur, bridage, zéro XY, Z dessus, départ broche manuel et changements d'outil. Archiver programme final/hash et photo du montage. Attendu : cohérence avec le travail réel.

**A02 — Contrôleur et essai à vide.** Inspecter le programme exporté et son aperçu contrôleur, bornes et signe de Z, voyages et serrages ; essai à vide supervisé approprié avant matière. Attendu : aucune collision/surcourse ni départ broche supposé. Consigner les observations.

**A03 — Profile et attaches.** Carré100/rectangle100×50, intérieur et extérieur, passes, sens retenus, attaches et transitions supportées sur matériau sacrificiel. Mesurer dimensions externes/internes, profondeur et épaisseur des ponts, observer tenue et états des chants. PASS seulement selon les tolérances convenues avant l'essai.

**A04 — Pocket avec îlot.** rectangle_with_island, mesurer cavité/fond/îlot, vérifier îlot préservé et comportement entrée/engagement. Documenter avances/RPM réels et observations FS4. Ne pas généraliser à toute topologie.

**A05 — Drill/peck.** Quatre points20/60, outil de perçage réel adapté : mesurer positions et profondeurs, observer pecks/retraits et dégagement. Relever l'outil réel et le cycle. Pas de liaison de coupe entre trous.

**A06 — Options revendiquées.** Si support atelier annoncé, surfacer zone bornée (planéité/limites), dogbone (ajustement), V-bit (groove/spines et profondeur). Sinon délimiter leur statut logiciel/limité dans la décision finale ; ne pas inventer une validation physique.

**A07 — FS4 et acceptation.** Sur Profile/Pocket au minimum, enregistrer contexte machine/outil/matière, engagements, avances/plongée/RPM résolus, mesures et observations. Un essai ne promeut pas automatiquement un preset à Validated. Anomalies corrigées puis nouveaux essais concernés ; décision humaine go/no-go, date et portée explicitement signées.

## Preuves à conserver

Chaque ligne : statut, testeur/date, fichier d'entrée/hash, paramètres ou projet sauvegardé, résultat mesuré, programme/hash, capture/log et anomalie/issue. Utiliser les trois CSV plateformes et le CSV atelier. Non testé et non applicable doivent être justifiés séparément ; conserver les échecs précédant une correction. Ne modifier aucun résultat en PASS sans exécution.
