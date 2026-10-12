# FS3.5 — revue humaine du parcours logiciel

Aucun usinage demandé. Utiliser la version locale reconstruite :
`http://127.0.0.1:18767/?api=wasm`. Le bundle M4-r2 ne contient pas FS3.5.

Ouvrir [le projet logiciel](FS3_5-software-fixture.ivac-project.json).
Il représente explicitement le contexte de référence LongMill pour tester le
logiciel ; il ne qualifie pas le ShopBot et ne doit pas être envoyé à la machine.

1. Stock : vérifier dimensions, puis choisir le matériau de référence proposé.
2. Operations : ouvrir Profile et voir l'outil physique disponible.
3. Vérifier les paramètres courants : 10 000 rpm, avance 1 000 mm/min,
   plongée 500 mm/min et passe 2,8 mm. Ils peuvent être modifiés sans suggestion.
4. Cliquer Suggest feeds & speeds. Le contexte est déjà renseigné. Confirmer
   le rainurage pleine largeur de ce cas logiciel, puis Calculate proposal.
5. Vérifier la distinction valeurs suggérées / courantes, mode Prudent,
   données Reference data et source Sienci. Appliquer explicitement.
6. Modifier l'avance à 1 500 : Modified doit apparaître. Modifier la passe à
   2 mm : Recommendation out of date apparaît ; les vitesses restent conservées.
7. Sauvegarder et rouvrir : avance manuelle 1 500 et passe 2 restent présentes.
8. Settings → Feeds & Speeds : désactiver Sienci. L'assistant indique l'absence
   de sources ; les valeurs restent utilisables. Réactiver ensuite Sienci.
9. Tool library : vérifier que géométrie et type sont prioritaires ; ouvrir
   Fallback cutting defaults uniquement pour les valeurs héritées.
10. Machine → Tooling : vérifier Tools available on this machine, puis
    Add tool from library pour consulter l'inventaire secondaire.

Critère humain : le parcours est-il compréhensible sans connaître les datasets,
les origines internes ou les rôles rough/finish ? Signaler les libellés/étapes
ambigus. Une réponse positive valide FS3.5, pas un preset ni une coupe physique.

STOP : attendre l'approbation avant FS4.
