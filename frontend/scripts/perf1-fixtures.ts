// Developer fixture export. Calls the production adapters; never reads workspace/user state.
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { performance } from 'node:perf_hooks';
import { explicitProjectFixture } from '../src/lib/state/explicit-project.fixture';
import { buildProject } from '../src/lib/api/build-project';
import { combineImports } from '../src/lib/state/file-transform';
import { augmentWithStockOutline } from '../src/lib/state/stock-outline';
import { computeFootprint } from '../src/lib/sim/footprint';
import { toCpsPostRequest } from '../src/lib/components/cps-post-form';
import type { ImportResponse, RenderTextLayerResponse } from '../src/lib/api/types';
import type { ProjectFile } from '../src/lib/state/project-types';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const output = resolve(process.argv[2] ?? resolve(root, 'dist-test/perf-v9-authored'));
mkdirSync(output, { recursive: true });
type PerfProject = ProjectFile &
  Required<Pick<ProjectFile, 'stock' | 'tools' | 'machine' | 'operations' | 'textLayers'>>;
const load = (name: string) =>
  explicitProjectFixture(
    JSON.parse(readFileSync(resolve(root, 'docs/shopbot-modernization', name), 'utf8')),
  ) as PerfProject;
const square = load('VCARVE-software-perimeter.ivac-project.json');
const text = load('VCARVE-texte-Test.ivac-project.json');
const sources = load('GEOMETRY-software-sources.ivac-project.json');
const clone = <T>(value: T): T => structuredClone(value);
const cases: { name: string; document: PerfProject; description: string }[] = [];
cases.push({
  name: 'square-outline',
  document: square,
  description: 'Existing software qualification fixture; outline groove.',
});
const medial = clone(square);
medial.operations[0] = {
  ...medial.operations[0],
  fullMedialAxis: true,
} as (typeof medial.operations)[0];
cases.push({
  name: 'square-medial',
  document: medial,
  description: 'Same square, variable-depth strategy.',
});
cases.push({
  name: 'text-Test-medial',
  document: text,
  description:
    'Saved Test text demo, representative surrogate unless human confirms observed case.',
});
const outline = clone(text);
outline.operations[0] = {
  ...outline.operations[0],
  fullMedialAxis: false,
} as (typeof outline.operations)[0];
cases.push({
  name: 'text-Test-outline',
  document: outline,
  description: 'Same saved text with outline strategy; refusals reported separately.',
});
const holes = clone(text);
holes.textLayers[0].text = 'B8';
cases.push({
  name: 'text-B8-medial',
  document: holes,
  description: 'Representative nested glyph holes; not the observed case.',
});
cases.push({
  name: 'pocket-islands',
  document: sources,
  description: 'Existing multi-import Pocket source fixture; normal outer/inner nesting.',
});
const profile = clone(sources);
profile.operations = [profile.operations[1]];
profile.operations[0].enabled = true;
cases.push({
  name: 'profile-contours',
  document: profile,
  description: 'Same SVG/DXF contours, saved Profile multi-layer binding.',
});
const many = clone(profile);
many.operations[0].sourceLayers = ['bench'];
const source = many.imports[0].source;
source.filename = 'PERF1-circles128-poly32.svg';
source.segments = [];
source.objects = [];
source.object_meta = [];
for (let id = 1; id <= 128; id++) {
  const cx = 10 + ((id - 1) % 16) * 18,
    cy = 10 + Math.floor((id - 1) / 16) * 18;
  for (let i = 0; i < 32; i++) {
    const a = (2 * Math.PI * i) / 32,
      b = (2 * Math.PI * (i + 1)) / 32;
    source.segments.push({
      type: 'LINE',
      layer: 'bench',
      color: 7,
      bulge: 0,
      start: { x: cx + 3 * Math.cos(a), y: cy + 3 * Math.sin(a) },
      end: { x: cx + 3 * Math.cos(b), y: cy + 3 * Math.sin(b) },
    });
    source.objects.push(id);
  }
  source.object_meta.push({
    id,
    closed: true,
    layer: 'bench',
    color: 7,
    bbox: { min_x: cx - 3, min_y: cy - 3, max_x: cx + 3, max_y: cy + 3 },
  });
}
source.layers = [{ name: 'bench', color: 7, segment_count: 4096 }];
source.bbox = { min_x: 7, min_y: 7, max_x: 283, max_y: 139 };
many.imports = [many.imports[0]];
many.stock.customX = 300;
many.stock.customY = 160;
cases.push({
  name: 'profile-many-segments',
  document: many,
  description:
    'Synthetic 128 circular polygons × 32 edges (4096 line segments); clearly sampled fixture geometry, no CAM approximation change.',
});
const invalid = clone(sources);
invalid.operations = [invalid.operations[3]];
invalid.operations[0].enabled = true;
cases.push({
  name: 'missing-layer',
  document: invalid,
  description:
    'Known missing source: diagnostics/empty result, never counted as successful machining.',
});

