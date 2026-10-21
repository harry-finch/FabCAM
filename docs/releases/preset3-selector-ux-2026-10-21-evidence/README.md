# PRESET-3 selector refinement evidence — 2026-10-21

See [the checkpoint report](../FABCAM_PRESET3_AUDIT_AND_IMPLEMENTATION.md#unified-selector-ux-refinement--2026-10-21).

Source: FabCAM-v1.1 at API commit 5ca56321bd7201eeda6dd5b777966c3e82882e8b plus existing dirty PRESET-1/2/3 and this refinement. source-fingerprints.json pins the scoped frontend files. The incremental patch compares three components to their preserved dirty pre-edit versions; it is not a standalone patch against HEAD.

Browser replay used the real WASM backend, 1280×720 and 680×800 viewports, 11 installed synthetic/reference records, and software-only.ivac-project.json. Numerical fixture values are illustrative and are not cutting guidance. Before narrow/compatible/incompatible/manual captures replay the original three components in a disposable frontend copy; existing attribution and source values were preserved. Normal before captures were taken before edits. Screenshots qualify appearance/interactions only.

Final commands (frontend directory, Node 24): npm test; npm run check; npm run lint; npm run build with RUSTC_WRAPPER empty, CARGO_NET_OFFLINE=true and the existing Rust/WASM toolchain PATH. Logs record 1505 passing tests, one pre-existing locale skip, ten passing WASM package guards, type/check/lint/build success. Existing source-race, Undo/Redo and generation/export guard tests ran; no hardware, export execution, installed-native qualification or physical acceptance is claimed.

Additional manual observations: arrow/Home/End focus movement does not select; Enter selects; incompatible/Draft choices cannot Apply; filtering a selection out keeps its comparison with an explicit notice; Browse restores its pinned mode after visiting Recommended; full-width refusal and missing material block Apply; manual replacement requires acknowledgment; Apply→Undo→Redo restores 1620→1000→1620 feed in one application history step. Both entry points share the same selector. Expanded secondary filters remain available below 700px, footer remains visible while scrolling. Browser viewport override was reset and task-owned servers/tabs closed at handoff.

All evidence remains local/uncommitted with the pending integrated frontend changes. No commit, push, merge, version or release action. Human UX review remains open.
