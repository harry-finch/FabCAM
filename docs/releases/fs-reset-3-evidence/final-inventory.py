from pathlib import Path
import ast,subprocess,re,json,csv,hashlib
source=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip();out=Path('docs/releases/fs-reset-3-evidence')
syntax=ast.parse(Path('docs/releases/fs-reset-3-evidence/start-inventory.py').read_text());pattern=next(ast.literal_eval(n.value.args[0]) for n in syntax.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='legacy' for t in n.targets));pattern=pattern.replace(chr(92)*2,chr(92));legacy=re.compile(pattern,re.I)
files=subprocess.check_output(['git','ls-files'],text=True).splitlines();rows=[];history=[];hashes={}
for file in files:
 p=Path(file)
 if not p.is_file():continue
 if file.startswith(('docs/','.agents/','.beads/')):
  if p.suffix in ['.md','.json','.csv','.tsv','.py','.txt']:
   try:
    count=sum(bool(legacy.search(l)) for l in p.read_text().splitlines())
    if count:
     current=file in {'docs/ARCHITECTURE.md','docs/BUILDING.md','docs/CLI_GENERATE.md','docs/README.md','docs/shopbot-modernization/STATUS.md','docs/shopbot-modernization/ROADMAP.md','docs/releases/RELEASE_ROADMAP.md','docs/shopbot-modernization/adr/ADR-0030-fabcam-fabspeed-separation.md'}
     history.append([file,count,'KEEP' if current else 'HISTORICAL','Current navigation/contract with preserved historical checkpoints' if current else 'Preserved prior decisions, software evidence or governance; current authority ADR-0030/FS_RESET_3_RESULTS'])
   except (UnicodeError,OSError):pass
  continue
 try:text=p.read_text()
 except UnicodeError:continue
 hashes[file]=hashlib.sha256(p.read_bytes()).hexdigest()
 for n,line in enumerate(text.splitlines(),1):
  if not legacy.search(line):continue
  kind='KEEP';why='Neutral current CAM/library/presentation vocabulary; no recommendation dependency'
  if 'test' in file or '/benches/' in file or file.endswith('explicit-project.fixture.ts') or (file.endswith('presets.rs') and 'assert!' in line):kind='TEST-ONLY';why='Explicit legacy refusal/no-runtime-dependency regression or neutral fixture'
  elif file=='assets/ivaCAM_GH_social.svg':why='Binary PNG payload lexical coincidence inside SVG; no executable legacy dependency'
  elif file=='AGENTS.md' or file.startswith('README'):kind='HISTORICAL';why='Preserved upstream/governance terminology; ADR-0030 supersedes old recommendation model'
  elif file.endswith('physical-identities.ts'):why='Retain existing generic library physical ID only; old reference bindings are inert metadata'
  elif file.endswith('project-file-ops.ts'):why='V9 snapshot omits retired metadata; no old project loader'
  elif file.endswith('project-v9.ts') or file.endswith('ivac-wasm/src/lib.rs'):why='Explicit rejection of retired request keys before conversion'
  elif '/i18n/' in file:why='Retained generic material/tool labels using historic key prefixes; no recommendation computation'
  rows.append([file,n,kind,why,line[:1000]])
for name,header,data in [('final-consumers.csv',['path','line','disposition','reason','source_line'],rows),('final-historical.csv',['path','lines','disposition','reason'],history)]:
 with (out/name).open('w',newline='') as f:w=csv.writer(f,lineterminator='\n');w.writerow(header);w.writerows(data)
meta={'source':source,'tracked_files':len(files),'lexical_reference_lines':len(rows),'files_with_references':len(set(r[0] for r in rows)),'documentation_files_with_references':len(history),'historical_files':sum(r[2]=='HISTORICAL' for r in history),'pattern':pattern,'scope':'Tracked UTF-8 only. Legacy lexical scan plus Cargo metadata/transport registrations and negative tests. User/untracked data never cleanup targets.'};(out/'final-inventory.json').write_text(json.dumps(meta,indent=2)+'\n');(out/'final-source-hashes.json').write_text(json.dumps(hashes,indent=2)+'\n');print(meta)
