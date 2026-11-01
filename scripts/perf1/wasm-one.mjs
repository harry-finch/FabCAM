import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const [modulePath,fixture]=process.argv.slice(2);
const w=await import(pathToFileURL(modulePath).href);
const bytes=readFileSync(modulePath.replace(/\.js$/, '_bg.wasm'));
const compiled=await WebAssembly.compile(bytes);
const exports=await w.default({module_or_path:compiled});
const request=JSON.parse(readFileSync(fixture,'utf8'));
const before=process.memoryUsage();const linearBefore=exports.memory.buffer.byteLength;
const start=performance.now();let response,accepted;
try {response=w.generate(request);accepted=true;}catch(e){response={error:typeof e==='string'?JSON.parse(e):e};accepted=false;}
const ms=performance.now()-start;const after=process.memoryUsage();
console.log(JSON.stringify({ms,accepted,linear_before:linearBefore,linear_after:exports.memory.buffer.byteLength,rss_before:before.rss,rss_after:after.rss,response},(_k,v)=>v instanceof Map?Object.fromEntries(v):v));
