// Read-only compatibility probes of the committed ALPHA-3.1 readers.
// Node 24; ALPHA31_CLI identifies the locally qualified executable.
// Generated probe inputs stay in a fresh temporary directory; no app data is used.
import { readFileSync, writeFileSync, existsSync, mkdtempSync } from 'node:fs';
import { resolve, dirname, extname } from 'node:path';
import { tmpdir } from 'node:os';
import { pathToFileURL, fileURLToPath } from 'node:url';
import { createRequire, registerHooks } from 'node:module';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
const root=process.cwd(), baseline=resolve(root,'../FabCAM-alpha-3-1');
const receipt=JSON.parse(readFileSync(resolve(root,'docs/releases/evidence/alpha31/runtime-comparison.json')));
const hash=b=>createHash('sha256').update(b).digest('hex');
const inventory=JSON.parse(readFileSync(resolve(root,'docs/releases/evidence/alpha31/software-files.json')));
for(const f of inventory.files)assert.equal(hash(readFileSync(resolve(root,f.path))),f.sha256,f.path);
const binary=process.env.ALPHA31_CLI; assert.ok(binary);
assert.equal(hash(readFileSync(binary)),receipt.nativeExecutableSha256);
const pkg=resolve(baseline,'crates/ivac-wasm/pkg');
assert.equal(hash(readFileSync(resolve(pkg,'ivac_wasm_bg.wasm'))),receipt.wasmSha256);
const wasm=await import(pathToFileURL(resolve(pkg,'ivac_wasm.js')));
await wasm.default({module_or_path:readFileSync(resolve(pkg,'ivac_wasm_bg.wasm'))});
const ts=createRequire(resolve(baseline,'frontend/package.json'))('typescript');
registerHooks({
 resolve(specifier,context,next){
  if(specifier.startsWith('.')&&context.parentURL?.startsWith('file:')){
   const p=fileURLToPath(new URL(specifier,context.parentURL));
   if(!extname(p)&&existsSync(p+'.ts'))return {url:pathToFileURL(p+'.ts').href,shortCircuit:true};
  }
  return next(specifier,context);
 },
 load(url,context,next){
  if(url.startsWith('file:')&&url.endsWith('.ts'))return {format:'module',shortCircuit:true,source:ts.transpileModule(readFileSync(fileURLToPath(url),'utf8'),{compilerOptions:{module:ts.ModuleKind.ESNext,target:ts.ScriptTarget.ES2022}}).outputText};
  return next(url,context);
 }
});
const {validateProjectV10}=await import(pathToFileURL(resolve(root,'frontend/src/lib/state/project-v10.ts')));
const {captureTemplate,validTemplate}=await import(pathToFileURL(resolve(root,'frontend/src/lib/state/operation-templates.ts')));
const {validateLoadedSnapshot}=await import(pathToFileURL(resolve(root,'frontend/src/lib/state/cutting-parameters.ts')));
const input=JSON.parse(readFileSync(resolve(root,'tests/fixtures/alpha31/convex-pocket.request.json')));
const portable=JSON.parse(readFileSync(resolve(root,'docs/releases/evidence/alpha31/browser-saved-pocket.ivac-project.json')));
const scratch=mkdtempSync(resolve(tmpdir(),'alpha32-reader-'));
let number=0;const rows=[];
function cli(v){const p=resolve(scratch,`${number++}.json`);writeFileSync(p,JSON.stringify(v));return spawnSync(binary,['generate','--request',p],{encoding:'utf8',maxBuffer:64*1024*1024});}
function error(fn){try{fn();return null;}catch(e){return typeof e==='string'?e:e.message||JSON.stringify(e);}}
const current=cli(input);assert.equal(current.status,0,current.stderr);const native=JSON.parse(current.stdout);
assert.equal(wasm.generate(input).gcode,native.gcode);
validateProjectV10(portable);
rows.push({case:'unchanged-v10-v1',native:true,wasm:true,portable:true,program_sha256:hash(native.gcode)});
const template=captureTemplate(portable.operations[0],portable.tools,'Synthetic compatibility','alpha32-reader');
assert.ok(validTemplate(template));
const context={machine_id:'synthetic-machine',machine_label:'Synthetic machine',material_id:'synthetic-material',material_label:'Synthetic material',signature:'{"operation":{"type":"pocket"}}'};
const parameters={stages:structuredClone(input.project.operations[0].params.stages),pocket_overlap:0.5,planar_stepover_mm:null};
const snapshot={collection_id:'qa',collection_name:'QA',preset_id:'probe',preset_name:'Synthetic',context,parameters};
validateLoadedSnapshot(snapshot);
const collection={format:'fabcam-preset-collection',version:2,id:'qa',name:'QA',presets:[{id:'probe',name:'Synthetic',context,parameters,tested:true,test_date:null,notes:null}]};
const parsePreset=v=>wasm.presetCommand(JSON.stringify({action:'validate_collection',json:JSON.stringify(v)}));
parsePreset(collection);
// Deliberately hypothetical unsupported payloads: no reader/planner implementation.
for(const variant of ['scope-on-v1','new-discriminator-only','new-discriminator','new-discriminator-disabled']){
 const req=structuredClone(input),file=structuredClone(portable),t=structuredClone(template),s=structuredClone(snapshot),c=structuredClone(collection);
 const mutate=stages=>{if(variant!=='scope-on-v1')stages.contract='pocket_finishing_v2';if(variant!=='new-discriminator-only')stages.finishing.scope='walls_only';};
 mutate(req.project.operations[0].params.stages);mutate(file.operations[0].stages);mutate(t.stages);mutate(s.parameters.stages);mutate(c.presets[0].parameters.stages);
 if(variant.endsWith('disabled')){req.project.operations[0].enabled=false;file.operations[0].enabled=false;}
 const n=cli(req);assert.notEqual(n.status,0);assert.equal(n.stdout,'');
 const before=JSON.stringify(file);const f=error(()=>validateProjectV10(file));assert.ok(f);assert.equal(JSON.stringify(file),before);
 const out={case:variant,native:n.stderr.trim().split('\n').at(-1),portable:f,portable_input_unchanged:true,template_rejected:!validTemplate(t),snapshot:error(()=>validateLoadedSnapshot(s)),collection:error(()=>parsePreset(c)),wasm:{}};
 assert.ok(out.template_rejected&&out.snapshot&&out.collection);
 for(const name of ['generate','generateStreaming','generateTwoSided']){let writes=0;out.wasm[name]=error(()=>wasm[name](req,()=>{writes++;}));assert.ok(out.wasm[name]);assert.equal(writes,0);}
 rows.push(out);
}
for(const [key,where] of [['finishingScope','portable-operation'],['finishing_scope','native-params']]){
 if(where==='portable-operation'){
  const file=structuredClone(portable);file.operations[0][key]='walls_only';validateProjectV10(file);
  rows.push({case:'unsafe-sidecar-'+where,accepted_ignored_semantics:true,note:'Unknown operation field does not revise the guarded stage contract; never use this as finishing intent.'});
 }else{
  const req=structuredClone(input);req.project.operations[0].params[key]='walls_only';const n=cli(req);assert.notEqual(n.status,0);let w;const wError=error(()=>{w=wasm.generate(req);});if(w)assert.equal(w.gcode,native.gcode);
  rows.push({case:'unsafe-sidecar-'+where,native_refused:n.stderr.trim().split('\n').at(-1),wasm_refusal:wError,wasm_accepted_unchanged:!!w});
 }
}
const file11=structuredClone(portable);file11.version=11;assert.ok(error(()=>validateProjectV10(file11)));
const request11=structuredClone(input);request11.project.cutting_version=11;assert.notEqual(cli(request11).status,0);assert.ok(error(()=>wasm.generate(request11)));
rows.push({case:'outer-version-11',portable_refused:true,native_refused:true,wasm_refused:true});
const oldCase=receipt.cases.find(c=>c.name==='rectangle-cascade-0.05-0');assert.ok(oldCase);
const oldBytes=readFileSync(resolve(baseline,'dist-test/alpha31/runtime/rectangle-cascade-0.05-0.request.json'));
assert.equal(hash(oldBytes),oldCase.requestSha256);const oldRequest=JSON.parse(oldBytes);
const oldOutput=cli(oldRequest);assert.equal(oldOutput.status,0,oldOutput.stderr);const old=JSON.parse(oldOutput.stdout);
assert.equal(hash(old.gcode),oldCase.programSha256);
const edges=oldRequest.project.segments, xs=edges.flatMap(s=>[s.start.x,s.end.x]),ys=edges.flatMap(s=>[s.start.y,s.end.y]);
const tool=oldRequest.project.tools[0], margin=tool.diameter/2+0.25;
const stage=old.cutting_stages.find(s=>s.stage==='finishing') ?? old.cutting_stages[1];
const interior=old.toolpath.slice(stage.first_segment,stage.end_segment_exclusive).filter(m=>m.kind!=='rapid'&&m.from.z<0&&m.to.z<0).filter(m=>{
 const p={x:(m.from.x+m.to.x)/2,y:(m.from.y+m.to.y)/2};
 return p.x>Math.min(...xs)+margin&&p.x<Math.max(...xs)-margin&&p.y>Math.min(...ys)+margin&&p.y<Math.max(...ys)-margin;
});assert.ok(interior.length>0);
rows.push({case:'historical-L-positive-F-zero',request_sha256:oldCase.requestSha256,program_sha256:hash(old.gcode),matches_alpha31_qualified_program:true,finishing_interior_moves:interior.length,note:'Recorded rectangle has interior finishing cutting moves despite F=0. This characterizes full-area legacy behavior, not wall-height qualification.'});
const result={source_sha:'81e8d47d3ca6860303d534393bad37056ea0cfb2',software_fingerprint_sha256:'1673ddb0e22994cb716b04711c4943c4635d707dc1956c6fa9901c15a6c1224c',native_sha256:receipt.nativeExecutableSha256,wasm_sha256:receipt.wasmSha256,method:'Unmodified current frontend reader functions (TS transpilation only), qualified native CLI and real WASM; hypothetical unsupported inputs. No new contract implemented.',rows,limits:['No installed-native UI or hardware','No new-reader or new-path qualification','Portable validation input immutability tested; restore entry-point ordering inspected in source, not a new browser lifecycle test','HTTP/Tauri handlers inspected for shared typed decoding; not separately invoked in this audit']};
writeFileSync(resolve(root,'docs/releases/evidence/alpha32-preflight/readers.json'),JSON.stringify(result,null,2)+'\n');
console.log('PASS:',rows.length,'compatibility scenarios; strict nested refusals and unsafe sidecar demonstrated.');
