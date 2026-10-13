# UX2.2 — Shell and welcome design checkpoint

2026-10-13 · ivac-7737. User selected UX2.2 as the next development milestone.
Source audit and design proposal complete; production UI implementation has not
started. Design/lifecycle decision remains ivac-dwdd. Earlier CAM4 and FS human
reviews remain independent and open. FS4 is deferred and nonblocking.

## Recommendation

Approved bounded continuation (ivac-5eeh): user requires the active machine name
and optionally the operation tool in the menu bar. Implemented desktop machine
name in its existing configuration tab and a separate selected-operation tool
shortcut. The tool shortcut reveals the operation inspector; the shop library
remains independent because an embedded project tool can differ from an inventory
entry with the same ID. Missing tool references and unnamed machines are explicit.
No physical mounted-tool state is inferred. Names use the current project snapshot,
react to selection and preserve full accessible labels/tooltips when truncated.
The tool shown is the primary `toolId`; secondary finish/spot tools stay in the
operation inspector. This bounded change does not approve New/Close semantics.

Validation: frontend check/lint/build; 19 targeted i18n/sidebar tests passed,
1 optional skip. Browser review uses CAM4 software fixture and includes no
selection, long names, machine navigation and reduced 1024×768 layout.
[Desktop capture](UX2_2-machine-tool.jpg). No native bundle or machining claim.

Choose **Atelier**: retain compact Project/Machine/Tools navigation, put Settings
and Help at the trailing edge, group file commands with project identity, and keep
the existing GenerateBar beside the 2D/3D workspace controls. Move Report and
infrequent file actions into an accessible project menu. Welcome occupies the
workspace only when no document is active; it offers New, Open, Import, local
recents and a secondary example. No forced onboarding.

Alternative **Document**: one document-centric command row with a Project menu,
filename, save/edit actions, Generate/Export; Machine and Tools move into a compact
setup strip. It saves a navigation row, but relocates familiar setup entry points
and requires more overflow handling. Atelier is the smaller upstream change.

The accompanying interactive proposal uses illustrative filenames and locally
simulated actions. It is not a screenshot or evidence of functioning application
code. Both alternatives distinguish welcome, new empty, geometry/no operations,
and operations/stale paths. Product copy will use existing English/German i18n;
French review labels do not imply an added application locale.

## Verified command inventory

Paths below are relative to `frontend/src/`, except the Tauri entry.

| Command | Current entry and authority | Availability / shortcut | Disposition |
| --- | --- | --- | --- |
| Open drawing/project | App toolbar, mobile overflow; `lib/services/file_ops.ts::openAny` | Not loading on toolbar; Ctrl/Cmd+O outside text inputs | KEEP router, EXTEND welcome/menu presentation |
| Recent | `RecentMenu.svelte`, `workspace-session.svelte.ts::openRecentProject` | Disabled for empty list; guarded replace, extension routing | KEEP storage and guard; EXTEND welcome list |
| Import/add drawing | `LayerList.svelte`, `FileUpload.svelte`, `file_ops.ts::addDrawing` | SVG/DXF picker; additive import | KEEP additive meaning; reuse for welcome/empty workspace |
| Save | Toolbar/mobile, `file_ops.ts::saveProject` | Toolbar requires transformed import; Ctrl/Cmd+S uses handler directly | KEEP transport; reconcile empty-project presentation in approved lifecycle scope |
| Save As | No separate handler | Save already opens a native picker on every call | DEFER distinct overwrite/Save As behavior; do not advertise two identical commands |
| New / Close project | No user-facing shared actions found; `project.clearProject` is a low-level reset | No New/Close shortcuts in `app-menu.ts` | EXTEND only under lifecycle contract below |
| Undo/Redo | Toolbar and project history | Ctrl/Cmd+Z, Shift+Ctrl/Cmd+Z or Ctrl/Cmd+Y; typing guard | KEEP history and labels |
| Generate | `GenerateBar.svelte`, narrow `PhoneWarnings` | Existing geometry/text and pipeline conditions | KEEP mounted execution owner and generate bus |
| Export program | GenerateBar, mobile save menu; `file_ops.ts::exportGeneratedGcode` | Context freshness and idle pipeline; warning policy differs by entry | KEEP authoritative guards; no new independent readiness claim |
| Preview | App 2D/3D, scene/simulation controls | Existing generated data and view state | KEEP; make view selection discoverable |
| Text | Layer controls / dialog | Bare T outside typing | KEEP; creating text must enter workspace |
| Settings/Help/About/Report | App navigation/dialog flags | Help shortcuts ? / F1 outside typing | KEEP handlers; EXTEND compact placement |
| Exit application | `workspace-session.svelte.ts`, App exit | Native close interception / narrow exit | KEEP; not synonymous with Close project |

`crates/ivac-tauri/src/lib.rs` explicitly omits the native window menu. Do not
invent a second native command system. Global shortcut resolution lives in
`lib/state/app-menu.ts`; App owns the effects. Hidden `FileUpload` inputs, URL
sample startup and file-association listener must remain mounted on welcome.

## Lifecycle ownership and findings

