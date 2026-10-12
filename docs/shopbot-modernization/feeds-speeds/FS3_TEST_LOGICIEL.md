# FS3 — test logiciel à valider

Ce test prend environ cinq minutes et se fait entièrement dans le navigateur.
La fixture est réservée au test logiciel : aucune exportation vers la machine.
Elle choisit un contexte de référence LongMill, pas la ShopBot du fablab.
L’application native M4-r2 ne contient pas FS3.

## Ouvrir

L’aperçu local FS3 est accessible à http://127.0.0.1:18767/?api=wasm tant que
le serveur reste lancé. Pour le relancer depuis `ivacam/frontend`, après le
build : `npm run preview -- --host 127.0.0.1 --port 18767` (Node 24).

Avec Open, charger `FS3-software-fixture.ivac-project.json` dans ce dossier.
Ouvrir Operations, sélectionner l’unique opération, puis Feeds & speeds.

## Parcours attendu

1. Cliquer Load Sienci reference data. Choisir LongMill MK2 source reference
   context, la catégorie Source softwood / soft plywood / MDF et le cutter
   quarter-inch single-flute flat upcut. Garder Prudent.
2. Sans confirmer full-width slotting, Calculate doit annoncer qu’aucun profil
   utilisable ne correspond. Cela ne modifie aucune vitesse du projet.
3. Cocher la confirmation full-width slotting pour cette fixture logicielle.
   Calculate doit proposer 17 000 rpm, avance 1 620 mm/min, plongée 810 mm/min,
   profondeur par passe 2,8 mm, chipload affiché 0,0953 mm/dent.
   La preuve est Reference data, sans mention de validation locale.
4. Ouvrir Source and provenance : source Sienci, licence, profil/dataset et
   version du moteur doivent être consultables. Apply enregistre les valeurs.
5. Dans Tool 1 · finish, passer Feed à 1 500. L’origine de l’avance doit devenir
   manual. Le profil CAM utilise ce rôle finish. Calculate seul ne doit pas
   écraser 1 500.
6. Save → Save project. Recharger le fichier téléchargé avec Open. Aucun dataset
   n’est nécessaire pour retrouver les valeurs et leurs origines.
7. Generate doit produire le programme logiciel avec avance F1500 et broche
   S17000, sans recalcul de recommandation. Ne pas utiliser ce programme
   sur une machine.
8. Changer Step / pass à -2 : les valeurs enregistrées restent présentes et le
   contexte devient obsolète. Calculate doit refuser ce DOC hors des références
   exactes fournies ; il ne doit pas écraser l’avance manuelle. Undo restaure
   l’édition via l’historique habituel.

Retour attendu : étapes OK ou numéro de l’étape et comportement observé.
Cette validation porte sur le logiciel et la compréhension de l’interface.
Les essais de coupe et presets du fablab sont FS4, après approbation de FS3.
