import { expect, it, vi } from 'vitest';
vi.mock('./workspace.svelte', () => ({ workspace: {} }));
vi.mock('./text_preview.svelte', () => ({ resetPreviewCache: vi.fn() }));
vi.mock('./import-ops', () => ({ refreshSourceWatch: vi.fn(), setImported: vi.fn() }));
import { programExportStatus, programWarningSummary } from './program-qualification';
import { setup } from '../../../bench/perf1-project';
import { runFrontendBenchmarks } from '../../../bench/perf1-run';
import { writeFileSync } from 'node:fs';
import { History } from './history';
import { updateOperationCommand } from './commands';
it.skipIf(!process.env.PERF1_BENCHMARK_OUT)(
  'manual PERF1 baseline/candidate diagnostics benchmark',
  async () => {
    const rows = await runFrontendBenchmarks(() => process.memoryUsage().heapUsed);
    writeFileSync(process.env.PERF1_BENCHMARK_OUT!, JSON.stringify(rows, null, 2) + '\n');
  },
  60000,
);
it.skipIf(Boolean(process.env.PERF1_BENCHMARK_BASELINE))(
  'one fresh serialization per diagnostics call; no context survives a subsequent edit or Undo',
  () => {
    const p = setup();
    const history = new History();
    const original = JSON.stringify;
    let contexts = 0;
    const spy = vi.spyOn(JSON, 'stringify').mockImplementation(((
      value: unknown,
      ...args: unknown[]
    ) => {
      if (value && typeof value === 'object' && 'drawingBindings' in value && 'post' in value)
        contexts++;
      return Reflect.apply(original, JSON, [value, ...args]);
    }) as typeof JSON.stringify);
    try {
      expect(programExportStatus(p, 'linuxcnc').stale).toBe(false);
      expect(contexts).toBe(1);
      programWarningSummary(p, 'linuxcnc');
      expect(contexts).toBe(2);
      const op = p.data.operations.find((o) => o.stages)!;
      history.exec(
        updateOperationCommand(op.id, {
          stages: {
            ...op.stages!,
            main: { ...op.stages!.main, feed_mm_min: op.stages!.main.feed_mm_min! + 1 },
          },
        }),
        p.data,
      );
      expect(programExportStatus(p, 'linuxcnc').stale).toBe(true);
      expect(contexts).toBe(3);
      expect(programWarningSummary(p, 'linuxcnc').stale).toBe(true);
      expect(contexts).toBe(4);
      history.undo(p.data);
      expect(programExportStatus(p, 'linuxcnc').stale).toBe(false);
      history.redo(p.data);
      expect(programExportStatus(p, 'linuxcnc').stale).toBe(true);
      p.gen.inputContextKey = '';
      expect(programExportStatus(p, 'linuxcnc').stale).toBe(true);
      expect(programWarningSummary(p, 'linuxcnc').stale).toBe(true);
    } finally {
      spy.mockRestore();
    }
  },
);

it('fresh diagnostics observe in-place tool, machine, stock, origin and source mutations', () => {
  const changes: Array<[string, (p: ReturnType<typeof setup>) => void]> = [
    [
      'tool metadata',
      (p) => {
        p.data.tools[0].substrate = 'carbide';
        p.data.tools[0].coating = 'uncoated';
      },
    ],
    [
      'machine',
      (p) => {
        p.data.machine.fastMoveZ += 1;
      },
    ],
    [
      'stock',
      (p) => {
        p.data.stock.thickness += 1;
      },
    ],
    [
      'work zero',
      (p) => {
        p.data.workOffset.x_mm += 1;
      },
    ],
    [
      'source geometry',
      (p) => {
        p.transformedImport!.segments[0].start.x += 1;
      },
    ],
  ];
  for (const [name, change] of changes) {
    const p = setup();
    expect(programExportStatus(p, 'linuxcnc').stale, name).toBe(false);
    change(p);
    expect(programExportStatus(p, 'linuxcnc').stale, name).toBe(true);
    expect(programWarningSummary(p, 'linuxcnc').stale, name).toBe(true);
  }
});
