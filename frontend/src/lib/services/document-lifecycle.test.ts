import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { ProjectState } from '../state/project.svelte';
import { History } from '../state/history';
import { setImported } from '../state/import-ops';
import { defaultWorkOffset, identityFileTransform } from '../state/project-types';

const io = vi.hoisted(() => ({
  project: {} as ProjectState,
  choice: vi.fn(),
  save: vi.fn(),
  write: vi.fn(),
  read: vi.fn(),
  open: vi.fn(),
  importFile: vi.fn(),
  recent: vi.fn(),
  invoke: vi.fn(),
}));
vi.mock('../state/project.svelte', () => ({ project: io.project }));
vi.mock('../state/workspace.svelte', () => ({ workspace: { addRecentProject: io.recent } }));
vi.mock('../state/confirm.svelte', () => ({ confirmStore: { askChoice: io.choice } }));
vi.mock('../state/text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('../state/import-ops', () => ({ refreshSourceWatch: vi.fn(), setImported: vi.fn() }));
vi.mock('../api/env', () => ({ isTauri: () => true, isAndroid: () => false }));
vi.mock('../api/http', () => ({ defaultClient: () => ({ importFile: io.importFile }) }));
vi.mock('../api/client', () => ({ tryParseStructuredError: () => null }));
vi.mock('../i18n', () => ({ t: (key: string) => key }));
vi.mock('@tauri-apps/plugin-dialog', () => ({ open: io.open, save: io.save }));
vi.mock('@tauri-apps/plugin-fs', () => ({ readTextFile: io.read, writeTextFile: io.write }));
vi.mock('@tauri-apps/api/core', () => ({ invoke: io.invoke }));
import {
  resetDocument,
  saveProject,
  loadProjectPath,
  loadFile,
  addDrawingFile,
  openAny,
} from './file_ops';
import {
  clearProject,
  snapshotProject,
  restoreProject,
  projectHasUnsavedWork,
  markDocumentSaved,
} from '../state/project-file-ops';

beforeEach(() => {
  vi.clearAllMocks();
  vi.stubGlobal('window', {});
  const p = io.project;
  Object.assign(p, {
    data: {
      imports: [],
      operations: [],
      fixtures: [],
      textLayers: [],
      reliefSources: [],
      tools: [{ id: 1, name: 'Test cutter', kind: 'endmill', diameter: 6 }],
      machine: { name: 'Atelier machine', mode: 'mill' },
      machineProfileId: 'workshop',
      stock: {
        visible: true,
        mode: 'manual',
        margin: 5,
        thickness: 18,
        customX: 200,
        customY: 100,
        offsetX: 25,
      },
      workOffset: defaultWorkOffset(),
      visibleLayers: new Set(),
      dirty: false,
      groupOpsByTool: false,
    },
    gen: {
      generated: { gcode: 'old output' } as { gcode: string } | null,
      generatedBack: { gcode: 'back' } as { gcode: string } | null,
      generating: false,
      pipelineState: 'idle',
      inputContextKey: 'old-context' as string | null,
      reset() {
        this.generated = null;
        this.generatedBack = null;
        this.generating = false;
        this.pipelineState = 'idle';
        this.inputContextKey = null;
      },
      endGenerate() {
        this.generating = false;
      },
    },
    sel: { selectedOpId: 12, pickMode: { kind: 'approach-point', opId: 12 } },
    history: new History(),
    documentActive: true,
    documentName: 'old.ivac-project.json',
    documentEpoch: 1,
    documentBaseline: null,
    savedToProject: false,
    activeProjectPath: '/source.svg',
    loading: false,
    clearProject() {
      clearProject(p);
    },
    snapshot() {
      return snapshotProject(p);
    },
    restore(file: Parameters<typeof restoreProject>[1]) {
      restoreProject(p, file);
    },
    markDocumentSaved(name: string, content = JSON.stringify(p.snapshot())) {
      markDocumentSaved(p, name, content);
    },
    setError: vi.fn(),
    setActiveProjectPath: vi.fn(),
    setImported: vi.fn(() => {
      p.documentActive = true;
      p.data.dirty = true;
    }),
    addImported: vi.fn(() => {
      p.documentActive = true;
      p.data.dirty = true;
    }),
    convertImportedTextEntities: vi.fn().mockResolvedValue(undefined),
  });
  Object.defineProperty(p, 'hasUnsavedWork', {
    configurable: true,
    get: () => projectHasUnsavedWork(p),
  });
  p.documentBaseline = JSON.stringify(p.snapshot());
  io.choice.mockResolvedValue('extra');
  io.save.mockResolvedValue('/saved.ivac-project.json');
  io.write.mockResolvedValue(undefined);
  io.invoke.mockResolvedValue(undefined);
});

describe('Atelier project lifecycle through shared file services', () => {
  it('new keeps machine/tools/dimensions but clears job, both outputs, pick state and history', async () => {
    const p = io.project,
      machine = p.data.machine,
      tools = p.data.tools;
    p.data.operations = [{ id: 12, name: 'old op' }] as typeof p.data.operations;
    expect(await resetDocument(true)).toBe(true);
    expect(p.documentActive).toBe(true);
    expect(p.data.machine).toBe(machine);
    expect(p.data.tools).toBe(tools);
    expect(p.data.stock).toMatchObject({
      customX: 200,
      thickness: 18,
      offsetX: 0,
      offsetY: 0,
      offsetZ: 0,
    });
    expect(p.data.operations).toEqual([]);
    expect(p.gen.generatedBack).toBeNull();
    expect(p.gen.inputContextKey).toBeNull();
    expect(p.sel.pickMode).toBeNull();
    expect(p.documentName).toBeNull();
    expect(p.hasUnsavedWork).toBe(false);
    expect(p.documentEpoch).toBe(2);
  });
  it('protects a configured empty job even after generation cleared dirty', async () => {
    io.project.data.stock.thickness = 24;
    io.project.data.dirty = false;
    io.choice.mockResolvedValue('cancel');
    expect(await resetDocument(false)).toBe(false);
    expect(io.choice).toHaveBeenCalledOnce();
    expect(io.project.documentActive).toBe(true);
    expect(io.project.data.stock.thickness).toBe(24);
  });
  it.each(['cancelled', 'failed'])('aborts close after a %s save', async (mode) => {
    io.project.data.stock.thickness = 24;
    io.choice.mockResolvedValue('primary');
    if (mode === 'cancelled') io.save.mockResolvedValue(null);
    else io.write.mockRejectedValueOnce(new Error('disk full'));
    expect(await resetDocument(false)).toBe(false);
    expect(io.project.documentName).toBe('old.ivac-project.json');
    expect(io.project.hasUnsavedWork).toBe(true);
  });
  it('save-and-close writes once and returns to welcome', async () => {
    io.project.data.stock.thickness = 24;
    io.choice.mockResolvedValue('primary');
    expect(await resetDocument(false)).toBe(true);
    expect(io.write).toHaveBeenCalledOnce();
    expect(io.project.documentActive).toBe(false);
    expect(io.project.hasUnsavedWork).toBe(false);
  });
  it('save records the chosen name without retargeting the source watcher or making output current', async () => {
    await saveProject();
    expect(io.project.documentName).toBe('saved.ivac-project.json');
    expect(io.project.activeProjectPath).toBe('/source.svg');
    expect(io.project.gen.inputContextKey).toBe('old-context');
    expect(io.recent).toHaveBeenCalledWith('/saved.ivac-project.json', 'saved.ivac-project.json');
    expect(io.project.hasUnsavedWork).toBe(false);
  });
  it('edits during a delayed save remain unsaved', async () => {
    let finish!: () => void;
    io.write.mockImplementationOnce(
      () =>
        new Promise<void>((resolve) => {
          finish = resolve;
        }),
    );
    const pending = saveProject();
    await vi.waitFor(() => expect(io.write).toHaveBeenCalled());
    io.project.data.stock.thickness = 30;
    finish();
    await pending;
    expect(io.project.hasUnsavedWork).toBe(true);
  });
  it('picker cancellation leaves the document intact', async () => {
    io.open.mockResolvedValue(null);
    await openAny();
    expect(io.project.documentName).toBe('old.ivac-project.json');
    expect(io.project.documentEpoch).toBe(1);
  });
  it('invalid file format preserves content', async () => {
    io.read.mockResolvedValue('{"kind":"other"}');
    await loadProjectPath('/bad.json');
    expect(io.project.documentEpoch).toBe(1);
    expect(io.project.documentName).toBe('old.ivac-project.json');
    expect(io.project.setError).toHaveBeenCalled();
  });
  it('failed post-parse restore rolls back the actual history and old document', async () => {
    const p = io.project;
    p.history.exec(
      {
        label: 'thickness',
        apply: () => {
          p.data.stock = { ...p.data.stock, thickness: 22 };
        },
        revert: () => {
          p.data.stock = { ...p.data.stock, thickness: 18 };
        },
      },
      p.data,
    );
    const source = p.data.stock;
    io.read.mockResolvedValue('{"kind":"ivac-project","version":1,"tools":[null]}');
    await loadProjectPath('/corrupt.json');
    expect(p.documentName).toBe('old.ivac-project.json');
    expect(p.data.stock).toBe(source);
    expect(p.history.undoSize).toBe(1);
    p.history.undo(p.data);
    expect(p.data.stock.thickness).toBe(18);
    expect(p.gen.generated).toEqual({ gcode: 'old output' });
  });
  it('failed text conversion restores the old job after a drawing replacement', async () => {
    io.importFile.mockResolvedValue({});
    vi.mocked(io.project.convertImportedTextEntities).mockRejectedValueOnce(new Error('bad text'));
    await loadFile({ name: 'new.svg' } as File);
    expect(io.project.documentName).toBe('old.ivac-project.json');
    expect(io.project.gen.generated).toEqual({ gcode: 'old output' });
  });
  it('add drawing uses additive entry and keeps existing operations', async () => {
    io.project.data.operations = [
      { id: 12, name: 'existing' },
    ] as typeof io.project.data.operations;
    io.importFile.mockResolvedValue({});
    await addDrawingFile({ name: 'extra.svg' } as File);
    expect(io.project.addImported).toHaveBeenCalled();
    expect(io.project.data.operations[0].id).toBe(12);
    expect(io.project.documentEpoch).toBe(1);
  });
  it('successful empty-file load is an active clean document', async () => {
    io.read.mockResolvedValue(JSON.stringify(io.project.snapshot()));
    await loadProjectPath('/empty.ivac-project.json');
    expect(io.project.documentActive).toBe(true);
    expect(io.project.hasUnsavedWork).toBe(false);
    expect(io.project.documentName).toBe('empty.ivac-project.json');
  });
  it('restores every saved import transform after import initialization', () => {
    const p = io.project;
    const file = p.snapshot();
    const source = {
      segments: [],
      objects: [],
      layers: [],
      object_meta: [],
      bbox: { min_x: 0, min_y: 0, max_x: 1, max_y: 1 },
    } as unknown as import('../api/types').ImportResponse;
    file.imports = [
      { id: 1, source, fileTransform: { ...identityFileTransform(), scale: 2 } },
      { id: 2, source, fileTransform: { ...identityFileTransform(), translate: { x: 17, y: 0 } } },
    ];
    vi.mocked(setImported).mockImplementationOnce((state) => {
      state.data.imports = [{ id: 1, source, fileTransform: identityFileTransform() }];
    });
    restoreProject(p, file);
    expect(p.data.imports).toEqual(file.imports);
    expect(p.data.imports).toHaveLength(2);
  });
  it('refuses a transition while a file load is in progress', async () => {
    io.project.loading = true;
    expect(await resetDocument(false)).toBe(false);
    expect(io.project.documentEpoch).toBe(1);
  });
});

// UX3 uses the same guarded save/open lifecycle as contour jobs.
it('saves and reopens a standalone surfacing zone in guarded format 6', () => {
  const p = io.project;
  p.data.operations = [
    {
      id: 1,
      name: 'Surface',
      kind: 'planar_surface',
      enabled: true,
      toolId: 1,
      sourceLayers: null,
      depth: -0.5,
      startDepth: 0,
      step: -0.5,
      surface: {
        origin: [20, 30],
        width_mm: 40,
        height_mm: 20,
        margin_mm: 1,
        overtravel_mm: 2,
        stepover_mm: 2.4,
        direction: 'along_y',
      },
    },
  ];
  const saved = p.snapshot();
  expect(saved.kind).toBe('ivac-project-surfacing');
  expect(saved.version).toBe(6);
  const reopened = JSON.parse(JSON.stringify(saved));
  clearProject(p);
  restoreProject(p, reopened);
  expect(p.data.operations[0]).toEqual(saved.operations![0]);
  expect(p.data.imports).toEqual([]);
});

it('saves and restores both V-Carve modes, radius caps and physical bit properties', () => {
  const p = io.project;
  p.data.operations = [false, true].map((fullMedialAxis, i) => ({
    id: i + 1,
    name: 'Software V-Carve',
    kind: 'vcarve' as const,
    enabled: true,
    toolId: 1,
    sourceLayers: null,
    depth: -1,
    startDepth: 0,
    step: -0.2,
    fullMedialAxis,
    carveMaxWidthMm: 2,
    multiPassRefine: false,
  }));
  p.data.tools = [
    {
      id: 1,
      name: 'Software V-bit',
      kind: 'v_bit',
      diameter: 6,
      tipDiameter: 0.5,
      tipAngleDeg: 60,
      flutes: 1,
      speed: 10000,
      plungeRate: 200,
      feedRate: 600,
      coolant: 'off',
    },
  ];
  const saved = JSON.parse(JSON.stringify(snapshotProject(p)));
  clearProject(p);
  restoreProject(p, saved);
  expect(p.data.operations).toEqual(saved.operations);
  expect(p.data.tools).toEqual(saved.tools);
});
