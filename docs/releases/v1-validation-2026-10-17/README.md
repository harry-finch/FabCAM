# V1 candidate validation — 2026-10-17

User authorized three-platform candidate builds and a root-level test kit.
Engineering task: ivac-ei85n.

Source: 2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b, 0.9.0-beta.2, pipeline54.
[Manual desktop-candidates run39053047404](https://github.com/harry-finch/FabCAM/actions/runs/39053047404).
macOS arm64 DMG, Windows x64 NSIS and Ubuntu amd64 DEB; no tag or Release.

Kit: `FabCAM/Validation-V1-2a3fe56/` beside the Git checkout `ivacam/`.
Contains35 SVG/DXF files, five saved projects, analytic expectations, PERF1 requests,
raw CLI-import checks and blank per-platform/physical result records.
All35 drawing imports succeeded using the current debug CLI; paired bounding boxes
agree within1e-8mm arithmetic allowance. This is import qualification only.
Overlapping duplicate contours chain into one import object; three-operand XOR is
therefore explicitly kept as a separate automated kernel check, not a false UI proof.

[French detailed protocol](PROTOCOLE-V1.md): S01–S25 per OS, H01–H04 human reviews,
A01–A07 operator workshop campaign. Physical thresholds/rates remain operator
decisions, recorded before cutting. Existing evidence is historical and current
result records start NOT TESTED. Build/runtime/install/machine acceptance remain
separate. macOS clean browser-download acceptance is still required; locally copied
CI artifacts do not establish download/Gatekeeper qualification.

All three CI jobs succeeded. DMG, NSIS EXE and DEB are downloaded into the root kit,
with their original candidate manifests and checksum files. All source SHAs equal
2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b; dirty=false; all checksums PASS.
The DMG integrity and copied FabCAM.app strict signature/arm64 identity PASS.
The DEB control/version/amd64 and native ELF x86_64 header PASS; the Windows
installer is a PE NSIS candidate with declared x64 payload, not runtime-qualified.
FabCAM.app is copied at kit root without replacing an installed app or launching it.
`ETAT-BUILDS.json`, `APPLICATIONS-VERIFIEES.json`, `CANDIDAT.json` and
`SHA256SUMS-KIT.txt` retain the result/provenance. No old app is relabelled.

macOS DMG: b31a971606733ccb2e8ff78c0f661d068bee9a55e72b0834a8bb1635830cea4f
Ubuntu DEB: 7e91cb231121426d4db7838a4b4928112e71cb7cf080aba4ed6251fde09c1d13
Windows EXE hash is recorded in APPLICATIONS-VERIFIEES.json.

All manual result rows remain NOT TESTED; human/physical/platform gates stay open.
No source commit/push, tag, Release, preview server or machine execution in this task.
