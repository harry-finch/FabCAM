import { performance } from 'node:perf_hooks';
import { cpus, platform, arch } from 'node:os';
import { buildSearchIndex, searchIndex } from '../src/lib/state/palette-search.ts';
// Published, synthetic metadata corpus. No application state or private evidence.
const commands = Array.from({ length: 50 }, (_, i) => ({
  id: `command:${i}`,
  category: 'command',
  label: `Generate operation ${i}`,
  aliases: 'generate generer berechnen pocket poche tasche save speichern',
}));
const tools = Array.from({ length: 5000 }, (_, i) => ({
  id: `tool:${i % 2 ? 'project' : 'inventory'}:${Math.floor(i / 2)}`,
  category: 'tool',
  label: `${i % 4 ? 'Upcut endmill' : 'V-bit'} cutter ${i}`,
  aliases: 'fraise schaftfraser single flute tool cutting workshop stock',
  diameter: i % 113 === 0 ? undefined : [6, 3.175, 6.35, 12][i % 4],
  flutes: i % 107 === 0 ? undefined : (i % 3) + 1,
  angle: i % 4 === 0 ? 60 : undefined,
}));
const templates = Array.from({ length: 200 }, (_, i) => ({
  id: `template:${i}`,
  category: 'template',
  label: `Workshop pocket template ${i}`,
  aliases: 'pocket planar surface profile retained manual',
}));
const corpus = [...commands, ...tools, ...templates];
const start = performance.now();
const index = buildSearchIndex(corpus);
const cold = performance.now() - start;
const queries = [
  '',
  'poche',
  'pokcet',
  '6mm upcut',
  '3.175 single flute',
  '1/8in',
  'V-bit 60',
  'stock',
  'zzzzzzzz',
];
for (const query of queries) searchIndex(index, query);
const measurements = queries.map((query) => {
  const times = [];
  for (let i = 0; i < 30; i++) {
    const start = performance.now();
    searchIndex(index, query);
    times.push(performance.now() - start);
  }
  times.sort((a, b) => a - b);
  return { query, p95_ms: +times[28].toFixed(3), max_ms: +times[29].toFixed(3) };
});
console.log(
  JSON.stringify(
    {
      environment: {
        node: process.version,
        platform: platform(),
        arch: arch(),
        cpu: cpus()[0]?.model,
      },
      records: corpus.length,
      cold_index_ms: +cold.toFixed(3),
      runs_per_query: 30,
      measurements,
    },
    null,
    2,
  ),
);
