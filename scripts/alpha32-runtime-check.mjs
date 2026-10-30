/** ALPHA-3.2 real transport campaign. Synthetic source only; no installers. */
import {readFileSync,writeFileSync,mkdirSync,mkdtempSync,copyFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
const root=resolve(import.meta.dirname,'..'), out=resolve(root,'dist-test/alpha32/runtime');mkdirSync(out,{recursive:true});
const hash=x=>createHash('sha256').update(x).digest('hex');
// Snapshot the built executable: concurrent Cargo feature sets can replace the
// shared target path. Every case and the receipt use these same immutable bytes.
const built=resolve(process.env.CARGO_TARGET_DIR||resolve(root,'target'),'debug/ivac');
const binary=resolve(mkdtempSync(resolve(out,'executable-')),'ivac');copyFileSync(built,binary);
const baseline=resolve(root,'dist-test/alpha32/baseline/ivac');
assert.equal(hash(readFileSync(baseline)),'ff4cde0746ec5f57ebe77c68b2bdfad302077f2fc7917e13ce41a9ed8f34c324');
const pkg=resolve(root,'crates/ivac-wasm/pkg'),wasm=await import(pathToFileURL(resolve(pkg,'ivac_wasm.js')));await wasm.default({module_or_path:readFileSync(resolve(pkg,'ivac_wasm_bg.wasm'))});
const oldPkg=resolve(root,'dist-test/alpha32/baseline/pkg'), oldWasm=await import(pathToFileURL(resolve(oldPkg,'ivac_wasm.js')));assert.equal(hash(readFileSync(resolve(oldPkg,'ivac_wasm_bg.wasm'))),'5fc690b54b18774598a982df9529126d3d264d58be0d89cd94b0aae13a46bb31');await oldWasm.default({module_or_path:readFileSync(resolve(oldPkg,'ivac_wasm_bg.wasm'))});
const base=JSON.parse(readFileSync(resolve(root,'tests/fixtures/alpha32/scoped-pocket.request.json'))),shapes=JSON.parse(readFileSync(resolve(root,'tests/fixtures/alpha31/polygons.json')));
const norm=x=>JSON.parse(JSON.stringify(x,(_k,v)=>v instanceof Map?Object.fromEntries(v):v));
function same(a,b,path=''){
 if(typeof a==='number'&&typeof b==='number'&&/\.(from|to|center)\.(x|y|z)$/.test(path)){assert.ok(Math.abs(a-b)<=1e-12,path);return;}
 if(a&&b&&typeof a==='object'&&typeof b==='object'){assert.deepEqual(Object.keys(a).sort(),Object.keys(b).sort(),path);for(const k of Object.keys(b))same(a[k],b[k],`${path}.${k}`);}else assert.deepEqual(a,b,path);
}
const cases=[];
for(const shape of Object.keys(shapes)) for(const strategy of ['cascade','zigzag','spiral']) for(const scope of ['walls_only','floor_only','walls_and_floor']){
 const v=structuredClone(base),pts=shapes[shape],op=v.project.operations[0],f=op.params.stages.finishing;
 v.project.segments=pts.map((a,i)=>({type:'LINE',start:{x:a[0],y:a[1]},end:{x:pts[(i+1)%pts.length][0],y:pts[(i+1)%pts.length][1]},layer:'0',color:7}));op.kind.strategy=strategy;f.scope=scope;f.lateral_allowance_mm=scope==='floor_only'?0:0.1;f.floor_allowance_mm=scope==='walls_only'?0:0.1;
 cases.push([`${shape}-${strategy}-${scope}`,v,strategy==='spiral'&&['concave','irregular_concave'].includes(shape)]);
}
for(const post of ['grbl','cps'])for(const scope of ['walls_only','floor_only','walls_and_floor']){const v=structuredClone(base),f=v.project.operations[0].params.stages.finishing;f.scope=scope;f.lateral_allowance_mm=scope==='floor_only'?0:0.1;f.floor_allowance_mm=scope==='walls_only'?0:0.1;v.post_processor=post;if(post==='cps')v.cps_post={source:{kind:'bundled',id:'shopbot'}};cases.push([`${post}-${scope}`,v,false]);}
for(const mode of ['missing-scope','unknown-scope','contradiction','disabled-unknown','malformed-main','null-finishing','distinct-tool']){
 const v=structuredClone(base),op=v.project.operations[0],f=op.params.stages.finishing;
 if(mode==='missing-scope')delete f.scope;if(mode==='unknown-scope'||mode==='disabled-unknown')f.scope='automatic';if(mode==='disabled-unknown')op.enabled=false;if(mode==='contradiction')f.scope='walls_only';if(mode==='malformed-main')op.params.stages.main.extra=true;
 if(mode==='null-finishing'){op.params.stages.finishing=null;delete op.finish_tool_id;}
 if(mode==='distinct-tool'){v.project.tools.push({...v.project.tools[0],id:2,diameter:1.5});op.finish_tool_id=2;f.cutting.tool_id=2;}
 cases.push([mode,v,!['null-finishing','distinct-tool'].includes(mode)]);
}
// Source curves and retained islands exercise the actual transport paths too.
const lines=pts=>pts.map((a,i)=>({type:'LINE',start:{x:a[0],y:a[1]},end:{x:pts[(i+1)%pts.length][0],y:pts[(i+1)%pts.length][1]},layer:'0',color:7}));
const circle=(x,y,r)=>lines([[x+r,y],[x,y+r],[x-r,y],[x,y-r]]).map(s=>({...s,type:'ARC',bulge:Math.tan(Math.PI/8),center:{x,y}}));
for(const shape of ['circle','rounded','island','annulus'])for(const scope of ['walls_only','floor_only','walls_and_floor']){
 const v=structuredClone(base),f=v.project.operations[0].params.stages.finishing;v.project.tools[0].diameter=6;
 f.scope=scope;f.lateral_allowance_mm=scope==='floor_only'?0:0.1;f.floor_allowance_mm=scope==='walls_only'?0:0.1;
 let segments;
 if(shape==='circle')segments=circle(25,25,8);
 if(shape==='annulus')segments=[...circle(25,25,15),...circle(25,25,5)];
 if(shape==='island')segments=[...lines([[10,10],[50,10],[50,50],[10,50]]),...lines([[24,24],[36,24],[36,36],[24,36]])];
 if(shape==='rounded'){segments=lines([[15,10],[35,10],[40,15],[40,35],[35,40],[15,40],[10,35],[10,15]]);for(const i of [1,3,5,7])Object.assign(segments[i],{type:'ARC',bulge:Math.tan(Math.PI/8)});}
 v.project.segments=segments;cases.push([`${shape}-${scope}`,v,false]);
}
const rows=[];
for(const [name,req,refuses] of cases){
 const file=resolve(out,name+'.request.json');writeFileSync(file,JSON.stringify(req,null,2)+'\n');const start=performance.now();
 const result=spawnSync(binary,['generate','--request',file],{encoding:'utf8',maxBuffer:128*1024*1024});assert.notEqual(result.status,null,name);assert.equal(result.status===0,!refuses,`${name}: ${result.stderr}`);
 const nativeMs=performance.now()-start;
 const old=spawnSync(baseline,['generate','--request',file],{encoding:'utf8',maxBuffer:128*1024*1024});assert.notEqual(old.status,0,`${name}: old native must reject`);assert.throws(()=>oldWasm.generate(req),undefined,`${name}: old WASM must reject`);
 const row={name,requestSha256:hash(readFileSync(file)),accepted:!refuses,nativeMs,oldReadersRefused:true};
 if(refuses){let events=[];for(const call of [()=>wasm.generate(req),()=>wasm.generateStreaming(req,(...v)=>events.push(v)),()=>wasm.generateTwoSided(req)])assert.throws(call);assert.ok(events.every(([event])=>event.kind!=='done' && !('gcode' in event)),`${name}: progress is not published output`);row.progressEventsBeforeRefusal=events.length;row.refusal=result.stderr.trim().split('\n').at(-1);}
 else{
  const native=JSON.parse(result.stdout);row.programSha256=hash(native.gcode);row.moves=native.toolpath.length;row.stages=native.cutting_stages.map(b=>({stage:b.stage,tool:b.parameters.tool_id,moves:b.end_segment_exclusive-b.first_segment}));
  writeFileSync(resolve(out,name+'.native.json'),JSON.stringify(native));
  if(req.post_processor!=='cps'){
   for(const actual of [norm(wasm.generate(req)),norm(wasm.generateStreaming(req,()=>{}))])for(const key of ['gcode','toolpath','cutting_stages','warnings'])same(actual[key],native[key],`${name}.${key}`);
   assert.equal(norm(wasm.generateTwoSided(req)).front.gcode,native.gcode);
   const streamFile=resolve(out,name+'.stream.ngc');const s=spawnSync(binary,['stream-gcode',file,'--output',streamFile],{encoding:'utf8'});assert.equal(s.status,0,s.stderr);assert.equal(readFileSync(streamFile,'utf8'),native.gcode);row.nativeWasmStreamTwoSidedExact=true;
  }else row.wasmCps='Not available in canonical browser WASM; native software only';
 }
 rows.push(row);writeFileSync(resolve(out,'partial-receipt.json'),JSON.stringify(rows,null,2));
}
// A genuine flipped back-side scoped operation, not merely a front-only wrapper.
const paired=[];
for(const scope of ['walls_only','floor_only','walls_and_floor']){
 const req=structuredClone(base),op=req.project.operations[0],f=op.params.stages.finishing;
 f.scope=scope;f.lateral_allowance_mm=scope==='floor_only'?0:0.1;f.floor_allowance_mm=scope==='walls_only'?0:0.1;
 req.project.stock.flip={axis:'x'};req.project.operations.push({...structuredClone(op),id:2,side:'back'});
 const dir=mkdtempSync(resolve(out,'pair-')),file=resolve(dir,'request.json');writeFileSync(file,JSON.stringify(req));
 const native=spawnSync(binary,['stream-gcode',file,'--two-sided','--output',resolve(dir,'pair.ngc')],{encoding:'utf8'});assert.equal(native.status,0,native.stderr);
 const actual=norm(wasm.generateTwoSided(req));assert.ok(actual.back);
 const row={scope,requestSha256:hash(readFileSync(file)),nativeWasmExact:true,programs:{}};
 for(const side of ['front','back']){const program=readFileSync(resolve(dir,`pair.${side}.ngc`),'utf8');assert.equal(program,actual[side].gcode);assert.equal(actual[side].cutting_stages.length,2);row.programs[side]={sha256:hash(program),moves:actual[side].toolpath.length};}
 paired.push(row);
}
const receipt={baseSha:spawnSync('git',['rev-parse','HEAD'],{cwd:root,encoding:'utf8'}).stdout.trim(),uncommitted:true,nativeExecutableSha256:hash(readFileSync(binary)),wasmSha256:hash(readFileSync(resolve(pkg,'ivac_wasm_bg.wasm'))),cases:rows,twoSidedPairs:paired};
writeFileSync(resolve(out,'receipt.json'),JSON.stringify(receipt,null,2)+'\n');console.log(`PASS ${rows.length} cases; ${rows.filter(x=>x.accepted).length} accepted; ${rows.filter(x=>!x.accepted).length} fail-closed; old readers reject all new contracts.`);
