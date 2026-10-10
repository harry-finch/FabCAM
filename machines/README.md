# Lab machine snapshots

[Sorbonne ShopBot PRS](sorbonne-shopbot-prs.ivac-machine.json) is a native
ivaCAM machine snapshot, format version 1. Open the Machine dialog, choose
**Load…**, and select this file. The existing dialog loads it without an
application or schema change. Save the project to retain the machine; use the
existing machine-profile controls to save a reusable workspace profile.

**Read the [machine record](../docs/shopbot-modernization/shopbot/SORBONNE_MACHINE.md)
before changing its unverified fields.** The profile's name deliberately includes
`TO VERIFY ON MACHINE`. `fastMoveZ: 0` is an unconfigured sentinel, not a safe
height: the ShopBot post refuses SBP export until a positive clearance is authored.
`workArea.z: 0` means unknown travel, not a zero-travel machine. X/Y are the
user-reported nominal 1200 × 600 mm, not measured usable travel or enforced
motion limits. Leave the unverified name until the physical record is completed.

Use a CPS-enabled CLI/server/desktop backend. Standalone browser WASM cannot
execute the bundled ShopBot post. No tool library, feeds or spindle settings are
supplied: a machine load preserves the current project's tools and operations.
Review them individually before later qualification. Setting positive clearance
only removes the software export guard; it does not qualify the machine.
