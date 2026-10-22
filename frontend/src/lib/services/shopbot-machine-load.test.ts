// Exercise the actual machine file import/export path. Only desktop I/O and
// application stores are mocked; the checked-in snapshot and migrations are real.
import { readFileSync } from 'node:fs';
import { describe, expect, it, vi } from 'vitest';
import type { MachineSettings } from '../state/project-types';
import { profileFromCurrent, profilePayload } from '../state/machine_profiles';
import { buildProject } from '../api/build-project';
import { toCpsPostRequest } from '../components/cps-post-form';

const io = vi.hoisted(() => ({
  text: '',
  cancelSave: false,
  project: {
    data: { machine: {} as MachineSettings },
    setMachine: vi.fn(),
    setError: vi.fn(),
  },
  write: vi.fn(),
  invoke: vi.fn(),
}));
vi.mock('../state/project.svelte', () => ({ project: io.project }));
vi.mock('../state/workspace.svelte', () => ({ workspace: {} }));
vi.mock('../state/confirm.svelte', () => ({ confirmStore: {} }));
vi.mock('../api/env', () => ({ isTauri: () => true, isAndroid: () => false }));
vi.mock('../api/http', () => ({ defaultClient: {} }));
vi.mock('../api/client', () => ({ tryParseStructuredError: () => null }));
vi.mock('../i18n', () => ({ t: (key: string) => key }));
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: async () => '/profile.ivac-machine.json',
  save: async () => (io.cancelSave ? null : '/saved.ivac-machine.json'),
}));
vi.mock('@tauri-apps/plugin-fs', () => ({
  readTextFile: async () => io.text,
  writeTextFile: io.write,
}));
vi.mock('@tauri-apps/api/core', () => ({ invoke: io.invoke }));
import { loadMachine, readMachine, saveMachine } from './file_ops';

describe('Sorbonne ShopBot M3 snapshot', () => {
  it('loads, selects the owned post, and survives workspace/profile and file round trips', async () => {
    io.text = readFileSync(
      new URL('../../../../machines/sorbonne-shopbot-prs.ivac-machine.json', import.meta.url),
      'utf8',
    );
    io.project.setMachine.mockImplementation((machine: MachineSettings) => {
      io.project.data.machine = machine;
    });
    await loadMachine();
    expect(io.project.setError).not.toHaveBeenCalled();
    expect(io.invoke).toHaveBeenCalledWith('clear_pipeline_cache_cmd');
    expect(io.project.setMachine).toHaveBeenCalledOnce();
    const machine = io.project.data.machine;
    expect(toCpsPostRequest(machine.cpsPost)).toEqual({
      source: { kind: 'bundled', id: 'shopbot' },
      properties: {},
    });
    const wire = buildProject({
      transformedImport: {
        bbox: { min_x: 0, min_y: 0, max_x: 10, max_y: 10 },
        filename: 'test.svg',
        format: 'svg',
        layers: [],
        object_meta: [],
        objects: [],
        segments: [],
        unit_scale: 1,
        warnings: [],
        unit_status: 'unknown',
        unit_source: '',
      },
      machine,
      tools: [
        {
          id: 1,
          name: 'test only',
          kind: 'endmill',
          diameter: 6,
          flutes: 2,
          speed: 18000,
          plungeRate: 100,
          feedRate: 600,
          coolant: 'off',
        },
      ],
      operations: [
        {
          id: 1,
          name: 'test only',
          enabled: true,
          kind: 'profile',
          toolId: 1,
          sourceLayers: null,
          depth: -1,
          startDepth: 0,
          step: null,
          offset: 'outside',
        },
      ],
    });
    expect(wire).not.toBeNull();
    expect(wire!.operations[0].params.fast_move_z).toBe(0);
    expect(wire!.machine.work_area).toEqual({ x: 1200, y: 600, z: 0 });
    expect(machine.name).toContain('TO VERIFY ON MACHINE');
    expect(machine).toMatchObject({
      unit: 'mm',
      mode: 'mill',
      arcs: false,
      fastMoveZ: 0,
      workArea: { x: 1200, y: 600, z: 0 },
      toolchangeStrategy: 'manual_m0_pause',
      parkAtHome: false,
      optionalStop: false,
      cpsPost: { source: 'bundled', bundledId: 'shopbot', properties: {} },
    });
    for (const unknown of [
      'rapidSpeed',
      'maxFeedMmMin',
      'spindleRpmMin',
      'spindleRpmMax',
      'parkXy',
      'accel',
      'jerk',
    ]) {
      expect(machine).not.toHaveProperty(unknown);
    }
    const profile = profileFromCurrent(machine, [], [], 'mp-sorbonne-shopbot');
    expect(profilePayload(profile)).toMatchObject({ machine, tools: [] });
    expect(profilePayload(profile).machine.physicalId).toEqual(profile.machine.physicalId);
    await saveMachine();
    const saved = JSON.parse(io.write.mock.calls[0][1] as string);
    expect(saved.kind).toBe('machine');
    expect(saved.format_version).toBe(1);
    expect(saved.payload).toEqual(machine);
    io.text = JSON.stringify(saved);
    await loadMachine();
    expect(io.project.data.machine).toEqual(machine);
  });
});

describe('UX6-A staged machine file exchange', () => {
  it('reads and migrates a draft without changing the project or clearing generation caches', async () => {
    vi.clearAllMocks();
    const before = JSON.stringify(io.project.data.machine);
    io.text = JSON.stringify({
      kind: 'machine',
      format_version: 1,
      payload: {
        ...io.project.data.machine,
        name: 'Imported draft',
        spindleRpmMax: undefined,
        postProfile: { name: 'Retained incompatible scaffold', program_end: 'M30' },
      },
    });
    const draft = await readMachine();
    expect(draft?.name).toBe('Imported draft');
    expect(draft?.postProfile?.program_end).toBe('M30');
    expect(draft?.spindleRpmMax).toBeUndefined();
    expect(JSON.stringify(io.project.data.machine)).toBe(before);
    expect(io.project.setMachine).not.toHaveBeenCalled();
    expect(io.invoke).not.toHaveBeenCalled();
  });
  it('exports the displayed draft and preserves the committed project even if the picker is canceled', async () => {
    vi.clearAllMocks();
    const before = JSON.stringify(io.project.data.machine);
    const draft = { ...io.project.data.machine, name: 'Export only', unit: 'inch' as const };
    io.cancelSave = false;
    await saveMachine(draft);
    expect(JSON.parse(io.write.mock.calls[0][1] as string).payload).toEqual(draft);
    io.write.mockClear();
    io.cancelSave = true;
    await saveMachine(draft);
    io.cancelSave = false;
    expect(io.write).not.toHaveBeenCalled();
    expect(io.project.setMachine).not.toHaveBeenCalled();
    expect(JSON.stringify(io.project.data.machine)).toBe(before);
  });
  it('canceled and invalid imports do not replace the machine', async () => {
    vi.clearAllMocks();
    io.text = '';
    expect(await readMachine()).toBeUndefined();
    io.text = '{invalid';
    expect(await readMachine()).toBeUndefined();
    expect(io.project.setError).toHaveBeenCalledOnce();
    expect(io.project.setMachine).not.toHaveBeenCalled();
    expect(io.invoke).not.toHaveBeenCalled();
  });
});
