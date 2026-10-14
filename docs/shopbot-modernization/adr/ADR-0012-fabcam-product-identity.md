# ADR-0012: FabCAM product identity with stable compatibility identifiers

Status: Accepted identity policy; local implementation awaiting human review
Date: 2026-10-14

## Context

User established FabCAM as the public product identity. The fork is
`harry-finch/FabCAM`, derived from `aalarchiv/ivacam`. A1 branding is integrated;
original authorship, GPL notices and compatibility contracts must remain intact.

## Decision

Use **FabCAM** for UI/browser/window titles, About, reports, current documentation
and display-only metadata. Credit ivaCAM by Soenke J. Peters and preserve original
upstream/dependency acknowledgements. Keep LICENSE unchanged and GPL-3.0-or-later,
as declared by both workspaces.

Preserve crates/modules/binaries, `org.opcenter.ivacam`, storage paths and keys,
IPC/schema IDs, extensions, migrations, CPS IDs/vendors, machine-code headers,
data provenance and historical fixtures. Version stays 0.5.0. About/title reuse
Vite defines resolving the unversioned frontend to Tauri, currently matching Cargo.

Global `productName=ivaCAM` remains: the installed Tauri utils 2.9 documentation
derives the default MSI upgrade code from `<productName>.exe.app.x64`.
[Tauri reference](https://v2.tauri.app/reference/config/#wixconfig).
Installer/artifact names also depend on it. No installation/executable migration
is authorized. Update the window title and macOS CFBundleName/CFBundleDisplayName
through `bundle.macOS.bundleName` and a display-only Info.plist overlay. Retain
bundle filename, executable, identifier, signing, permissions and security settings.

## Alternatives considered

Global replacement would mix public identity with persisted/install contracts;
those require separate migration qualification. UI-only changes would leave safe
display metadata stale, so update display fields with an explicit packaging boundary.

## Consequences

`ivaCAM.app`, Windows installer names, internal IDs and machine-code provenance
tags can remain visible as legacy contracts. Original ivaCAM credits are not
rewritten as FabCAM authorship; no upstream endorsement is implied. Git history,
remotes, legal notices and public/private source-pack boundaries remain intact.

## Validation

Check frontend/lifecycle tests, project round trips, protected-file hashes, version
agreement, storage resolver/keys, native bundle metadata and existing recents/settings.
Capture actual About attribution/license and runtime identity. Windows/Linux native
installer/runtime naming is untested here. Installed-upgrade testing must precede
any future packaging-name migration; functional/physical qualification gates stay intact.
