# Agent Instructions

## Fork governance after M4

This section takes precedence over the upstream Git/session assumptions below.
Read `docs/shopbot-modernization/STATUS.md` and `ROADMAP.md` before new work.
M0–M4 are the completed ShopBot foundation; M4 baseline is
`e7aeebdc43294bbfe3fdead8e6d6b7eb15e1b2f1`. Preserve their names and evidence limits.
Use named future tracks: FS (Feeds & Speeds), UX (milling workflow), CAD
(geometry preparation), WJ (ProtoMAX/waterjet), VINYL (drag knife, lower priority).
Priority: FS foundation → priority milling UX/CAM improvements → WJ → VINYL.
Every milestone ends at human approval; recording a roadmap does not authorize
starting FS0 or implementing any later milestone.

The future FS assistant is milling-specific, independent of UI and toolpath
geometry generation, and independently testable. An adapter supplies CAM
context; accepted or manually edited parameters are explicitly persisted.
Post-processors must not call the engine dynamically. Manual overrides remain
authoritative; aggressiveness and evidence are separate. Waterjet/drag knife
must not depend on FS. See `docs/shopbot-modernization/feeds-speeds/README.md`
and ADR-0002. Use the architecture/documentation skills; use feeds-speeds only
if it exists, and do not create that skill without an explicit request.

Work on `feature/shopbot-mvp`; origin is `harry-finch/ivacam`, upstream is
`aalarchiv/ivacam`. Commits and pushes require explicit user authorization;
M4 publication approval does not authorize publication of subsequent work.
No automatic merge, PR, upstream sync or force-push. A reviewable documentation
handoff may remain uncommitted pending approval.


This project uses **bd** (beads) for issue tracking. Run `bd prime` for full workflow context.

## Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work atomically
bd close <id>         # Complete work
```

> **Local-only repo:** there is no git remote configured, so there is
> no `git push` / `bd dolt push` step. Commits + bd data live locally
> (see Session Completion below).

## Non-Interactive Shell Commands

**ALWAYS use non-interactive flags** with file operations to avoid hanging on confirmation prompts.

Shell commands like `cp`, `mv`, and `rm` may be aliased to include `-i` (interactive) mode on some systems, causing the agent to hang indefinitely waiting for y/n input.

**Use these forms instead:**
```bash
# Force overwrite without prompting
cp -f source dest           # NOT: cp source dest
mv -f source dest           # NOT: mv source dest
rm -f file                  # NOT: rm file

# For recursive operations
rm -rf directory            # NOT: rm -r directory
cp -rf source dest          # NOT: cp -r source dest
```

**Other commands that may prompt:**
- `scp` - use `-o BatchMode=yes` for non-interactive
- `ssh` - use `-o BatchMode=yes` to fail instead of prompting
- `apt-get` - use `-y` flag
- `brew` - use `HOMEBREW_NO_AUTO_UPDATE=1` env var

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:ca08a54f -->
## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

## Session Completion

**This repo is local-only — there is no git remote configured. Skip
the push step until a remote is added** (`git remote -v` is empty, and
bd memory `audit-2026-05-22-fitness` confirms "Issues are saved locally
only"). Do NOT spend cycles trying to push to a remote that doesn't
exist.

**MANDATORY WORKFLOW (local-only variant):**

1. **File issues for remaining work** — Create bd issues for anything
   that needs follow-up
2. **Run quality gates** (if code changed) — Tests, linters, builds
3. **Update issue status** — Close finished work, update in-progress
   items via `bd close <id>` / `bd update <id> --claim`
4. **Commit locally** — Every logical change should land as its own
   commit on `main`. NEVER stop with uncommitted work in the working
   tree.
5. **Verify** — `git status` shows a clean working tree; `bd list
   --status=in_progress` is empty or accurately reflects active work
6. **Hand off** — Provide a brief context summary for the next session

**IF A REMOTE IS LATER ADDED:** restore the push step
(`git pull --rebase && git push`) between steps 4 and 5. Until then,
treat local commits as authoritative.
<!-- END BEADS INTEGRATION (block edited locally — ivac-uqvd / ci5i; bd init may regenerate, re-apply the no-remote workflow if so) -->
