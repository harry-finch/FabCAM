# Audit indépendant PERF1-A — 2026-10-15

## Avis

PERF1-A fournit une base crédible pour sélectionner les investigations suivantes.
Les conclusions de coût sont étayées ; aucune preuve vérifiée ne justifie de
réécrire le moteur géométrique. Accepter A comme diagnostic, avec les réserves
ci-dessous. Ce document ne vaut ni acceptation humaine ni autorisation de B.

Audit du rapport, scripts, sondes natives/frontend, pipeline CPS et observations
conservées. Pas de nouvelle campagne chronométrée, de rerun de la suite complète,
de modification de production ni de qualification physique.

## Vérifications indépendantes

- 18 fichiers de requête : SHA-256 recalculés, concordants avec le manifeste et
  les deux séries natives. 7 appels dans chacune des 36 séries ; statut uniforme.
- 14 réponses retournées : empreintes complètes contrôle/profilage identiques,
  stabilité déclarée sur chaque répétition. 4 séries de refus : même message sur
  toutes les répétitions contrôle/profilage.
- 7 paires de réponses natives/WASM encore présentes dans le répertoire temporaire :
  programmes identiques et nombres de mouvements identiques, vérifiés directement.
- Médianes recalculées : Test CPS contrôle 903,525 ms ; profilé 781,950 ms, dont
  769,415 ms CPS (98,40 %). Many-segment : 99,60 % CPS. Pocket : 328,745 ms
  d'analyse de matière restante, soit 83,01 % du pipeline profilé.
- La séparation process-cold / warm, les spans inclusifs, les frontières différentes
  WASM/native, les refus, l'absence de mesure physique et le caractère substitutif
  du projet Test sont explicitement et correctement décrits.

## Constats et améliorations proposées

### P2 — La stabilité des séries doit échouer explicitement en cas d'échec

`scripts/perf1-report.py:41` utilise `get("identical_to_first", True)` ; les
appels en erreur ne portent pas ce champ. Une première réussite suivie d'un échec
peut donc produire `warm_output_stable: true`. La classification et la comparaison
des refus ne consultent également que le premier appel. Contre-exemple minimal
évalué pendant l'audit : succès identique + échec => stabilité vraie.
Les séries présentes ont toutes été vérifiées et ne présentent pas ce défaut.

Proposition : vérifier le statut de chaque répétition ; comparer toutes les
réponses réussies et tous les refus structurés ; représenter absent/non applicable
explicitement. Ajouter des tests de synthèse succès→échec, refus→succès,
message de refus différent, échantillon manquant. Toute divergence doit empêcher
une conclusion de conformité, indépendamment des performances.

### P2 — Les preuves de comparaison ne doivent pas dépendre silencieusement de /tmp

`scripts/perf1-report.py:52-64` recherche les réponses dans un chemin temporaire
fixe ; leur absence supprime simplement les résultats de comparaison lors de la
réécriture du résumé. Leur présence n'est pas reliée aux empreintes des observations
chargées : un ancien fichier peut donc être associé à une nouvelle mesure.
Le rapport documente cette dépendance ; elle reste fragile pour la reproductibilité.

Proposition : chemin de preuves paramétrable, identifiant de campagne et empreintes
liant chaque comparaison aux requêtes, binaires et réponses. Conserver un petit
rapport de comparaison durable ; si les réponses sont absentes, afficher
`not_verified`, jamais une preuve implicite. Vérifier la taille des parcours avant
le calcul de delta (`zip` tronque au plus court, même si un booléen séparé existe).

`scripts/perf1-native.py:66` recopie aussi le hash du manifeste sans recalculer le
fichier exécuté. Les fichiers présents concordent, mais une prochaine modification
manuelle peut être mal attribuée. Calculer et vérifier le hash avant lancement ;
ne lire une ancienne réponse auxiliaire que si la campagne courante l'a produite.

### P2 — Définir le gain utilisateur attendu du cache avant de retenir B1

