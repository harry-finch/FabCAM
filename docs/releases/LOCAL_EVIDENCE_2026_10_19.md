# Local-only evidence — 2026-10-19

Per-case TEST1 artefacts from the 2026-10-19 checkpoints are kept **outside Git**
(no retention policy authorizes tracking them; 8 211 files, ~559 MB). Committed
reports keep their summaries, logs, screenshots and `SHA256SUMS.txt` where the
checkpoint produced one; every TEST1 case can be replayed from its seed with
`scripts/test1/run.py --replay`. Each row's aggregate digest is the SHA-256 of the
sorted `sha256  path` lines of its files, so a local copy can be verified.

| Local directory | Files | Size | Aggregate SHA-256 |
| --- | ---: | ---: | --- |
| `docs/releases/tool-provenance-implementation-evidence/test1` | 1536 | 119.5 MiB | `dbd60c1789f34bf8…` |
| `docs/releases/tool-provenance-implementation-evidence/test1-accepted` | 1536 | 119.5 MiB | `112c308c9beaa2fd…` |
| `docs/releases/tool-provenance-implementation-evidence/test1-final` | 1097 | 58.0 MiB | `f13308c93b6c491d…` |
| `docs/releases/ux-consolidation-2026-10-19-evidence/test1` | 965 | 38.6 MiB | `33cd91ccba29454e…` |
| `docs/releases/ux-consolidation-2026-10-19-evidence/test1-release` | 1536 | 111.6 MiB | `43a7af6d474e7ffc…` |
| `docs/releases/ux-safety-load-2026-10-19-evidence/final` | 1541 | 111.8 MiB | `454127c31976eedd…` |