`project.svelte.ts` always contains a project object. Neither empty geometry nor
`activeProjectPath === null` denotes no document: a browser-loaded project has no
native path, and a user may delete all geometry. New/Close need a runtime-only
document-active flag owned at the existing project/session boundary, explicitly
updated on successful entry/reset; it must not be serialized or become a second
project store. Ordinary edits/deletes, Save, stale paths and failed generation
must never reset it to welcome.

`workspace-session.svelte.ts` loads and prunes workspace recents, then offers a
desktop reopen banner; it does not silently reopen. Preserve that policy, render
the offer on welcome and dismiss it on successful document entry. Browser native
paths are not reusable file permissions: offer a file picker instead. Recents
already have clear-all and missing-file pruning; per-entry removal is deferred.

Three verified compatibility limits require explicit handling in the implementation
scope, rather than being hidden by a polished shell:

1. `unsaved.ts::computeUnsavedWork` returns false when content is empty, even if
   dirty. A new project's edited stock/work origin/fixtures can therefore escape
   the discard guard. New/Close must protect those edits through the existing
   unsaved authority; include this bounded repair in the proposed scope.
2. `saveProject` marks the project saved but does not update active path/recents.
   The active path can still identify the imported drawing. Do not label it as the
   saved project filename. Proposed bounded fix: maintain session document identity
   after successful load/save without repointing the source watcher to a JSON
   project or changing save transport/format. Failed/cancelled saves retain identity.
3. `clearProject` shallow-copies stock (retains dimensions), preserves machine/tools,
   and does not explicitly clear every generated field. It is a load helper, not
   a ready-made New action. Proposed New retains machine/tools/stock dimensions,
   resets per-job stock placement and work origin, fixtures, geometry, operations,
   material/FS context, all generated/simulation state, selection and history.
   Close uses the same job reset, then shows welcome. Verify asynchronous generation
   cannot repopulate a replaced/closed document. No filesystem writes on New/Close.

Imports parse/validate before clearing, but restore/text-conversion occurs after
the reset in several paths. Preflight rejection preserves the current project;
post-reset failure is not proven transactional. New shell transitions must not
claim failure recovery beyond evidence. Qualify these paths; if they fail, handle
the bounded repair explicitly before shipping replacement navigation.

Export policy is also not fully unified: GenerateBar checks aggregate critical
warnings; shared export checks freshness and fixture collision policy. Retain the
GenerateBar export action as the canonical desktop action. Do not introduce a
green blanket “ready for machine” indicator or duplicate warning logic in a header.

## Proposed transition contract for approval

| Trigger | Result on success | Cancel/failure |
| --- | --- | --- |
| Startup | Welcome; existing optional reopen offer | Remain on welcome |
| New | Guard unsaved work; reset job as above; active empty workspace | Prior document and view retained |
| Open/recent/example | Existing guarded replacement; active workspace after successful commit | Picker/guard cancellation retains prior context; load failure needs qualified recovery |
| Import | Existing additive handler; active workspace after successful commit | Keep previous content and presentation |
| Close project | Guard; cancel/invalidate pending job results; reset; welcome | Prior document retained; save failure/cancel aborts Close |
| Delete last geometry | Stay active; offer import/text next step | Never infer welcome from emptiness |
| Edit then Save | Saved-document status; old output remains stale | Export remains blocked until successful regeneration |

New/Close remain menu actions with no newly assigned shortcut initially. Existing
shortcuts and mobile fallback stay functional. Save action must also work for an
active empty configured project; avoid enabling it merely because an inactive
default project singleton exists.

## Qualification plan

Extend `unsaved.test.ts`, `workspace.test.ts`, `app-menu.test.ts`, history and
`generation-context.test.ts` where their existing authorities change. Add meaningful
service-level transition tests with injected IO for guard outcomes and success/
failure, rather than tests that only repeat CSS or a mock's state calculation.

Required sequences: untouched/modified empty project; fresh never-saved import;
New/Close with Save, Discard and Cancel; failed/cancelled Save; Open picker cancel;
invalid drawing/project; late import/generation result after switch; text-only
project; delete last geometry; reopen saved empty project; stale output then Save;
critical export policy; native missing recent and browser picker fallback.

Run frontend tests/check/lint/build after implementation. Review actual application
at 1280×800, 1440×900 and 1024×768 with long filenames, German copy, keyboard focus,
blocking warnings, welcome and all empty/stale states. Preserve narrow/mobile
controls. Native picker/close evidence requires a real Tauri session; browser
mockups cannot qualify it. No CAM/post/FS mathematics or physical evidence changes.

## Decision requested

Approve Atelier (recommended) or Document, plus the explicit New/Close and bounded
unsaved/document-identity/reset scope above, then implement and return to human
review with automated and actual UI evidence. This is the first design checkpoint
specified by UX2_2_PLAN, not completion of UX2.2. No commit/push authorized.

## Approved implementation — 2026-10-13

The user explicitly approved Atelier, including the active machine in the top bar.
Design gate ivac-dwdd is closed. The bounded lifecycle repairs described above
are implemented; see [qualification](UX2_2_QUALIFICATION.md) for current evidence
and human/native review ivac-ein1. Earlier proposal/approval wording is historical.
