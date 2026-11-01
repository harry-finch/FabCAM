# Cutting values and tested preset collections

## Author explicit operation values

FabCAM owns explicit RPM, feed, plunge and depth per pass. Tool defaults may seed
new operations once; editing a library default does not rewrite existing operations.
Machine/tool validation is independent of any tested record or calculation service.
Profile and ordinary Pocket show **Main Cutting**, with **Add a finishing stage** OFF
by default. Finishing needs its own actual tool, rates, DOC and applicable allowances.
Pocket supports Walls / Floor / Walls and floor under the strict v2 stage contract.
Other families retain their actual role assignments. See [CUTTING_CONTRACT](CUTTING_CONTRACT.md).

Incomplete values may be saved for editing but cannot generate. Known limit violations
refuse; unknown limits are disclosed, not silently invented. Geometry, entry, coverage
and ShopBot transition restrictions still apply after loading any numbers.

## Save a tested configuration

Set stable workshop machine/tool identities, choose the actual Stock material and
complete the operation values. **Save preset** explains missing context. Select or
create a named collection and meaningful entry name. Explicitly attest a workshop
test only if it actually occurred on that recorded context. Software validation and
another author's claim do not establish physical testing on this machine. Synthetic
QA records must never be advertised as workshop recommendations.

There is no Draft/Trial promotion, manufacturer ranking, interpolation or source-pack
recommendation runtime. FabSpeed is independent; optional exchange is planned only.

## Strict tested loading

**Load preset** reviews recorded context and proposed replacement values. Strict
loading requires exact compatible machine/tool/material/operation signatures and
actual stage/role assignments; a matching local tool number is insufficient. It does
not silently switch machine, cutter or material. A context refusal preserves the
original record and author claim; Recorded compatibility context is read-only saved
information, not reconstructed current machine data. Imported attestations are not
renewed by viewing/loading.

Loaded numerical snapshots survive library edits/deletion. Modified values are
marked as modified from the preset; Undo/Redo restores the authored state. Regenerate
and requalify the full program after changes before export.

## Reviewed starting-value reuse — ALPHA-4

The separate reviewed-copy path makes a **numerical starting point**, including
across differing contexts; it does not loosen strict tested loading. It accepts
supported **V2 staged Profile/Pocket** sources and explicitly selected RPM/feed/plunge/
DOC fields from source Main or Finishing. Choose target Main or an already-enabled
Finishing stage. It does not create Finishing, replace tools/material/geometry,
scale numbers, calculate chipload, change finishing scope/allowances or reuse stepover.

The comparison/explicit review precedes one atomic history command. V10 save/reopen
preserves advisory `fabcamPresetValueReuse` metadata. That metadata is not CAM input,
a live library reference, a compatibility certificate or a target physical attestation.
Source-context testing never transfers. A new tested target preset requires a fresh,
truthful explicit attestation after the actual target trial. See
[ALPHA-4 qualification](releases/ALPHA_4_REVIEWED_PRESET_REUSE.md).

## Manage and transfer collections

**Presets** manages named local collections and import/export. Current staged data
uses `fabcam-preset-collection` **V2**. Historical V1 is preserved/inspectable;
V1 Profile/Pocket Rough/Finish cannot apply to V10 Main/Finishing. Unchanged families
may retain their exact matching behavior. Existing V1 collections are not silently
converted. Neither format is an old recommendation source pack or experimental recipe.

Both versions are limited to **2 MiB and 256 presets per collection**. Collision
choices are explicit; updates detect stale writes. Browser storage and native fixed
application-data storage use their existing bounded persistence contracts. Native
file selection grants remain subject to SEC-HARDEN-1; installed-native acceptance
is still unqualified. Keep backups and original author/provenance/licensing context.

ProjectFile **10** is distinct from collection V2 and native cutting V10. Portable
V1–9 projects are refused, with originals untouched; suffix compatibility does not
mean old content is migrated. [STATUS](shopbot-modernization/STATUS.md) and the
[security report](releases/SEC_HARDEN_1_APPLICATION_SECURITY.md) distinguish software
qualification from native/platform/physical acceptance.
