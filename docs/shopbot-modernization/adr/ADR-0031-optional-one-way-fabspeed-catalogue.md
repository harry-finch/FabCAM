# ADR-0031: Optional One-Way FabSpeed Catalogue for FabCAM

Status: **Proposed** — product-level principles below explicitly approved by the
maintainer's roadmap-consolidation request; implementation contract not accepted.
Date: 2026-10-24
Scope: optional V1.2 integration; no production implementation.
[Release plan](../../releases/RELEASE_ROADMAP.md#v12--optional-one-way-fabspeed-integration) ·
[Accepted separation ADR-0030](ADR-0030-fabcam-fabspeed-separation.md).

## Context

FS-RESET removed the integrated recommendation engine. FabCAM now owns explicit
per-actual-role operation parameters, independent CAM validators and local tested
collections. FabSpeed is a separate calculator/experimentation application. The
maintainer approves optional automatic **one-way FabSpeed → FabCAM** integration,
with independent local resources and local CAM extensions. No FabSpeed storage,
API completeness or service availability has been audited in this task; those
facts cannot be assumed from this product decision.

## Decision — approved product principles

| Resource / responsibility | Authoritative owner |
|---|---|
| Calculation, optimization, experiments/test history and FabSpeed preset validation | FabSpeed |
| Shared technical machine/tool/material catalogue and source presets | FabSpeed for FabSpeed-origin common records |
| CAM machine configuration, operations, explicit actual-role values, paths, simulation, validation and export | FabCAM |
| Independent local machine/tool/material/preset resources | FabCAM |
| CAM-specific extensions on linked resources, stable local references and cache | FabCAM; upstream common fields remain owned by FabSpeed |

**Strict direction:** FabCAM may read identifiers and retrieve/cache FabSpeed
records. It must not write the upstream database/catalogue, submit experiments,
upload CAM project/operation payloads, create/validate upstream records or introduce
bidirectional synchronization. An identifier-based read is allowed. If an existing
calculation API requires operation context submission, stop for a separate product
contract decision; do not silently enable reverse business-data flow.

**Synchronization UX:** transparent background synchronization when data is
available, initial startup synchronization where feasible, reconnect or bounded
refresh, optional Settings “Synchronize now”, visible last-sync/failure state and
persistent offline access. FabSpeed need not run continuously. Mechanism, schedule
and process lifecycle remain OPEN; no daemon/API/file-access choice is made here.

**Identity and field ownership:** retain stable FabCAM local IDs for existing
projects and stable shared FabSpeed IDs. Explicitly link local and upstream records
without destructive project-reference rewrites. Names, diameter or model do not
prove physical identity. Separate upstream common fields, local CAM fields,
synchronization metadata and effective safety ratings. Never overwrite a resource
wholesale or discard local CAM configuration during refresh.

**Autonomy and application:** local standalone resources and tested-preset
collections remain supported. Generation, simulation, validation and export never
require live FabSpeed. Synchronization does not mutate authored operation values.
Copy a compatible preset only after explicit review and local CAM validation,
preserving actual-role assignments and independent executable values. Changes to
relevant machine/tool safety limits invalidate or requalify affected generated
results, rather than leaving stale output presented as qualified.

**Provenance:** FabSpeed owns its preset status and experiment history; FabCAM may
display them but never invent, upgrade or convert them into a local workshop-test
attestation. A calculated proposal is not an experimentally validated preset.
Propose an exact compatible validated preset first. Existing proposals may be
retrieved only through an approved read contract, shown as untested unless real
validated evidence says otherwise, previewed and validated before explicit copying.
Suggestions may remain accessible alongside presets with that distinction clear.
No automatic application, reintroduced engine or automatic validation claim.

## INT-0 technical decisions — OPEN

INT-0 must inspect the real FabSpeed and FabCAM models and produce a reviewed
contract for:

- Shared/local identifier namespaces, linking/relinking and migration-safe reference
  preservation; units, schemas, versions and cross-application compatibility.
- Exact common/local field boundaries, editable permissions, current effective
  safety limits, precedence and generation invalidation/requalification ownership.
- Update/version/conflict handling, upstream removal/revocation and retention of
  operation provenance; distinction between stale, unavailable and withdrawn data.
- Local cache persistence, atomic transaction scope, concurrent updates, failure
  recovery, bounded resources, offline/restart and reconnect behavior.
- Upstream trust/authentication/access scope, source rights, schema/size validation,
  provenance integrity and untrusted-input handling. Local transport alone is not
  evidence of trust.
- Actual API/storage capabilities and read-only access: local API, background
  service or published catalogue, especially when FabSpeed is closed. Do not invent
  endpoints, protocols, current shared IDs or an always-running service.
- Minimal hybrid UX, explicit resource linking and status/recovery disclosure.
- INT-3 feasibility: whether previously calculated proposals are readable by ID;
  if not, keep the API/UX product decision OPEN without uploading CAM context.

INT-0 is documentation/architecture research and may run alongside V1.1
qualification by separate authorization. It changes no production code. The
present task prepares this ADR; it does not claim that INT-0's audit is complete.

## Alternatives considered

- **Bidirectional sync or uploading operation context:** rejected by the approved
  one-way business-data policy.
- **Embedded calculator or mandatory online lookup:** rejected; violates separation
  and FabCAM runtime autonomy.
- **Replace local libraries with upstream resources:** rejected; loses standalone
  ownership and can destroy local references/CAM extensions.
- **Name/geometry-based automatic identity inference or wholesale updates:** rejected;
  cannot establish physical identity or preserve field ownership safely.
- **Manual file exchange as the sole product experience:** insufficient for approved
  automatic synchronization UX; a published read-only snapshot may still be a
  transport if INT-0 proves refresh/offline/recovery semantics.

## Consequences and validation

This is an additive future adapter/cache boundary, not permission to change v9,
the preset schema or frozen V1. Any required persistence/model change needs its
own reviewed compatibility/migration decision. Reuse explicit parameter validation,
operation history/application and publication guards; do not wrap a new calculator
inside CAM. No chosen database, endpoint, shared schema or dependency is implied.

INT-0 acceptance requires evidence-backed contracts and explicit architecture
review. INT-1/2 must prove ownership-preserving refresh, stable references, offline
and missing-service operation, transactional failures, withdrawal and safety-limit
invalidation, followed by hybrid UX acceptance. INT-3 requires a reviewed read
contract, tested-vs-calculated provenance, no upstream payload submission and no
automatic apply/attestation. Full source-bound CAM/security regressions and separate
platform/physical decisions remain required for the resulting release.

The V1.2 path is INT-0 → INT-1 → INT-2 → INT-3, with synchronization code on a
separate future branch from an appropriately qualified V1.1 baseline. None of
INT-1/2/3 blocks V1.1 acceptance. **HARD STOP: proposal only; no sync implementation.**
