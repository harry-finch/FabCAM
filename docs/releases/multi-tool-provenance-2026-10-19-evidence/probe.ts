// Read-only Phase A counterexamples. Run using frontend's vite-node, not a qualification fixture.
import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import { verificationInput } from '../../../frontend/src/lib/state/program-qualification';
import { verifyProgram } from '../../../frontend/src/lib/sim/verify-program';
import { computeCellSize } from '../../../frontend/src/lib/sim/config';
const file = JSON.parse(readFileSync(new URL('../ux-c-2026-10-19-evidence/saved-preparation.ivac-project.json', import.meta.url), 'utf8'));
const data = structuredClone(file);
data.settings = {cellResolutionMode:'auto',cellResolutionMm:0.1,maxSimulationCells:250000};
const p:any={data, stockSizingImport:data.imports[0].source,gen:{generated:{toolpath:[]}}};
const base=verificationInput(p,'probe');
data.operations.unshift({id:999,kind:'pause',enabled:true,toolId:0});
assert.throws(()=>verificationInput(p,'probe'),/Missing initial simulation tool/);
console.log('CONFIRMED: leading program-only toolId=0 rejects before simulation');
data.operations.shift();
let freed=false;
class Mock { set_fixtures(){} set_toolpath(s:any){return s.length} advance(){} take_diagnostics(){return {warnings:[]}} push_cell_size_coarsened(){} free(){freed=true} }
const seg={from:{x:0,y:0,z:5},to:{x:1,y:0,z:5},kind:'rapid',gcode_line:1,op_id:999};
assert.throws(()=>verifyProgram(Mock as any,{...base,front:[seg]} as any,'front'),/Missing tool for operation 999/);
assert(freed);
console.log('CONFIRMED: emitted synthetic/unknown op_id not in enabled-op map rejects and frees simulator');
assert.equal(computeCellSize(25.4,data.settings),25.4/15);
assert.equal(computeCellSize(1.5,data.settings),0.1);
console.log(JSON.stringify({auto25_4:computeCellSize(25.4,data.settings),auto1_5:computeCellSize(1.5,data.settings),note:'requested cells only, before cap; no collision-miss claim'}));
console.log('All Phase A counterexample assertions passed; mock proves lookup/lifecycle only, not machine geometry.');
