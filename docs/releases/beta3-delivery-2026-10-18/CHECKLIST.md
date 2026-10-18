# Liste de validation V1

Voir PROTOCOLE-V1.md pour les réglages, fichiers et critères exacts. Toutes les cases sont NON TESTÉES.

## Sur chaque plateforme

- [ ] S01 — Intégrité
- [ ] S02 — Installation propre
- [ ] S03 — Mise à niveau
- [ ] S04 — Fichiers, hors ligne et interface
- [ ] S05 — Parité SVG/DXF
- [ ] S06 — GEOM1 et historique
- [ ] S07 — Origines et brut
- [ ] S08 — Profile outside/inside/on-line
- [ ] S09 — Concavité et poche avec îlot
- [ ] S10 — Poche à trois niveaux
- [ ] S11 — XOR et sources multiples
- [ ] S12 — Trait ouvert dans une poche
- [ ] S13 — Perçage/peck
- [ ] S14 — Attaches ordinaires, leads et entrée
- [ ] S15 — Attaches et géométrie combinée (lot8)
- [ ] S16 — Phase d'attaches automatiques
- [ ] S17 — Usure outil et aperçu
- [ ] S18 — Identité d'outil, historique, texte
- [ ] S19 — Valeurs manuelles / FS
- [ ] S20 — RPM de finition
- [ ] S21 — Fraîcheur, erreurs et export
- [ ] S22 — Annuler, relancer et erreurs
- [ ] S23 — Sortie et parité entre plateformes
- [ ] S24 — Performances et UX
- [ ] S25 — Capacités limitées optionnelles

## Revues humaines

- [ ] H01 — GEOM1** : examiner S05/S06/S11 et les sources originales, lineage, undo/reopen ; gate ivac-e2jnb ouverte tant que non acceptée
- [ ] H02 — FS/UX** : revue des sources/licences et des recommandations FS, S18/S19, lisibilité et annulation S22 ; conserver les gates existantes, dont les limites de latence
- [ ] H03 — Audits récents** : relire les rapports des lots4–8, leurs défauts confirmés, limites et refus ; lot7 ivac-rnf0l, lot8 ivac-v103z et autres gates restent ouverts jusqu'à acceptation
- [ ] H04 — Distribution et acceptation finale** : toutes plateformes requises S01–S24 satisfaites, écarts et S25 explicitement tranchés, documentation et risques cohérents, campagne atelier validée

## Atelier opérateur

- [ ] A01 — Dossier machine
- [ ] A02 — Contrôleur et essai à vide
- [ ] A03 — Profile et attaches
- [ ] A04 — Pocket avec îlot
- [ ] A05 — Drill/peck
- [ ] A06 — Options revendiquées
- [ ] A07 — FS4 et acceptation


## Compléments beta.3

- [ ] S26 — Largeur matière des attaches, conversion/Undo/reopen
- [ ] S27 — Collision, aperçu d'inspection et refus d'export
- [ ] S28 — Dégagement, brut périmé et HATCH
- [ ] S29 — Grande poche, mémoire et annulation
