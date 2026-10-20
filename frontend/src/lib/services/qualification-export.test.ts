// Stage A regressions, actual export service with mocked desktop I/O.
// I/O and application stores are mocked; production restore/snapshot,
// diagnostics setter, warning summarizer and export service execute unchanged.
import { readFileSync } from 'node:fs';
import { beforeEach, expect, it, vi } from 'vitest';
import { setSimDiagnostics } from '../state/project-generate-ops';
import { workshopWarnings } from '../state/workshop-report';
import {
  qualificationKey,
  qualifiedDiagnostics,
  programExportStatus,
} from '../state/program-qualification';
import { exportSafetyProblem } from '../state/generate-palette-policy';
import type { ProjectState } from '../state/project.svelte';
import type { SimDiagnostics } from '../api/types';
const io = vi.hoisted(() => ({
  project: {} as ProjectState,
  write: vi.fn(),
  beforeSave: () => {},
  desktop: true,
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
vi.mock('../api/env', () => ({ isTauri: () => io.desktop, isAndroid: () => false }));
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
    return io.write(args.path, args.text);
  },
}));
import { exportGeneratedGcode } from './file_ops';
const file = JSON.parse(
  readFileSync(
    new URL(
      '../../../../docs/releases/ux-c-2026-10-19-evidence/saved-preparation.ivac-project.json',
      import.meta.url,
    ),
    'utf8',
  ),
);
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
const cpsPost = { source: 'bundled', bundledId: 'shopbot', properties: {} };
beforeEach(() => {
  vi.clearAllMocks();
  io.beforeSave = () => {};
  io.desktop = true;
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
it('P0: actual export service must reject an already known full-program collision', async () => {
  await exportGeneratedGcode('linuxcnc');
  expect(io.write).not.toHaveBeenCalled();
});
it('P0: Home must not clear the program block or report critical warning', () => {
  const status = () => programExportStatus(io.project, 'linuxcnc');
  const policy = { blockOnCriticalSimWarnings: true, blockOnWorkAreaViolation: true };
  expect(exportSafetyProblem(status(), 'front', policy)).toBeDefined();
  setSimDiagnostics(io.project, { warnings: [] }); // Driver publishes restored prefix at Home.
  expect(exportSafetyProblem(status(), 'front', policy)).toBeDefined();
  expect(
    workshopWarnings([], qualifiedDiagnostics(io.project, 'linuxcnc')?.warnings ?? []).bad,
  ).toBe(1);
});
it('P0 control: mandatory fixture refusal still protects actual export', async () => {
  io.project.gen.generated!.warnings = [
    { kind: 'fixture_path_collision', message: 'fixture' },
  ] as never;
  await exportGeneratedGcode('linuxcnc');
  expect(io.write).not.toHaveBeenCalled();
});

it.each(['pending', 'failed', 'stale', 'unavailable'] as const)(
  'actual service blocks %s verification',
  async (phase) => {
    io.project.gen.qualification!.phase = phase;
    await exportGeneratedGcode('linuxcnc');
    expect(io.write).not.toHaveBeenCalled();
  },
);
it('blocks null verification and settings invalidation before effects run', async () => {
  io.project.gen.qualification = null;
  await exportGeneratedGcode('linuxcnc');
  expect(io.write).not.toHaveBeenCalled();
});
it('clean complete program exports; repeated rewind does not change qualification', async () => {
  io.project.gen.qualification!.front = { warnings: [] };
  for (let i = 0; i < 5; i++) {
    setSimDiagnostics(io.project, i % 2 ? collision : { warnings: [] });
    await exportGeneratedGcode('linuxcnc');
  }
  expect(io.write).toHaveBeenCalledTimes(5);
});
it('honours explicitly disabled sim policy but never disables fixture refusal', async () => {
  io.project.data.settings.blockOnCriticalSimWarnings = false;
  io.project.gen.qualification = null;
  await exportGeneratedGcode('linuxcnc');
  expect(io.write).toHaveBeenCalledOnce();
});
it('rejects replacement generation, changed settings, cancelled generation and document switch', async () => {
  io.project.gen.qualification!.front = { warnings: [] };
  for (const mutate of [
    () => io.project.gen.generatedVersion++,
    () => io.project.data.settings.maxSimulationCells++,
    () => io.project.documentEpoch++,
    () => {
      io.project.gen.inputContextKey = null;
    },
  ]) {
    mutate();
    await exportGeneratedGcode('linuxcnc');
    expect(io.write).not.toHaveBeenCalled();
  }
});

it('rechecks settings after the native Save dialog, before the actual write', async () => {
  io.project.gen.qualification!.front = { warnings: [] };
  io.beforeSave = () => {
    io.project.data.settings.cellResolutionMm = 0.2;
  };
  await exportGeneratedGcode('linuxcnc');
  expect(io.write).not.toHaveBeenCalled();
});
it('applies the work-area policy at actual export independently of sim policy', async () => {
  io.project.data.settings.blockOnCriticalSimWarnings = false;
  io.project.gen.generated!.warnings = [{ kind: 'out_of_work_area', message: 'outside' }] as never;
  await exportGeneratedGcode('linuxcnc');
  expect(io.write).not.toHaveBeenCalled();
});

it('browser entry point also refuses critical simulation before any download', async () => {
  io.desktop = false;
  await exportGeneratedGcode('linuxcnc');
  expect(io.project.setError).toHaveBeenCalled();
  expect(io.write).not.toHaveBeenCalled();
});
it('a front-only diagnostic cannot qualify an existing back program', async () => {
  io.project.gen.qualification!.front = { warnings: [] };
  io.project.gen.generatedBack = io.project.gen.generated;
  await exportGeneratedGcode('linuxcnc', 'back');
  expect(io.write).not.toHaveBeenCalled();
});
