// Reproduce: Rust panic inside the wasm instance, then reuse the SAME instance.
import { readFileSync } from 'node:fs';
const pkg = process.argv[2];
const m = await import(pkg + '/ivac_wasm.js');
m.initSync({ module: readFileSync(pkg + '/ivac_wasm_bg.wasm') });
const quad = (s) => `solid t
facet normal 0 0 1
outer loop
vertex 0 0 1
vertex ${s} 0 1
vertex ${s} ${s} 1
endloop
endfacet
facet normal 0 0 1
outer loop
vertex 0 0 1
vertex ${s} ${s} 1
vertex 0 ${s} 1
endloop
endfacet
endsolid t
`;
const stl = new TextEncoder().encode(quad(10));
const ok = (label, f) => { try { const r = f(); console.log('OK  ', label, JSON.stringify(r)?.slice(0, 80)); } catch (e) { console.log('FAIL', label, e?.constructor?.name, String(e?.message ?? e).slice(0, 160)); } };
ok('baseline fromStl(10mm, 4)', () => { const r = m.fromStl(stl, 4); return { cols: r.cols, rows: r.rows }; });
ok('version()', () => m.version());
ok('fromStl max_dim=u32::MAX', () => m.fromStl(stl, 4294967295));
ok('version() after trap', () => m.version());
ok('fromStl(10mm, 4) after trap', () => { const r = m.fromStl(stl, 4); return { cols: r.cols, rows: r.rows }; });
for (let i = 0; i < 3; i++) ok(`repeat trap #${i}`, () => m.fromStl(stl, 4294967295));
ok('fromStl(10mm, 4) after 4 traps', () => { const r = m.fromStl(stl, 4); return { cols: r.cols, rows: r.rows }; });
