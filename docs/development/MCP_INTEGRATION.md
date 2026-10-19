# MCP integration — developer planning guide

2026-10-18 · **PLANNED — no MCP server, transport or tool implemented**.
Optional POST-V1.5 / V2+ only. [MCP1 plan](../releases/MCP1_PLAN.md) owns phases,
source inventory, authority/security and acceptance; this is the developer entry point.

## Architecture and setup status

Candidate local `fabcam-mcp` over stdio; no executable/install/config command or
dependency is available from this plan. Future A must verify current MCP specification,
client compatibility and SDK/transport choice before instructions are published.
No remote service/new network listener without separate review. Reuse authoritative
Rust/application services; audit portable document loading versus core wire requests
and Svelte command/history ownership before extraction.

## Tool/resource and permission policy

Illustrative read tools and later mutations are in MCP1. Detailed protocol schemas
are unfrozen; future implementation documents versioned structured inputs/results,
units/frames/IDs, capabilities, errors, limits and permission scope. Any resources
must be explicit authorized project/session data, not an arbitrary filesystem browser.
Read-only analyze/recommend does not apply; write/generate/export permissions differ.
Private source packs/raw profiles are not automatically exposed with recommendations.

## Example future workflows — not executable API examples

Read-only: authorize project snapshot → inspect sources/operations → analyze supported
geometry → inspect permitted tools → request derived recommendation → present proposal.
No save/operation change or physical-validation claim.

Editing: propose against exact revision → preview/validate → human approval → Apply
via existing owner → invalidate generation. Offline copies and desktop-owned unsaved
sessions need different conflict/history/save contracts; never edit a live document's
disk file behind its owner. Revalidate after approval if context changed.

Generation: explicit request permission → qualified shared pipeline → structured
diagnostics/current result → human review → separately authorized file export through
preflight. No hardware command, stale/partial export or implicit approval from a client.

## Lifecycle, errors and adversarial testing

Design explicit load/snapshot/session/revision/request identity, resource quotas,
cancellation/timeouts, late results, retries/disconnect and atomic mutation/rollback.
Preserve common errors/refusals and qualification limits; do not invent a second ready
state. Test paths/symlinks/unauthorized reads, private data, hostile project instructions,
schema failures, wrong/stale revisions, concurrent desktop edits, duplicate requests,
Undo/save/reopen, output guards and bounded computation. Protocol conformance and
multiple clients need actual evidence, not successful one-client happy path.

## Related contracts and implementation handoff

[ARCH-MACH](../releases/ARCH_MACH_PLAN.md), existing FS ownership, geometry identity,
history, post and output-validation contracts remain authoritative. Future handoff must
document implemented versus deferred tools/transports/versions, setup, actual schemas,
permissions/errors, test commands/results and known limits. Proposed agent skill links
this guide and MCP1 after approval, not a new source of business rules.

**HARD STOP — roadmap/architecture/security approval required before implementation.**
