# FabCAM — kit de validation V1

Candidat **0.9.0-beta.2**, pipeline **54**, source **2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b**.
Ce candidat permet de tester la V1 ; sa production ne vaut pas acceptation de la V1.

1. Ouvrir **PROTOCOLE-V1.md** et préparer l'environnement de test.
2. Pour un essai immédiat sur ce Mac, ouvrir **FabCAM.app** placé dans ce dossier (cela ne valide pas une installation propre). Pour le contrôle de distribution, installer le fichier de votre plateforme dans **applications/** après vérification de ses sommes SHA-256 et du `candidate.json`.
3. Tester les cas S01–S25 sur **chaque** plateforme ciblée. Utiliser les dessins dans `dessins/` et projets dans `projets/`.
4. Compléter le CSV de votre plateforme dans **resultats/** ; conserver les projets, programmes exportés, captures et messages d'erreur à côté.
5. Faire les revues humaines H01–H04 et la campagne atelier A01–A07, consignée séparément. Aucune machine n'a été exécutée pour préparer ce kit.

macOS : DMG Apple Silicon arm64. Windows : installateur NSIS x64. Ubuntu : DEB amd64.
Les architectures Intel Mac/universal et les autres distributions Linux ne sont pas qualifiées par ce kit.
Le workflow construit sur macOS 15 / Windows Server 2022 / Ubuntu 22.04 ; ce ne sont pas les systèmes hôtes de validation.

Builds : https://github.com/harry-finch/FabCAM/actions/runs/39053047404
Les artefacts GitHub sont des candidats de test, sans tag ni Release. Vérifier **ETAT-BUILDS.json** et **CANDIDAT.json** avant usage.

Les dessins sont en millimètres. Les SVG compensent l'inversion Y de l'import et ont un canevas de 140×140 mm : mesurer les contours, pas le canevas.
Le corpus `reference` provient des fixtures M1. Les dessins `regressions` sont des cas analytiques ajoutés pour cette campagne ; leur présence n'est pas une preuve de réussite.
`03-percage-quatre-points` est uniquement DXF, pour préserver de vrais POINTs (pas des petits cercles SVG).
`10-cercle-arc-rayon2.dxf` est une LWPOLYLINE fermée à deux arcs 120°/240°, centre (10,20), rayon 2 ; son SVG est un cercle de comparaison.

Les projets de démonstration et valeurs ci-dessous sont destinés aux essais logiciels. Les réglages atelier, tolérances et bridages sont choisis et validés par l'opérateur sur sa machine.
`references/offline-square.sbp` est un témoin historique : une différence avec le nouveau candidat doit être expliquée, pas écrasée ni assimilée automatiquement à une régression.