Le cache de résultat complet est un candidat raisonnable pour une régénération
strictement identique. Il ne réduit pas le coût du premier calcul ni celui d'une
modification qui change l'IR. Le benchmark actuel répète principalement une requête
inchangée : un excellent résultat B1 ne suffira pas à conclure que les itérations
usuelles profondeur/outil/texte sont devenues rapides.

Conserver les objectifs proposés pour les hits, mais ajouter : première génération,
changement réel puis génération, répétition identique, aller-retour A→B→A et éviction.
Mesurer explicitement hit/miss, coût de clé/copie, mémoire retenue et absence de
régression significative sur miss, dans des séries contrôle/optimisé entrelacées.
Prévoir plusieurs processus/sessions ; n=5 suffit à orienter A, pas à caractériser
un percentile de queue ou une croissance mémoire.

### P2 — La fin de simulation mérite une mesure dédiée avant de promettre une preview rapide

Le premier frame (~935 ms) et la fin du replay (~3531 ms) ne représentent pas le
même état utilisateur. Les fins de replay des runs retenus passent de 2309,9 et
2611,3 ms à 3531,2 / 4050,5 / 3970,3 ms. La corrélation avec l'ordre des appels
n'établit pas une fuite, mais justifie une séquence propre dans une page fraîche.

Ajouter des marqueurs distincts : timer programmé/exécuté, chargement driver,
construction/configuration, replay, LOD, première frame après replay. Enregistrer
résolution, cellules, playhead et mode d'affichage. Répéter sans autres benchmarks,
puis refaire après rechargement. Le gain de 50 % sur première frame reste une cible
valable, à compléter par la latence de relief utilisable et sa stabilité.

## Stratégie d'architecture proposée

- **KEEP** : géométrie, tolérances, validations V-Carve, séparation IR/post/preview,
  traitement explicite des refus. Pas d'assouplissement pour obtenir des gains.
- **EXTEND, sous approbation** : réutilisation bornée en mémoire du `PostOutput`
  réussi pour le seul post ShopBot embarqué audité. Insérer autour de `run_post`
  (`pipeline.rs:758`), après génération/validation de l'IR ; conserver ensuite le
  décodage, les diagnostics et les contrôles de fraîcheur existants.
- Clé : contenu source + nom, IR complet, propriétés, identité runtime/prélude/IR
  et budgets effectifs. Ne pas se limiter au nom « shopbot » ou au nom du projet.
  Distinguer les dépendances du post des champs amont : un changement de stock qui
  ne change aucun input réel du post ne doit pas forcément invalider son résultat,
  mais les analyses/fraîcheurs du pipeline doivent rester correctes.
- Borne en octets/entrées, éviction, cache vide au redémarrage pour une première
  version, aucune insertion sur erreur/annulation, contrôle d'annulation avant
  retour d'un hit. Conserver l'intégralité des diagnostics et métadonnées ; tester
  concurrence, refus, propriété de spindle, post personnalisé et provenance des lignes.
- **Mesurer avant de remplacer** : `run_post_with_limits` (`ivac-cps/src/run.rs:282`)
  reconstruit le moteur, évalue les scripts, convertit l'IR puis exécute et extrait
  la sortie. Sous-profiler ces étapes permettra de choisir un éventuel gain sur
  les misses sans lancer prématurément une réécriture de VM ou de post.
- **DEFER séparément** : cache/analyse restante Pocket et interruption CPS bornée.
  La latence d'annulation observée doit être conservée comme critère de suivi ;
  un hit rapide ne résout pas l'annulation d'un calcul nouveau.

## Décision recommandée

Valider A comme base d'investigation. Avant B, durcir les preuves ci-dessus et
préciser les scénarios hit/miss ainsi que les mesures de replay. Puis autoriser
éventuellement B1 seul, borné et réversible, avec une décision d'architecture courte.
Ne pas engager B2, une nouvelle VM ou des changements géométriques par cette revue.
Le gate humain ivac-xzys reste ouvert. Aucun commit/push ni PERF1-B exécuté.
