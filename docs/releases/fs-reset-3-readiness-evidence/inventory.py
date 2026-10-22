#!/usr/bin/env python3
"""Source-bound preflight inventory. No files are deleted or application data read.
Run from repo root: python3 docs/releases/fs-reset-final-evidence/inventory.py
Outputs exact line references, per-file dispositions and protected historical index.
Conservative mixed-file dispositions require symbol-level review, never bulk deletion.
"""
import csv, hashlib, json, re, subprocess
from pathlib import Path
SOURCE = 'd6b08fe82011c54d0166a6559b329804c2e56cee'
OUT = Path('docs/releases/fs-reset-3-readiness-evidence')
legacy = re.compile(r'feeds[_ -]?speeds|feedsspeeds|fs[_-]?context|fsIdentity|fsLocalIdentity|fsFullWidth|appliedContextKey|(?-i:\bFs[A-Z]\w*)|source[_-]?pack|sourcepack|applied[_-]?cutting|appliedcutting|experimental[_-]?recipe|experimentalrecipe|sienci|RateOrigins|AppliedRateSlot|captureLabPreset|capture_lab_preset|[\"\x27]fs\d*\.', re.I)
bridge = re.compile(r'\b(?:PipelineRequest|Project)\s*\{|Project::default\(|\brun_pipeline(?:_two_sided|_streaming)?\(')
modules = {'feeds-speeds','feeds-speeds-persistence','feeds-speeds-explanation','feeds-speeds-pack-store','feeds-speeds-sources','feeds-speeds-sources.svelte','lab-presets','lab-presets-store','cutting-identities','preset-identities','preset-library','preset-adaptation','cutting-selector','cutting-selector-sources','cutting-selector-labels','cutting-comparison','cutting-origin-presentation','experimental-recipe','recipe-values','trial-presentation'}
module_re = re.compile(r'(?:/|[\"\x27])(?:'+ '|'.join(re.escape(x) for x in sorted(modules))+r')[\"\x27]')
source_files = subprocess.check_output(['git','ls-tree','-r','--name-only',SOURCE],text=True).splitlines()
legacy_components={'DuplicatePresetDialog','FeedsSpeedsCompatibility','FeedsSpeedsSourcesSettings','LabPackCreate','LabPresetDialog','LabPresetManager','PresetBrowser','PresetOperationPicker','PresetsLibrary','SelectedRecipeContext','TrialComparison','TrialProvenance','CuttingRecommendationDetails','ExperimentalRecipeStatus','FeedsSpeedsSection','SavedCuttingRates'}
# Source references in gate scripts can be indirect (a historical request loaded by path).
gates={'scripts/pre-release.sh','scripts/numsafe3-cli-check.py','scripts/numsafe3-audit-probes.py','scripts/sec1-cli-check.py','scripts/perf1-native.py','scripts/perf1-cancel.py','scripts/perf1-cancel-http.py','scripts/perf1-b2-wasm-check.py','scripts/perf1-paired.py','scripts/test1/build.py','scripts/test1/run.py','scripts/test1/reduce.py','.github/workflows/ci.yml','.github/workflows/desktop-candidates.yml','.github/workflows/release-desktop.yml','.github/workflows/promote-desktop-draft.yml','xtask/src/main.rs','frontend/scripts/wasm-package-state.mjs','frontend/scripts/wasm-package-state.test.mjs'}
def disposition(f, line, n, test_at):
    if f.startswith('docs/') or f.startswith(('.agents/','.beads/')) or f in {'AGENTS.md','README.md','README_de.md'}:
        return 'KEEP','HISTORY','Preserve historical evidence/governance; update current links only.'
    if f=='LICENSE':
        return 'KEEP','LICENSE','Preserve license; lexical false positive is not legacy code.'
    if '/benches/' in f or (f.startswith('crates/ivac-core/examples/')):
        return 'TEST FIXTURE','TESTS','Port explicit request construction; retain geometry/performance assertions and historical evidence.'
    if f in gates or f=='scripts/ui-smoke.sh':
        return 'REPLACE FIRST','GATES','Keep every gate/assertion; author v9 request inputs for public transports, update dependency fingerprints and regenerate output proofs.'
    if '/tests/' in f or '.test.' in f or '/fixtures/' in f or f.endswith('.fixture.ts') or (test_at and n>=test_at):
        return 'TEST FIXTURE','TESTS','Review property before removal: port generic safety/geometry fixtures to explicit v9; delete only recommendation-specific expectations after replacement ledger.'
    if f.startswith(('data/feeds-speeds/','frontend/public/feeds-speeds/')):
        return 'SAFE TO REMOVE','ASSETS','Remove shipped source data only after consumers/tests migrate; preserve history, licensing for any retained copies and all user-owned packs.'
    if f.startswith('crates/ivac-feeds-speeds/'):
        if f.endswith(('model.rs','validation.rs','source_pack.rs','dataset.rs')):
            return 'REPLACE FIRST','DOMAIN','Extract retained Identity/material vocabulary and prove generic limits/validation coverage before deleting recommendation-only remainder.'
        return 'SAFE TO REMOVE','ENGINE','Recommendation calculation/ranking/adaptation exclusively; remove after callers/schema/tests are detached.'
    if f.startswith('crates/ivac-core/src/feeds_speeds'):
        return 'REPLACE FIRST','CORE-BRIDGE','Remove legacy recommendation/trial only after saved_rates/effective_machine/validation internal callers and generic tests are replaced.'
    if f=='crates/ivac-core/src/cutting_parameters.rs':
        return 'REPLACE FIRST','CORE-BRIDGE','KEEP neutral roles/limits; replace only old-contract delegates after internal fixture/CLI migration.'
    if f.startswith('crates/ivac-core/src/pipeline'):
        return 'KEEP','CAM','Retain driver/numeric/provenance/fixture/work-origin guards; remove only explicitly legacy AppliedCutting branches after comparison.'
    if f.startswith(('crates/ivac-core/src/project','crates/ivac-core/src/presets')):
        return 'REPLACE FIRST','DOMAIN','Keep generic project/preset contracts; move material Identity from FS crate, remove obsolete schema/fields only after callers/tests.'
    if f=='crates/ivac-cli/src/main.rs':
        return 'HUMAN DECISION','CLI','Public documented generate command: modernize explicit input or explicitly retire; preserve preflight/refusal/staging/partial-publication contracts.'
    if f.startswith('crates/ivac-tauri/src/source_packs'):
        return 'SAFE TO REMOVE','SOURCE-STORAGE','Remove obsolete commands/registration only; never erase existing application-data files or reuse its namespace destructively.'
    if f in {'crates/ivac-server/src/main.rs','crates/ivac-tauri/src/commands.rs','crates/ivac-tauri/src/lib.rs','crates/ivac-wasm/src/lib.rs'} or f.startswith('frontend/src/lib/api/'):
        return 'REPLACE FIRST','TRANSPORT','Keep v9/preset/generation/authorization; remove only recommendation/source exports, capabilities, RPC and generated FS models coherently.'
    if f.startswith('schema/') or f in {'Cargo.toml','Cargo.lock','crates/ivac-core/Cargo.toml','crates/ivac-core/src/lib.rs'}:
        return 'REPLACE FIRST','BUILD-SCHEMA','Detach migrated consumers first, then remove obsolete dependency/schema registrations and regenerate; preserve current types.'
    stem=Path(f).stem
    if stem in legacy_components:
        return 'SAFE TO REMOVE','LEGACY-UI','Disconnected recommendation/trial UI; verify no imports remain before deleting; retain shared generic components.'
    if f.endswith('/feeds-speeds.ts') or f.endswith('/cutting-identities.ts'):
        return 'REPLACE FIRST','ACTIVE-HELPERS','Active report/palette/edit/asset callers still exist; neutral v9 rates/identity helpers must replace retained behavior, including report defect.'
    if stem in modules or stem.replace('.svelte','') in modules:
        return 'REPLACE FIRST','LEGACY-STATE','Separate generic persistence/identity/settings and active imports from obsolete source/ranking/trial state before removal.'
    if '/i18n/' in f:
        return 'REPLACE FIRST','I18N','Delete only keys with zero retained consumers. fs.material/fs35 labels are reused by active v9/generic UI; keep or rename with callers; regenerate keys.'
    if f.startswith('frontend/'):
        return 'REPLACE FIRST','ACTIVE-CONSUMER','Keep current generic workflow; remove legacy fields/imports only after explicit v9 replacement and preservation tests.'
    return 'HUMAN DECISION','OTHER','Inspect exact reference before deletion; no path-only removal authorization.'
