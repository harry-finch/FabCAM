import {readFileSync,writeFileSync} from 'node:fs';
const e=new URL('./',import.meta.url); const inputs=JSON.parse(readFileSync(new URL('math-input.json',e)));
const {instance}=await WebAssembly.instantiate(readFileSync(new URL('math.wasm',e)));
const bits=x=>{const b=Buffer.alloc(8);b.writeDoubleBE(x);return b.toString('hex');};
const [a,x,y]=inputs.inputs;
inputs.wasm_bits=[bits(instance.exports.probe_tan(a)),bits(instance.exports.probe_hypot(x,y))];
inputs.node=process.version;inputs.v8=process.versions.v8;
writeFileSync(new URL('MATH_SUMMARY.json',e),JSON.stringify(inputs,null,2)+'\n'); console.log(inputs);
