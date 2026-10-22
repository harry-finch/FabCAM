import { defaultProjectMachine } from '../state/project-defaults';
import { explicitProjectFixture } from '../state/explicit-project.fixture';
// Stage A regressions, actual export service with mocked desktop I/O.
// I/O and application stores are mocked; production restore/snapshot,
// diagnostics setter, warning summarizer and export service execute unchanged.
import { readFileSync } from 'node:fs';
import { beforeEach, expect, it, vi } from 'vitest';
import { qualificationKey } from '../state/program-qualification';
import type { ProjectState } from '../state/project.svelte';
import type { SimDiagnostics } from '../api/types';
const io = vi.hoisted(() => ({
  project: {} as ProjectState,
  write: vi.fn(),
  beforeSave: () => {},
}));
vi.mock('../state/project.svelte', () => ({ project: io.project }));
vi.mock('../state/workspace.svelte', () => ({ workspace: {} }));
vi.mock('../state/confirm.svelte', () => ({ confirmStore: {} }));
vi.mock('../state/text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('../state/import-ops', () => ({ setImported: vi.fn(), refreshSourceWatch: vi.fn() }));
vi.mock('../state/generation-context', () => ({
  generationContextKey: () => 'current',
  cornerReliefGenerationError: () => null,
}));
vi.mock('../api/env', () => ({ isTauri: () => true, isAndroid: () => false }));
vi.mock('../api/http', () => ({ defaultClient: {} }));
vi.mock('../api/client', () => ({ tryParseStructuredError: () => null }));
vi.mock('../i18n', () => ({ t: (key: string) => key }));
vi.mock('@tauri-apps/plugin-dialog', () => ({
  save: async () => {
    io.beforeSave();
    return '/not-written.ngc';
  },
}));
vi.mock('@tauri-apps/plugin-fs', () => ({ writeTextFile: io.write }));
vi.mock('@tauri-apps/api/core', () => ({
  invoke: (command: string, args: { path: string; text: string }) => {
    expect(command).toBe('publish_program_file');
    io.write(args.path, args.text);
    return { path: args.path, cleanupWarning: null };
  },
}));
const historicalFile = JSON.parse(
  readFileSync(
    new URL(
      '../../../../docs/releases/ux-c-2026-10-19-evidence/saved-preparation.ivac-project.json',
      import.meta.url,
    ),
    'utf8',
  ),
);
const file = explicitProjectFixture(historicalFile);
const collision: SimDiagnostics = {
  warnings: [
    {
      kind: 'rapid_through_material',
      segment_idx: 0,
      worst_x: 0,
      worst_y: 0,
      rapid_pz: 0,
      subkind: 'tip',
      worst_cell_z: 1,
    },
  ],
} as SimDiagnostics;
const cpsPost = { source: 'bundled' as const, bundledId: 'shopbot', properties: {} };
beforeEach(() => {
  vi.clearAllMocks();
  io.beforeSave = () => {};
  Object.assign(io.project, {
    data: {
      ...structuredClone(file),
      serializedExtensions: {},
      machine: { ...file.machine, cpsPost },
      settings: {
        cellResolutionMode: 'auto',
        cellResolutionMm: 0.1,
        maxSimulationCells: 250000,
        blockOnCriticalSimWarnings: true,
        blockOnWorkAreaViolation: true,
      },
    },
    documentEpoch: 1,
    loading: false,
    sel: {},
    history: { clear: vi.fn() },
    setError: vi.fn(),
    gen: {
      generatedVersion: 1,
      generated: { gcode: 'G0 X1\n', warnings: [], toolpath: [] },
      generatedBack: null,
      pipelineState: 'idle',
      inputContextKey: 'current',
      simDiagnostics: collision,
    },
  });
  io.project.gen.qualification = {
    key: qualificationKey(io.project, 'linuxcnc')!,
    phase: 'current',
    front: collision,
  };
});
import { restoreProject, snapshotProject } from '../state/project-file-ops';
import { History } from '../state/history';
import { setMachineCommand } from '../state/commands';
it('P1: real restore/save must not inherit an absent optional post', () => {
  expect(file.machine?.cpsPost).toBeUndefined();
  restoreProject(io.project, structuredClone(file));
  expect(snapshotProject(io.project).machine?.cpsPost).toBeUndefined();
});
it('P1 control: explicitly authored inactive CPS data is preserved', () => {
  restoreProject(io.project, { ...structuredClone(file), machine: { ...file.machine, cpsPost } });
  expect(snapshotProject(io.project).machine?.cpsPost).toEqual(cpsPost);
});

it('CPS -> LinuxCNC -> CPS and reopen preserve authored settings, tools and unknown keys', () => {
  const cps = {
    ...structuredClone(file),
    machine: {
      ...file.machine,
      gcodeDialect: 'cps' as const,
      cpsPost,
      futureExtension: { retained: 42 },
    },
  };
  restoreProject(io.project, cps);
  const authored = JSON.parse(JSON.stringify(snapshotProject(io.project)));
  restoreProject(io.project, structuredClone(file));
  expect(io.project.data.machine.cpsPost).toBeUndefined();
  const definitions = (tools: import('../state/project-types').ToolEntry[]) =>
    tools.map(({ fsLocalIdentity: _identity, ...tool }) => tool);
  expect(definitions(io.project.data.tools)).toEqual(definitions(authored.tools));
  restoreProject(io.project, authored);
  expect(snapshotProject(io.project)).toEqual(authored);
  expect(io.project.history.clear).toHaveBeenCalledTimes(3);
});
it('empty v9 tools and operations do not inherit previous job or stock metadata', () => {
  io.project.data.stock.offsetZ = 88;
  io.project.data.stock.flip = { anything: true } as never;
  const old = {
    ...structuredClone(file),
    machine: { ...defaultProjectMachine(), toolchangeStrategy: 'atc' as const },
    tools: [],
    operations: [],
    stock: undefined,
  };
  restoreProject(io.project, old);
  expect(io.project.data.machine.toolchangeStrategy).toBe('atc');
  expect(io.project.data.machine.gcodeDialect).toBe('linuxcnc');
  expect(io.project.data.machine.cpsPost).toBeUndefined();
  expect(io.project.data.tools).toEqual([]);
  expect(io.project.data.stock.offsetZ).toBeUndefined();
  expect(io.project.data.stock.flip).toBeUndefined();
  expect(io.project.data.operations).toEqual([]);
});
it('machine edits after load remain undoable without reviving previous-project values', () => {
  restoreProject(io.project, structuredClone(file));
  const h = new History();
  const before = structuredClone(io.project.data.machine);
  h.exec(setMachineCommand({ ...before, name: 'edited' }), io.project.data);
  expect(io.project.data.machine.name).toBe('edited');
  h.undo(io.project.data);
  expect(io.project.data.machine).toEqual(before);
  h.redo(io.project.data);
  expect(io.project.data.machine.name).toBe('edited');
  expect(io.project.data.machine.cpsPost).toBeUndefined();
});