rows=[]; file_rows=[]; history=[]
for f in source_files:
    if Path(f).suffix.lower() in {'.svg','.png','.jpg','.jpeg','.webp','.pdf','.woff','.woff2','.ttf','.ico','.icns','.mp4','.zip'}:continue
    raw=subprocess.check_output(['git','show',SOURCE+':'+f])
    try:text=raw.decode('utf8')
    except UnicodeError:continue
    lines=text.splitlines(); test_at=next((i for i,l in enumerate(lines,1) if '#[cfg(test)]' in l),0)
    found=[(i,l) for i,l in enumerate(lines,1) if legacy.search(l) or module_re.search(l) or (not f.startswith('docs/') and bridge.search(l))]
    whole=f.startswith(('crates/ivac-feeds-speeds/','data/feeds-speeds/','frontend/public/feeds-speeds/')) or f in gates or (f.startswith('tests/fixtures/') and f.endswith('.json'))
    if whole and not found:found=[(1,lines[0] if lines else '')]
    if not found:continue
    if f.startswith(('docs/','.agents/','.beads/')):
        history.append([f,len(found),'KEEP','Historical/governance only; not executable cleanup scope',hashlib.sha256(raw).hexdigest()]);continue
    classes=set(); groups=set()
    for n,line in found:
        cl,group,action=disposition(f,line,n,test_at);classes.add(cl);groups.add(group)
        rows.append([f,n,cl,group,action,line])
    file_rows.append([f,len(found),'; '.join(sorted(classes)),'; '.join(sorted(groups)),hashlib.sha256(raw).hexdigest()])
for name,header,data in [('legacy-references.csv',['path','line','classification','group','action','source_line'],rows),('legacy-files.csv',['path','reference_lines','classifications','groups','sha256'],file_rows),('historical-references.csv',['path','reference_lines','classification','action','sha256'],history)]:
    with (OUT/name).open('w',newline='') as dest:
        w=csv.writer(dest,lineterminator="\n");w.writerow(header);w.writerows(data)
meta={'source_sha':subprocess.check_output(['git','rev-parse',SOURCE],text=True).strip(),'tracked_files_scanned':len(source_files),'operational_files':len(file_rows),'exact_operational_reference_lines':len(rows),'protected_historical_files':len(history),'patterns':[legacy.pattern,module_re.pattern,bridge.pattern],'scope':'All tracked UTF-8 at source SHA; explicit legacy module closure plus indirect gate script list. Per-line lexical references and whole-module candidates. Not a proof of arbitrary dynamic reachability or a bulk-delete manifest. Untracked user data excluded/protected.','counts':{c:sum(r[2]==c for r in rows) for c in ['SAFE TO REMOVE','REPLACE FIRST','KEEP','TEST FIXTURE','HUMAN DECISION']}}
(OUT/'inventory-receipt.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps(meta,indent=2))
