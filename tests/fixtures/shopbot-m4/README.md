# Premier test du logiciel — hors machine

Ce dossier contient un projet déjà configuré et son programme OpenSBP de
référence. **OFFLINE DEMO ONLY** : les paramètres numériques proviennent des
tests logiciels M2, pas d'une qualification de la ShopBot du laboratoire.

1. Lancer `ivaCAM.app` dans le dossier de livraison `FabCAM/ivaCAM-M4`.
2. Cliquer **Open** et choisir `offline-square.ivac-project.json` dans ce même dossier.
3. Vérifier le carré de **100 × 100 mm**, situé de (10,10) à (110,110),
   le brut virtuel **140 × 140 × 10 mm**, l'outil virtuel Ø6 et l'opération de
   contour extérieur. Le post du projet doit être **CPS / ShopBot OpenSBP MVP**.
4. Cliquer **Generate G-code**, puis consulter les vues **2D**, **3D** et le
   panneau **G-code**. Le programme commence par un commentaire ShopBot,
   `VD,,1`, `SA` et des `PAUSE`. Ses profondeurs sont −2, −4 puis −5 mm,
   les coordonnées XY du centre de l'outil restent entre 7 et 113 mm.
5. Ouvrir le détail des avertissements et noter leur texte exact. Les durées
   estimées ne comprennent pas les interventions manuelles. La simulation CAM
   ne reproduit pas les pauses, la broche manuelle ou chaque corde du post.
6. Si l'export est autorisé par les réglages de validation, vérifier que le
   bouton et le sélecteur proposent **`.sbp`**. Sauvegarder uniquement une copie
   locale pour la comparer à `offline-square.sbp`. Si un avertissement bloque
   l'export, conserver son texte pour le retour de test.
7. Sauvegarder une copie du projet, fermer/recharger et confirmer le maintien
   du post, de l'outil, des dimensions et des profondeurs.

Retour attendu : démarrage OK/échec, chargement OK/échec, génération OK/échec,
vues 2D/3D, post/extension affichés, avertissements exacts, export éventuel,
rechargement et comportement inattendu. Fournir une capture si utile.

Le profil réel `sorbonne-shopbot-prs.ivac-machine.json` est fourni séparément.
Son nom marque les paramètres à vérifier et son dégagement nul doit provoquer
un refus de génération ShopBot. Il ne contient pas les paramètres de la démo.

**Ne pas envoyer le programme de démonstration à la machine.** Les essais
physiques suivent le [protocole M4](../../../docs/shopbot-modernization/M4_VALIDATION.md)
après vérification du relevé machine et préparation d'un programme dédié.

## Référence physique r2 du 12 octobre 2026

Le sous-dossier `r2` conserve le projet et le SBP exacts de l’essai physique
accepté par l’opérateur, avec le manifeste de compilation et la provenance des
macros inspectées (sans leur code). Le manifeste est celui de la livraison
initiale et indique donc encore la qualification alors en attente ; le résultat
ultérieur fait foi dans M4_VALIDATION.md et M4_VCARVE_COMPARISON.md.
Ce mode broche est spécifique au câblage du laboratoire, outil 1 préinstallé.
Aucune qualification générale d’autres machines ou montages n’est impliquée.
