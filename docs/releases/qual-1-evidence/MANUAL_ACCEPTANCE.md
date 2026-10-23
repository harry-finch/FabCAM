# QUAL-1 host and file-transfer acceptance — pending

All rows below remain **NOT TESTED** unless a human supplies the actual execution
record. Automated serialization, mocked dialogs and static ACL tests do not close them.
Use disposable QA data and a separate browser profile/origin or OS account. Do not
reset the existing application's data directory. No installer or hardware is required
or authorized by these instructions.

## Host software gates

Use the pinned Rust 1.88.0 toolchain, wasm32 target, Node 24, pnpm 10.34.7,
wasm-pack and matching wasm-bindgen 0.2.120/wasm-opt, cargo-deny 0.20.2, a supported
Chromium executable and a clean checkout of the source stated in the QUAL-1 report.
The host must permit local listeners, FSEvents delivery, npm registry access and
fresh GitHub RustSec advisory retrieval. Preserve the complete log and exit status.

Run `bash docs/releases/qual-1-evidence/host-gates.sh /absolute/path/to/clean/checkout`.
The wrapper only configures executable discovery, builds frontendDist before the
Tauri compile, and invokes the **unchanged** canonical script. It neither excludes
watcher tests nor substitutes the browser smoke. On macOS it exposes the installed
Google Chrome binary under the name the canonical script expects, using a temporary
wrapper; the smoke itself creates an isolated browser profile. It does not change
browser/security settings. A current online advisory audit is mandatory.

## Actual v9 project file

1. In a disposable profile, create a new project with built-in DejaVu `O` geometry
   (50 mm, plain), MDF and a Pocket named `QUAL-1 SYNTHETIC — NO MACHINING`.
   Author rough 12000/600/120 and finish 10000/450/90 (RPM/feed/plunge), final Z -1,
   depth/pass 0.5 mm. These are software-QA inputs, **not cutting recommendations**.
2. Save through the real browser/native dialog to a new `.ivac-project.json` file.
   Inspect the actual file: version 9, text/geometry source, machine/tool IDs,
   material identity, rough/finish roles and all values must match.
3. Open that file through the normal picker in the disposable profile. Confirm
   geometry, actual role values and identities before generating. Qualify the full
   program; verify report values and stale-export refusal after a feed edit.
4. Clear a required cutting value, save/reopen this incomplete editable project,
   then request generation. Saving must work and generation must refuse with a
   precise missing-value error. Do not export a machine program.
5. Cancel both pickers. Try malformed and unsupported-version project copies.
   Original document, geometry, values and dirty state must survive failure.

## Actual preset collection file and storage conflicts

1. Create `QUAL-1 DISPOSABLE QA — NO MACHINING`. Save one preset labelled
   `SYNTHETIC SOFTWARE QA — NOT PHYSICALLY TESTED`. Exercise the required attestation
   checkbox only as a declared synthetic test; it is not a real workshop assertion.
2. Export through the actual dialog/download. Retain and hash the JSON file; inspect
   collection/schema/revision, preset IDs, context, role values and attestation.
3. Import the exact file in a second clean disposable profile/origin. Verify its
   attestation is displayed as the **source's claim**, not a test on the current lab
   machine. Review values before copy; tool/machine/material must not be replaced.
4. Reimport the same file: Cancel preserves storage; Independent copy allocates an
   independent collection identity; explicit Replace requires confirmation. Check
   operation snapshots already copied from the original remain unchanged.
5. Start a replacement review in tab A, change that disposable collection in tab B,
   then finish A. A stale overwrite must be refused. Refresh and inspect the newer
   data. Repeat cancellation without accepting replacement.
6. Import malformed JSON and a file larger than **2 MiB**; storage must remain
   byte-equivalent. On native, record that `stat` succeeds for the selected valid
   file and oversize is rejected before content reading. Exercise denied stat/read
   and failed write with a disposable destination; existing files remain intact.
7. On native, try a path outside dialog-authorized/request-authorized scope using
   the supported test harness. Verify refusal and no new broad filesystem grants.
   Review `fs:allow-stat` and native `ivac-file-export` path authorization separately.
   Static JSON capability inspection alone cannot establish native runtime success.

Record source SHA, OS/browser/runtime versions, screenshots, actual file hashes,
observed errors and pass/fail for each step. Installed macOS/Windows/Linux acceptance
and supervised ShopBot acceptance remain separate human milestones.
