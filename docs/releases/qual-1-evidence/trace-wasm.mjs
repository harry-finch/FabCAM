import { readFileSync, writeFileSync } from 'node:fs';
import init, { generate, qual1_take_trace } from '../../../qual-1-checkout/crates/ivac-wasm/qual1-pkg/ivac_wasm.js';
const [input, output] = process.argv.slice(2);
await init({module_or_path:readFileSync(new URL('../../../qual-1-checkout/crates/ivac-wasm/qual1-pkg/ivac_wasm_bg.wasm',import.meta.url))});
function clean(v) { if(v instanceof Map) return Object.fromEntries([...v].map(([k,x])=>[k,clean(x)])); if(Array.isArray(v)) return v.map(clean); if(v && typeof v==='object') return Object.fromEntries(Object.entries(v).map(([k,x])=>[k,clean(x)])); return v; }
const result=generate(JSON.parse(readFileSync(input)));
writeFileSync(output+'.response.json', JSON.stringify(clean(result)));
writeFileSync(output+'.trace.json', qual1_take_trace());
