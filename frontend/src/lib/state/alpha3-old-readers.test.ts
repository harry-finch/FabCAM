import { readFileSync } from 'node:fs';
import { runInNewContext } from 'node:vm';
import ts from 'typescript';
import { expect, it } from 'vitest';
import { validateProjectGeometry } from './project-geometry-validation';
import { buildOpEntry } from './op_defaults';
import { captureTemplate, validTemplate } from './operation-templates';
import { defaultProjectTools } from './project-defaults';

// Execute the actual archived v9/v1 reader sources, including their runtime
// dependencies. Geometry validation is unchanged from the archived source SHA.
const root = new URL('../../../../docs/releases/evidence/alpha3-preflight/', import.meta.url);
const names: Record<string, string> = {
  './op_types': 'op-types-v9.ts.txt',
  './cutting-parameters': 'cutting-parameters-v9.ts.txt',
  project: 'project-v9-reader.ts.txt',
  templates: 'operation-templates-v1.ts.txt',
};
const cache = new Map<string, Record<string, (...args: unknown[]) => unknown>>();
function old(name: string): Record<string, (...args: unknown[]) => unknown> {
  if (name === './project-geometry-validation')
    return {
      validateProjectGeometry: (value) =>
        validateProjectGeometry(value as Parameters<typeof validateProjectGeometry>[0]),
    };
  if (cache.has(name)) return cache.get(name)!;
  if (!names[name]) throw new Error(`Unarchived reader dependency ${name}`);
  const source = readFileSync(new URL(names[name], root), 'utf8');
  const compiled = ts.transpileModule(source, {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
  }).outputText;
  const exports: Record<string, (...args: unknown[]) => unknown> = {};
  cache.set(name, exports);
  runInNewContext(compiled, { exports, require: old });
  return exports;
}
it('the actual old portable project reader refuses v10 at its outer boundary', () => {
  const read = old('project').validateProjectV9;
  expect(() => read({ kind: 'ivac-project', version: 10 })).toThrow(/v9/);
});
it('the actual old template reader refuses stages instead of ignoring them', () => {
  const tools = defaultProjectTools();
  const op = buildOpEntry('profile', {
    nextId: 1,
    tools,
    reliefSources: [],
    objectMeta: [],
    selectionIds: [],
  });
  const template = captureTemplate(op, tools, 'Synthetic', 'alpha3');
  expect(validTemplate(template)).toBe(true);
  expect(old('templates').validTemplate(template)).toBe(false);
  const legacy = { ...template };
  delete legacy.stages;
  delete legacy.settings.retainedStockMm;
  legacy.settings.step = -1;
  legacy.tool = {
    ...legacy.tool,
    rough: { rpm: 10000, feed_mm_min: 800, plunge_mm_min: 100 },
    finish: { rpm: 10000, feed_mm_min: 800, plunge_mm_min: 100 },
  };
  // Historical inspection still works; loading it into a v10 operation is separate.
  expect(old('templates').validTemplate(legacy)).toBe(true);
});
