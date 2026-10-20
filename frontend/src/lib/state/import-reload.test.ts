// ADR-0026: a source reload reuses the confirmed unit choice (explicit
// scale); assumed, declared and legacy sources reload without one.
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { ImportResponse } from '../api/types';
import type { ProjectState } from './project.svelte';

const io = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock('../api/env', () => ({ isTauri: () => true }));
vi.mock('@tauri-apps/api/core', () => ({ invoke: io.invoke }));
vi.mock('./text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));

import { reimportFromPath } from './import-ops';

function source(patch: Partial<ImportResponse>): ImportResponse {
  return {
    filename: 'part.svg',
    format: 'svg',
    segments: [],
    layers: [],
    objects: [],
    object_meta: [],
    bbox: { min_x: 0, min_y: 0, max_x: 10, max_y: 10 },
    unit_scale: 1,
    warnings: [],
    unit_status: 'assumed',
    unit_source: '',
    ...patch,
  };
}

function project(src: ImportResponse): ProjectState {
  return {
    data: {
      drawing: undefined,
      operations: [],
      dirty: false,
      imports: [
        {
          id: 3,
          source: src,
          fileTransform: {
            translate: { x: 5, y: 0 },
            rotateDeg: 0,
            scale: 1,
            mirrorX: false,
            mirrorY: false,
          },
          lastImportPath: '/w/part.svg',
        },
      ],
    },
    geometryView: { objects: [] },
    sourceFileStaleNotice: null,
    setError: vi.fn(),
  } as unknown as ProjectState;
}

beforeEach(() => {
  vi.clearAllMocks();
  vi.stubGlobal('window', {});
});

describe('ADR-0026 source reload', () => {
  it('reuses an explicit unit choice and keeps the placement transform', async () => {
    const reloaded = source({ unit_status: 'explicit', unit_scale: 25.4 / 96 });
    io.invoke.mockResolvedValue(reloaded);
    const p = project(source({ unit_status: 'explicit', unit_scale: 25.4 / 96 }));
    expect(await reimportFromPath(p, '/w/part.svg')).toBe(true);
    expect(io.invoke).toHaveBeenCalledWith('import_path', {
      path: '/w/part.svg',
      scale: 25.4 / 96,
    });
    expect(p.data.imports[0].source).toBe(reloaded);
    expect(p.data.imports[0].fileTransform.translate.x).toBe(5);
    expect(p.data.dirty).toBe(true);
  });

  it('reloads assumed, declared and legacy sources without inventing a scale', async () => {
    for (const status of ['assumed', 'declared', 'physical', undefined] as const) {
      io.invoke.mockResolvedValue(source({}));
      const src = source({
        unit_status: status as ImportResponse['unit_status'],
        unit_scale: 25.4,
      });
      await reimportFromPath(project(src), '/w/part.svg');
      expect(io.invoke).toHaveBeenLastCalledWith('import_path', {
        path: '/w/part.svg',
        scale: null,
      });
    }
  });
});
