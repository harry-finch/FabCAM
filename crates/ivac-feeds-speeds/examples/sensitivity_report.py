"""Deterministic Markdown transition report from sensitivity example CSV (stdlib only)."""
import csv
import sys
from itertools import groupby

rows = list(csv.DictReader(open(sys.argv[1], encoding="utf-8")))
print("# FS3.9 — Diameter sensitivity measurements\n")
print("Software characterization only; source pack `sienci-router-reference` revision "
      "`2026-10-13.1`, policy `fs39-normalized-matching-1`. No machining validation.\n")
print(f"{len(rows)} samples: three modes × three scenarios, 1–14 mm at 0.01 mm "
      "plus source anchors and ±0.000001 mm around relative 10/25/50% boundaries.\n")
print("Context: softwood, explicit router-class transfer, confirmed flat upcut 1F, "
      "Pocket nominal50%, direct entry. doc1/doc10: requested DOC ceiling1/10 mm, "
      "no supplied machine limits. limited: DOC10, synthetic caps16000rpm/700feed/300plunge "
      "(not ShopBot specifications). All accepted results remain ReferenceData.\n")
print("Tables include every observed availability, quality, selected-record or source-pair "
      "transition. Δfeed compares adjacent samples, not hypothetical continuous limits; "
      "the CSV contains every sample and output. Floating-point evaluation can place an "
      "exact decimal boundary on either side; ±epsilon probes establish the two sides.\n")
for (scenario, mode), group in groupby(rows, lambda r: (r['scenario'], r['mode'])):
    print(f"## {scenario} / {mode}\n")
    print("| Diameter mm | Quality | Source mm | RPM | Feed | Plunge | DOC mm | Δfeed |")
    print("| --- | --- | --- | --- | --- | --- | --- | --- |")
    prev = None
    for r in group:
        if r['transition'] == 'true':
            delta = '—'
            if prev and prev['feed_mm_min'] and r['feed_mm_min']:
                delta = f"{(float(r['feed_mm_min']) / float(prev['feed_mm_min']) - 1) * 100:+.2f}%"
            print('| ' + ' | '.join([r['diameter_mm'], r['quality'], r['source_diameters'],
                  r['rpm'] or '—', r['feed_mm_min'] or '—', r['plunge_mm_min'] or '—',
                  f"{float(r['doc_mm']):.6f}" if r['doc_mm'] else '—', delta]) + ' |')
        prev = r
print("\n## Interpretation\n\nUnavailable below the smallest admissible anchor and above "
      "the largest admissible anchor follows the50% policy bound. The25% boundary "
      "changes the label, usually without a numeric jump beyond rounding. The10% "
      "small-tool boundary changes q² to q. At≈4.233333mm the second anchor becomes "
      "eligible and interpolation supersedes a single source; at≈5.772727mm close "
      "matching supersedes interpolation. These switches explain the larger changes. "
      "DOC1 and synthetic caps mask some changes but do not remove their cause.\n")
