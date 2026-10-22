import { readFileSync } from 'node:fs';
import { expect, it, vi } from 'vitest';
vi.mock('./text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('./workspace.svelte', () => ({ workspace: {} }));
import { reportCuttingText, reportToolIds, reportToolNames } from './report-cutting';
import { cuttingRoles } from './cutting-parameters';
import { buildOpEntry } from './op_defaults';
import { defaultProjectTools, defaultProjectMachine } from './project-defaults';
import { defaultWorkOffset } from './project-types';
import type { OpEntry } from './op_types';
import type { ProjectState } from './project.svelte';
import { snapshotProject, restoreProject } from './project-file-ops';
import { History } from './history';
import { buildReportMarkdown, type ReportData } from '../components/report-markdown';
const tools = defaultProjectTools();
function op(kind: OpEntry['kind'], patch = {}): OpEntry {
  const o = Object.assign(
    buildOpEntry(kind, { nextId: 1, tools, reliefSources: [], selectionIds: [], objectMeta: [] }),
    patch,
  );
  o.cutting = {
    rates: cuttingRoles(o).map((r, i) => ({
      ...r,
      rpm: 13000 + i,
      feed_mm_min: 410 + i,
      plunge_mm_min: 110 + i,
    })),
  };
  return o;
}
it('Profile uses Finish, never stored Rough or tool defaults', () => {
  const p = op('profile');
  expect(reportCuttingText(p)).toBe(`finish · #${p.toolId}: 13001 rpm · 411 / 111 mm/min`);
  expect(reportToolIds(p)).toEqual([p.toolId]);
});
it('Pocket attributes rough/finish to actual tools; Halfpipe has only rough', () => {
  const p = op('pocket', { finishToolId: 42 });
  expect(reportToolIds(p)).toEqual([p.toolId, 42]);
  expect(reportCuttingText(p)).toContain('finish · #42: 13001 rpm · 411 / 111 mm/min');
  expect(
    reportCuttingText(op('pocket', { pocketStrategy: 'halfpipe', finishToolId: 42 })),
  ).not.toContain('finish');
});
it('Drill includes distinct spotting and chamfer tools and rates', () => {
  const p = op('drill', {
    spotFirst: { spotToolId: 7, depth: -1 },
    chamferAfterWidthMm: 1,
    finishToolId: 8,
  });
  expect(reportToolIds(p)).toEqual([p.toolId, 7, 8]);
  expect(reportCuttingText(p)).toContain('spot · #7: 13001 rpm · 411 / 111 mm/min');
  expect(reportCuttingText(p)).toContain('chamfer · #8: 13002 rpm · 412 / 112 mm/min');
  expect(reportToolNames(p, tools)).toContain('#7 —');
  expect(reportCuttingText(op('chamfer', { chamferFinishPass: true }))).toContain('finish');
});
it('missing, wrong-tool and ambiguous rates never fall back to defaults', () => {
  const p = op('profile');
  p.cutting!.rates[1].rpm = null;
  expect(reportCuttingText(p)).toContain('— rpm · 411 / 111');
  p.cutting!.rates[1].tool_id = 999;
  expect(reportCuttingText(p)).toContain('— rpm · — / —');
  p.cutting!.rates.push(
    { ...p.cutting!.rates[0], role: 'finish' },
    { ...p.cutting!.rates[0], role: 'finish' },
  );
  expect(reportCuttingText(p)).toContain('— rpm · — / —');
  expect(reportCuttingText(op('pause'))).toBe('—');
  expect(reportToolIds(op('pause'))).toEqual([]);
});
it('edited rates survive real v9 snapshot/restore and feed the exported report unchanged', () => {
  const operations = [op('profile'), op('pocket', { finishToolId: tools[0].id }), op('drill')];
  operations.forEach((o, i) => (o.id = i + 1));
  operations[0].cutting!.rates[1].feed_mm_min = 321;
  const p = {
    data: {
      operations,
      tools,
      machine: defaultProjectMachine(),
      imports: [],
      visibleLayers: [],
      selectedEntities: [],
      fixtures: [],
      textLayers: [],
      reliefSources: [],
      workOffset: defaultWorkOffset(),
      serializedExtensions: {},
    },
    sel: {},
    history: new History(),
  } as unknown as ProjectState;
  const before = operations.map((o) => reportCuttingText(o));
  const file = JSON.parse(JSON.stringify(snapshotProject(p)));
  expect(file.version).toBe(9);
  restoreProject(p, file);
  expect(p.data.operations.map((o) => reportCuttingText(o))).toEqual(before);
  const md = buildReportMarkdown({
    projectName: 'QA',
    generatedBy: 'test',
    dateIso: '',
    dateDisplay: '',
    stats: null,
    estimatedTime: null,
    tools: [],
    warnings: { bad: 0, warn: 0, top: [] },
    ops: p.data.operations.map((o) => ({
      name: o.name,
      kind: o.kind,
      tool: reportToolNames(o, tools),
      source: '',
      depth: '',
      status: '',
      savedRates: reportCuttingText(o),
    })),
  } satisfies ReportData);
  before.forEach((text) => expect(md).toContain(text));
  const screen = readFileSync(
    new URL('../components/ReportDialog.svelte', import.meta.url),
    'utf8',
  );
  expect(screen).toContain('reportCuttingText(op,');
  expect(screen).toContain('savedRates: opRates(op)');
  expect(screen).toContain('{opRates(op)}');
  expect(screen).not.toContain('operationRates');
});
