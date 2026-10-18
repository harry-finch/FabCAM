# Holding, preview and project contracts — audits 11–12

2026-10-18 · ivac-jsdh9 · Software-only checkpoint, human review ivac-80ewz pending.
Incoming source is commit `2a3fe5632b4d77ab03d899bcb0a07db54a8b9f7b` plus the
uncommitted pipeline55 motion/driver audit. Prior evidence is retained unchanged.
This lot moves caches to pipeline56; public version remains 0.9.0-beta.2.

## Findings and disposition

| Report item | Verified result |
| --- | --- |
| Tab width excludes cutter radius | Confirmed semantic mismatch. User approved versioned physical width and explicit conversion; [ADR-0022](adr/ADR-0022-versioned-tab-width.md). Existing files remain legacy. New UI operations use material width. |
| Rectangle lifts whole arc | Confirmed. New mode splits analytically at line/disc or circle/disc crossings; retains full-depth portions outside tabs. Legacy emitter preserved intentionally. |
| Drag knife endpoint/arc compensation | Confirmed, explicitly deferred by user to separate VINYL checkpoint `ivac-wgovg`. Initial orientation, entry, exit, arcs and corner handling must be qualified together. Alleged 90° trigonometric error is not an error: the identities produce the correct direction vector. ShopBot refusal unchanged. |
| Preview compact words / UTF-8 / malformed axis | Fixed ASCII word scanner, accepts G0X10, ignores invalid numeric words without teleporting to zero, no UTF-8 byte slicing panic. Drill depth and retract plane stay modal; G80 cancels. |
| Template values interpreted again | Validate placeholders in the original template and substitute once. Names containing token-shaped text or Unicode remain literal. Unknown template tokens still produce warning comments. |
| Signed zero padding | Pad after the sign. |
| Missing drill angle | Missing Drill angles resolve to 118° in the API adapter and Rust deserializer; explicit 60°/135° remain untouched. UI placeholder agrees. ToolConfig has no tool kind; its cone formula is correct. Dowel angle was already corrected in an earlier lot, and the UI already exposed drill angle. |
| Pocket strategy invented defaults | Required strategy fields now required; unknown/mistyped fields rejected. Positive finite dimensions and valid angular ranges checked. Legacy Cascade/Spiral/Zigzag strings retained. Halfpipe geometry algorithm unchanged. |
| Positive/tiny step silently inherited | Positive steps were already preserved, contrary to report wording. Exact zero is an existing documented legacy inheritance sentinel and remains so. Tiny nonzero values now remain explicit and are rejected below the scheduler threshold, along with nonfinite/positive values, rather than inheriting or producing a full-depth fallback. |
| Nested schema collision | Same equality assertion as top-level definitions; differing nested shapes fail generation. |

## Geometry and numerical contract

For each tab, the protected local material disc has radius W/2 around the
projected anchor. A cylindrical cutter of radius R stays at or above the tab
height whenever its centre can intersect that disc: distance <= W/2+R.
Overrides participate in the same envelope; overlapping tabs use maximum height.
This is conservative local retention, not exact curved arc-length sizing.

The centre exclusion radius includes 0.0101 mm extra retention, covering the
current generic IR 0.01 mm chord budget and 0.0001 mm coordinate rounding.
This budget is qualified for the tested millimetre outputs; arbitrary custom
post precision or inch rounding is not qualified by this lot. This increases
remaining material, never the permitted XY geometry deviation.
Rectangle sub-arcs remain analytic. Variable-height curved ramps use chords
with sag <=0.005 mm. Ramp height is a Lipschitz upper envelope over each entire
interval: endpoints are raised by L*interval_length/2, at most 0.005 mm; total
interpolation excess is bounded by 0.010 mm before XY/rounding effects. It does
not permit cutting below the requested plateau. Parametric split deduplication
uses 1e-12 (dimensionless), separately from machining budgets.

Direct entry accounts for the physical footprint and ramp reach. Lead-out uses
the same versioned emitter. Tests cover straight cuts both ways, circle arcs
CW/CCW, overlapping tabs, closure seams, width/height overrides, entry protection
and final floor outside footprints. A ShopBot test exposed tiny endpoint Z
differences on nominally flat portions; use a common flat height and XYZ chords
for actual ramps. The planar-arc post guard was not weakened.

The preview lexer is not a complete arbitrary G-code interpreter: custom
multi-letter axes, general G98/G99 cycle return semantics and controller dialects
are not qualified here. No general Engrave/V-bit/knife capability is enabled.

## Validation and UX

Raw logs and source fingerprints: [checks.json](holding-preview-audit-2026-10-18-evidence/checks.json).
Four counterexamples failed before their fixes (compact/Unicode preview,
malformed axes, signed formatting and Unicode template values). New regressions
also exercise modal cycles, literal template values, width persistence and
cutter-sweep envelopes. Existing legacy snapshots are retained.

Browser review on local Vite source: new Profile uses material width; reopening
a test project with the field omitted displays legacy mode; conversion changes
the explanation; Undo restores legacy mode. Command tests cover Redo and JSON
persistence; API tests cover both wire values. The existing properties panel
collapses its section after an operation edit; reopening shows the current mode.
No generated-program or native-app browser claim: this review concerns controls.
Screenshots are retained alongside the logs. Human UX acceptance remains open.

| Final check | Result |
| --- | --- |
| Core with CPS, offline, serial | 1,188 PASS (14 additional tests vs pipeline55) |
| Nine integration suites | 92 reported PASS; 91 executed, optional FANUC absent |
| Frontend Vitest | 1,328 PASS; 1 pre-existing skipped |
| Frontend WASM package-state Node suite | PASS (see frontend log) |
| Svelte/TypeScript / ESLint/Prettier | PASS, 0 typecheck errors/warnings |
| Clippy warnings denied / Rust formatting / diff whitespace | PASS |
| Browser review | New mode, legacy reopen, conversion, Undo verified |

## Release / handoff

No physical holding-strength, machine, new installer or clean-host qualification.
Desktop candidates from run39053047404 remain pipeline54 and do not include these
changes. Do not reopen converted jobs in old builds that ignore width_mode.
Mac clean-install, Windows 11 and Ubuntu gates remain unchanged. Existing
cancellation-latency follow-up ivac-42ylx and cache isolation ivac-rt80e stay open.

**HARD STOP for human review; no commit, push, merge, tag, release or publication.**
