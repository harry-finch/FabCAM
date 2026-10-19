# Preuves du lot C — 19 octobre 2026

Build frontend local, revue navigateur uniquement. Voir [résultats](../UX_V1_LOT_C_2026_10_19.md).

## Captures retenues

- `zero-final-light-1280x800.jpg`, `zero-final-dark-1280x800.jpg` : neuf positions,
  origine au centre, stock/Z conservés ; grille et halo corrigés.
- `material-final-dark-1280x800.jpg` : build final, brouillon Plywood / matériau enregistré MDF,
  application recommandation indisponible et absence d'alerte vide résiduelle.
- `report-full-dark.jpg`, `report-light-full.jpg` : 1920×1080, fiche actuelle,
  contexte d'atelier et note logicielle ; contenu intérieur défilant.
- `report-alerts-final-1920.jpg` : partie opérations/alertes, 1920×780.
- `corner-focus-dom-light.jpg` : focus clavier du deuxième coin et surbrillance sans sélection,
  1280×720 ; antérieur à la correction du halo de l'origine.
- `text-blocked-light.jpg` : texte rempli sans outil compatible, ajout bloqué ; aide de police
  affinée ensuite. `geometry-light-full.jpg` : miroir/Snap contextuels, ancien halo.
- `work-zero-3d-1920.jpg` : rendu 3D 1920×780, avant fin de simulation ; marqueur en partie
  superposé aux parcours. Ne prouve aucune qualification de collision.
- `report-stale-1280.jpg` : résultats périmés supprimés, 1280×780 ; libellé devenu ensuite
  « Outdated — regenerate ».

Les autres JPG sont des captures de diagnostic intermédiaires, **pas une preuve du rendu
final** (notamment zero-1280, corner-focus-light, material-draft-1280 et premiers rapports).
Les captures plein écran à 780 px résultent de la surface native ; les tailles 1280×800 et
1920×1080 ont été capturées en fullPage et leurs dimensions vérifiées.

## Fichiers

- `preparation.ivac-project.json` : source des manipulations Frame/origine.
- `text-no-tool.ivac-project.json` : vérification texte sans outil.
- `saved-preparation.ivac-project.json` : sauvegarde réelle UI puis réouverture vérifiée.
- `report-ui-export.md` : export réel UI des valeurs et alertes ; antérieur à l'ajout final
  de la note logicielle/simulation en attente dans Markdown.
- `check.log`, `lint.log`, `test.log`, `build.log` : commandes finales, sorties non réécrites.

Fixtures de diagnostic uniquement, pas programmes à usiner. Aucun installateur ni export
machine distribué. Thème automatique restauré et serveur/onglet de revue fermés.
