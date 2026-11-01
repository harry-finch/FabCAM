import { snapshotProject, projectHasUnsavedWork } from '../src/lib/state/project-file-ops';
// Manual synthetic benchmark; never imported by the application entry point.
import { setup } from './perf1-project';
import { generationContextKey } from '../src/lib/state/generation-context';
import {
  programExportStatus,
  programWarningSummary,
  qualificationKey,
} from '../src/lib/state/program-qualification';
import { History } from '../src/lib/state/history';
import { updateOperationCommand } from '../src/lib/state/commands';

export async function runFrontendBenchmarks(memory: () => number | null = () => null) {
  const rows = [];
  for (const kind of ['small', 'drawing-10000', 'relief-512'] as const) {
    const p = setup();
    if (kind === 'drawing-10000') {
      const seed = p.transformedImport!.segments;
      const segments = Array.from({ length: 10000 }, (_, i) => ({
        ...seed[i % seed.length],
        start: { x: i % 100, y: Math.floor(i / 100) },
        end: { x: (i % 100) + 1, y: Math.floor(i / 100) },
      }));
      p.transformedImport!.segments = segments;
      p.data.imports[0].source.segments = segments;
    }
    if (kind === 'relief-512')
      p.data.reliefSources = [
        {
          id: 1,
          name: 'PERF SOFTWARE ONLY',
          origin: { x: 0, y: 0 },
          cell: 0.1,
          cols: 512,
          rows: 512,
          grid: {
            kind: 'heightgrid',
            z: Array.from({ length: 512 * 512 }, (_, i) => -(i % 100) / 100),
          },
        },
      ];
    p.gen.inputContextKey = generationContextKey(p, 'linuxcnc');
    p.gen.qualification = {
      key: qualificationKey(p, 'linuxcnc')!,
      phase: 'current',
      front: { warnings: [] },
    };
    const inputSha256 = Array.from(
      new Uint8Array(
        await crypto.subtle.digest('SHA-256', new TextEncoder().encode(p.gen.inputContextKey!)),
      ),
    )
      .map((v) => v.toString(16).padStart(2, '0'))
      .join('');
    const original = JSON.stringify;
    let serializations = 0,
      bytes = 0;
    let exportCounts, summaryCounts;
    try {
      JSON.stringify = ((value: unknown, ...args: unknown[]) => {
        const result = Reflect.apply(original, JSON, [value, ...args]);
        if (value && typeof value === 'object' && 'drawingBindings' in value && 'post' in value) {
          serializations++;
          bytes += new TextEncoder().encode(result).byteLength;
        }
        return result;
      }) as typeof JSON.stringify;
      programExportStatus(p, 'linuxcnc');
      exportCounts = { serializations, bytes };
      serializations = 0;
      bytes = 0;
      programWarningSummary(p, 'linuxcnc');
      summaryCounts = { serializations, bytes };
    } finally {
      JSON.stringify = original;
    }
    const run = () => {
      const rates = p.data.operations.find((o) => o.stages)!.stages!.main;
      const prior = rates.feed_mm_min!;
      rates.feed_mm_min = prior + 1;
      try {
        if (
          !programExportStatus(p, 'linuxcnc').stale ||
          !programWarningSummary(p, 'linuxcnc').stale
        )
          throw new Error('Stale synthetic edit escaped');
      } finally {
        rates.feed_mm_min = prior;
      }
    };
    for (let i = 0; i < 5; i++) run();
    const heapBefore = memory();
    const times = [];
    const heapSamples = [];
    for (let round = 0; round < 7; round++) {
      const start = performance.now();
      for (let i = 0; i < 20; i++) run();
      times.push((performance.now() - start) / 20);
      heapSamples.push(memory());
      await new Promise<void>((resolve) => setTimeout(resolve, 0));
    }
    const undoTimes = [];
    const history = new History();
    const op = p.data.operations.find((o) => o.stages)!;
    for (let round = 0; round < 7; round++) {
      const start = performance.now();
      for (let i = 0; i < 20; i++) {
        history.exec(
          updateOperationCommand(op.id, {
            stages: {
              ...op.stages!,
              main: { ...op.stages!.main, feed_mm_min: op.stages!.main.feed_mm_min! + 1 },
            },
          }),
          p.data,
        );
        history.undo(p.data);
        history.redo(p.data);
        history.undo(p.data);
      }
      undoTimes.push((performance.now() - start) / 20);
    }
    if (programExportStatus(p, 'linuxcnc').stale)
      throw new Error('Undo did not restore synthetic context');
    p.snapshot = () => snapshotProject(p);
    p.documentActive = true;
    p.savedToProject = true;
    p.documentBaseline = JSON.stringify(p.snapshot());
    if (projectHasUnsavedWork(p)) throw new Error('Synthetic saved baseline appears dirty');
    const unsavedTimes = [];
    for (let round = 0; round < 7; round++) {
      const start = performance.now();
      for (let i = 0; i < 20; i++) {
        const rates = p.data.operations.find((o) => o.stages)!.stages!.main;
        const prior = rates.feed_mm_min!;
        rates.feed_mm_min = prior + 1;
        try {
          if (!projectHasUnsavedWork(p)) throw new Error('Unsaved edit reported saved');
        } finally {
          rates.feed_mm_min = prior;
        }
      }
      unsavedTimes.push((performance.now() - start) / 20);
    }
    if (projectHasUnsavedWork(p)) throw new Error('Restored saved baseline appears dirty');
    rows.push({
      kind,
      input_sha256: inputSha256,
      unsaved_check_ms: unsavedTimes,
      persisted_utf8_bytes: new TextEncoder().encode(p.documentBaseline!).byteLength,
      exportCounts,
      summaryCounts,
      times_ms: times,
      heap_before: heapBefore,
      heap_after: memory(),
      heap_samples: heapSamples,
      undo_redo_cycle_ms: undoTimes,
    });
  }
  return rows;
}