writeFileSync(
  resolve(output, 'moderate.svg'),
  `<svg xmlns="http://www.w3.org/2000/svg" width="300" height="180" viewBox="0 0 300 180">${Array.from({ length: 128 }, (_, i) => `<circle cx="${10 + (i % 16) * 18}" cy="${10 + Math.floor(i / 16) * 18}" r="3"/>`).join('')}</svg>`,
);
writeFileSync(
  resolve(output, 'moderate.dxf'),
  `0\nSECTION\n2\nENTITIES\n${Array.from({ length: 128 }, (_, i) => `0\nCIRCLE\n8\nCircles\n10\n${10 + (i % 16) * 18}\n20\n${10 + Math.floor(i / 16) * 18}\n40\n3\n`).join('')}0\nENDSEC\n0\nEOF\n`,
);
const manifest = [];
for (const entry of cases) {
  const doc = entry.document;
  if (doc.stock.mode !== 'manual')
    throw new Error(
      'Text-aware auto-stock sizing is not emulated; use a reviewed manual-stock fixture.',
    );
  const raw = combineImports(doc.imports);
  // Match ProjectState.stockSizingImport using actual canonical text rendering.
  // Manual stock is centered on rendered text too; omitting this changes stock/WCS inputs.
  let sizing = raw;
  const draft = buildProject({ ...doc, transformedImport: raw });
  const families: (string | null | undefined)[] = [];
  for (const layer of draft?.text_layers ?? []) {
    const response = await fetch('http://127.0.0.1:18794/text/layer', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify(layer),
    });
    if (!response.ok) throw new Error(`Text sizing failed: ${response.status}`);
    const rendered = (await response.json()) as RenderTextLayerResponse;
    families.push(rendered.family_name);
    let bbox = sizing ? { ...sizing.bbox } : null;
    for (const segment of rendered.segments) {
      for (const point of [segment.start, segment.end]) {
        if (!bbox) bbox = { min_x: point.x, min_y: point.y, max_x: point.x, max_y: point.y };
        bbox.min_x = Math.min(bbox.min_x, point.x);
        bbox.min_y = Math.min(bbox.min_y, point.y);
        bbox.max_x = Math.max(bbox.max_x, point.x);
        bbox.max_y = Math.max(bbox.max_y, point.y);
      }
    }
    if (bbox)
      sizing = {
        ...(sizing ?? {
          filename: '',
          format: 'svg',
          segments: [],
          objects: [],
          object_meta: [],
          layers: [],
          unit_scale: 1,
          warnings: [],
        }),
        bbox,
      } as ImportResponse;
  }
  const geometry = doc.stock.visible
    ? augmentWithStockOutline(raw, computeFootprint(sizing, doc.stock, doc.machine.workArea))
    : raw;
  const start = performance.now();
  const project = buildProject({
    ...doc,
    transformedImport: raw,
    geometryView: geometry,
    stockSizingImport: sizing,
  });
  const adapterMs = performance.now() - start;
  if (!project) throw new Error(`No project for ${entry.name}`);
  for (const post of ['linuxcnc', 'cps'] as const) {
    const request = {
      post_processor: post,
      project,
      ...(post === 'cps' ? { cps_post: toCpsPostRequest(doc.machine.cpsPost) } : {}),
    };
    const payload = JSON.stringify(request);
    const name = `${entry.name}-${post}.json`;
    writeFileSync(resolve(output, name), payload + '\n');
    manifest.push({
      name,
      case: entry.name,
      description: entry.description,
      sha256: createHash('sha256')
        .update(payload + '\n')
        .digest('hex'),
      bytes: Buffer.byteLength(payload + '\n'),
      adapter_ms_single_observation: adapterMs,
      input_segments: project.segments.length,
      font_families: families,
      stock_sizing_bbox: sizing?.bbox,
      text: doc.textLayers.map((t) => ({
        text: t.text,
        size_mm: t.sizeMm,
        font_source_kind: t.fontSource.kind,
      })),
      operation: doc.operations,
      tools: doc.tools.map((tool) => ({ ...tool })),
      stock: doc.stock,
    });
  }
}
writeFileSync(resolve(output, 'manifest.json'), JSON.stringify(manifest, null, 2) + '\n');
console.log(
  `Exported ${manifest.length} requests using production buildProject/combineImports/stock adapters.`,
);
