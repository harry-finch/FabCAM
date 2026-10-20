// ADR-0026: the open/add-drawing flows gate on the import summary. A unit
// choice re-imports the same file with an explicit scale; cancelling commits
// nothing; fatal importer errors never become partial geometry.
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { ImportResponse } from '../api/types';
import type { ProjectState } from '../state/project.svelte';

const io = vi.hoisted(() => ({
  project: {} as ProjectState,
  importFile: vi.fn(),
  invoke: vi.fn(),
  reportError: vi.fn(),
}));
vi.mock('../state/project.svelte', () => ({ project: io.project }));
vi.mock('../state/workspace.svelte', () => ({ workspace: { addRecentProject: vi.fn() } }));
vi.mock('../state/confirm.svelte', () => ({ confirmStore: { askChoice: vi.fn() } }));
vi.mock('../state/text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('../state/import-ops', () => ({ refreshSourceWatch: vi.fn(), setImported: vi.fn() }));
vi.mock('../api/env', () => ({ isTauri: () => true, isAndroid: () => false }));
vi.mock('../api/http', () => ({ defaultClient: () => ({ importFile: io.importFile }) }));
vi.mock('../api/client', () => ({ tryParseStructuredError: () => null }));
vi.mock('../i18n', () => ({ t: (key: string) => key }));
vi.mock('@tauri-apps/api/core', () => ({ invoke: io.invoke }));
vi.mock('../state/document-load', () => ({
  commitDocumentLoad: async (_p: unknown, apply: () => Promise<void>) => apply(),
}));

import { addDrawingFile, loadFile, loadFromPath } from './file_ops';
import { importReview } from '../state/import-review';
import { UNIT_SCALES } from '../state/import-units';

function result(patch: Partial<ImportResponse> = {}): ImportResponse {
  return {
    filename: 'logo.svg',
    format: 'svg',
    segments: [],
    layers: [],
    objects: [],
    object_meta: [],
    bbox: { min_x: 0, min_y: 0, max_x: 96, max_y: 48 },
    unit_scale: 1,
    warnings: ["SVG size is not in physical units (width='96px', height='48px')"],
    unit_status: 'assumed',
    unit_source: "width='96px', height='48px'",
    ...patch,
  };
}

async function pendingReview() {
  await vi.waitFor(() => expect(importReview.pending).not.toBeNull());
  return importReview.pending!;
}

beforeEach(() => {
  vi.clearAllMocks();
  vi.stubGlobal('window', {});
  importReview.finish(null);
  Object.assign(io.project, {
    data: { imports: [], dirty: false },
    loading: false,
    loadingMessage: null,
    error: null,
    clearProject: vi.fn(),
    setImported: vi.fn(),
    addImported: vi.fn(),
    setActiveProjectPath: vi.fn(),
    setError: vi.fn(),
    convertImportedTextEntities: vi.fn().mockResolvedValue(undefined),
  });
});

describe('ADR-0026 import summary gate', () => {
  it('re-imports the same file with the chosen scale and commits that result', async () => {
    const file = new File(['<svg/>'], 'logo.svg');
    const chosen = result({ unit_status: 'explicit', unit_scale: UNIT_SCALES.px96, warnings: [] });
    io.importFile.mockResolvedValueOnce(result()).mockResolvedValueOnce(chosen);
    const done = loadFile(file);
    const req = await pendingReview();
    expect(io.project.loading).toBe(false); // the overlay is lifted while reviewing
    importReview.finish(await req.rescale!(UNIT_SCALES.px96));
    await done;
    expect(io.importFile).toHaveBeenLastCalledWith(file, undefined, UNIT_SCALES.px96);
    expect(io.project.setImported).toHaveBeenCalledWith(chosen);
  });

  it('cancelling the summary commits nothing', async () => {
    io.importFile.mockResolvedValueOnce(result());
    const done = addDrawingFile(new File(['<svg/>'], 'logo.svg'));
    await pendingReview();
    importReview.finish(null);
    await done;
    expect(io.project.addImported).not.toHaveBeenCalled();
    expect(io.project.loading).toBe(false);
  });

  it('clean imports with reliable units skip the summary', async () => {
    const clean = result({ unit_status: 'physical', warnings: [] });
    io.importFile.mockResolvedValueOnce(clean);
    await loadFile(new File(['<svg/>'], 'part.svg'));
    expect(importReview.pending).toBeNull();
    expect(io.project.setImported).toHaveBeenCalledWith(clean);
  });

  it('a fatal importer error is reported and never committed', async () => {
    io.importFile.mockRejectedValueOnce(new Error('DXF ARC has extrusion direction (0, 1, 0)'));
    await loadFile(new File(['x'], 'tilted.dxf'));
    expect(importReview.pending).toBeNull();
    expect(io.project.setImported).not.toHaveBeenCalled();
    expect(io.project.setError).toHaveBeenCalled();
  });

  it('desktop path import passes the explicit scale to the importer', async () => {
    const chosen = result({ unit_status: 'explicit', unit_scale: 25.4, warnings: [] });
    io.invoke.mockImplementation(async (cmd: string, args: { scale: number | null }) => {
      if (cmd !== 'import_path') return undefined;
      return args.scale === null ? result({ format: 'dxf' }) : chosen;
    });
    const done = loadFromPath('/parts/plate.dxf');
    const req = await pendingReview();
    importReview.finish(await req.rescale!(25.4));
    await done;
    expect(io.invoke).toHaveBeenCalledWith('import_path', {
      path: '/parts/plate.dxf',
      scale: 25.4,
    });
    expect(io.project.setImported).toHaveBeenCalledWith(chosen, '/parts/plate.dxf');
  });
});
