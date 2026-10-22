// Run from frontend/: node ../docs/releases/fs-reset-final-evidence/reproduce-report-rates.mjs
// Diagnostic of the current helper; no machine program is generated or exported.
import { createRequire } from 'node:module';
import fs from 'node:fs';
import crypto from 'node:crypto';
const require = createRequire(`${process.cwd()}/package.json`);
const ts = require('typescript');
const source = 'src/lib/state/feeds-speeds.ts';
const text = fs.readFileSync(source, 'utf8');
const js = ts.transpileModule(text, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ES2022 } }).outputText;
const { operationRates } = await import(`data:text/javascript;base64,${Buffer.from(js).toString('base64')}`);
const op = { kind: 'profile', toolId: 1, cutting: { rates: [
  { role: 'rough', tool_id: 1, rpm: 11000, feed_mm_min: 800, plunge_mm_min: 100 },
  { role: 'finish', tool_id: 1, rpm: 13000, feed_mm_min: 410, plunge_mm_min: 110 },
] } };
const tool = { id: 1, speed: 18000, feedRate: 2500, plungeRate: 500 };
const actual = operationRates(op, tool, 'finish');
console.log(JSON.stringify({ source, sha256: crypto.createHash('sha256').update(text).digest('hex'), expected: op.cutting.rates[1], actual, matchesV9: actual.rpm === 13000 && actual.feed_mm_min === 410 && actual.plunge_mm_min === 110 }, null, 2));
