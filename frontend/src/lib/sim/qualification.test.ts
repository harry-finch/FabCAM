import { describe, expect, it } from 'vitest';
import { QualificationRun, type VerificationWorker } from './qualification-run';
import type { Qualification, VerificationInput, VerificationReply } from './qualification';
import { verifyProgram } from './verify-program';
import type { SimDiagnostics, ToolpathSegment } from '../api/types';
const seg = (op_id = 1) =>
  ({
    op_id,
    tool_ref: op_id,
    kind: 'rapid',
    from: { x: 0, y: 0, z: 0 },
    to: { x: 1, y: 0, z: 0 },
    gcode_line: 1,
  }) as ToolpathSegment;
const input = (): VerificationInput => ({
  key: 'job1',
  grid: {
    minX: 0,
    minY: 0,
    maxX: 10,
    maxY: 10,
    cellSize: 1,
    originalCellSize: 0.5,
    topZ: 0,
    bottomZ: -5,
  },
  front: [seg()],
  fixtures: [],
  tools: { 1: { id: 1 } },
  backTools: { 1: { id: 1 } },
});
class Worker implements VerificationWorker {
  onmessage: VerificationWorker['onmessage'] = null;
  onerror: VerificationWorker['onerror'] = null;
  onmessageerror: VerificationWorker['onmessageerror'] = null;
  terminated = false;
  sent?: VerificationInput;
  postMessage(value: VerificationInput) {
    this.sent = value;
  }
  terminate() {
    this.terminated = true;
  }
  reply(data: VerificationReply) {
    this.onmessage?.({ data } as MessageEvent<VerificationReply>);
  }
}
const result = (key = 'job1') => ({
  key,
  front: { warnings: [] },
  elapsedMs: 5,
  simulatorVersion: { pipeline: 62 },
});
describe('request-scoped background qualification', () => {
  it('publishes pending then complete and terminates/releases the worker', () => {
    const worker = new Worker();
    const states: Qualification[] = [];
    const run = new QualificationRun(() => worker);
    run.start(input(), (s) => states.push(s));
    expect(states.map((s) => s.phase)).toEqual(['pending']);
    worker.reply({ result: result() });
    expect(states.map((s) => s.phase)).toEqual(['pending', 'current']);
    expect(worker.terminated).toBe(true);
  });
  it('discards callbacks from cancelled/replaced jobs, including old errors', () => {
    const workers: Worker[] = [];
    const states: Qualification[] = [];
    const run = new QualificationRun(() => {
      const w = new Worker();
      workers.push(w);
      return w;
    });
    run.start(input(), (s) => states.push(s));
    run.start({ ...input(), key: 'job2' }, (s) => states.push(s));
    workers[0].reply({ result: result() });
    workers[0].reply({ error: 'late' });
    expect(states.map((s) => s.phase)).toEqual(['pending', 'pending']);
    run.cancel();
    workers[1].reply({ result: result('job2') });
    expect(states).toHaveLength(2);
    expect(workers.every((w) => w.terminated)).toBe(true);
  });
  it.each(['error', 'mismatch', 'back-missing', 'messageerror', 'worker-error'] as const)(
    'fails closed: %s',
    (failure) => {
      const w = new Worker();
      let state: Qualification | undefined;
      const run = new QualificationRun(() => w);
      run.start({ ...input(), back: [seg()] }, (s) => {
        state = s;
      });
      if (failure === 'error') w.reply({ error: 'WASM failed' });
      else if (failure === 'worker-error') w.onerror?.({ message: 'OOM' } as ErrorEvent);
      else if (failure === 'messageerror') w.onmessageerror?.({} as MessageEvent);
      else w.reply({ result: result(failure === 'mismatch' ? 'obsolete' : 'job1') });
      expect(state?.phase).toBe('failed');
      expect(w.terminated).toBe(true);
    },
  );
  it('handles unavailable worker construction', () => {
    let state: Qualification | undefined;
    new QualificationRun(() => {
      throw Error('no worker');
    }).start(input(), (s) => {
      state = s;
    });
    expect(state?.phase).toBe('failed');
  });
});
describe('complete simulator traversal', () => {
  it('collects diagnostics across chunks/tools and frees grids independently for both sides', () => {
    const ranges: number[][] = [];
    let freed = 0;
    let fixtures = 0;
    class Sim {
      last = 0;
      set_fixtures() {
        fixtures++;
      }
      set_toolpath(s: ToolpathSegment[]) {
        return s.length;
      }
      advance(_tool: unknown, from: number, to: number) {
        ranges.push([from, to]);
        this.last = from;
      }
      take_diagnostics(): SimDiagnostics {
        return {
          warnings: [
            { kind: 'rapid_through_material', segment_idx: this.last, worst_x: 0, worst_y: 0 },
          ],
        } as SimDiagnostics;
      }
      push_cell_size_coarsened() {}
      free() {
        freed++;
      }
    }
    const job = { ...input(), front: Array.from({ length: 600 }, () => seg()), back: [seg()] };
    const front = verifyProgram(Sim, job, 'front');
    const back = verifyProgram(Sim, job, 'back');
    expect(ranges).toEqual([
      [0, 256],
      [256, 512],
      [512, 600],
      [0, 1],
    ]);
    expect(front.warnings).toHaveLength(3);
    expect(back.warnings).toHaveLength(1);
    expect(freed).toBe(2);
    expect(fixtures).toBe(2);
  });
  it('never returns complete on truncated ingestion or simulator failure and always frees', () => {
    let freed = 0;
    class Sim {
      set_fixtures() {}
      set_toolpath() {
        return 0;
      }
      advance() {}
      take_diagnostics() {
        return { warnings: [] };
      }
      push_cell_size_coarsened() {}
      free() {
        freed++;
      }
    }
    expect(() => verifyProgram(Sim, input(), 'front')).toThrow('Incomplete');
    expect(freed).toBe(1);
    expect(() =>
      verifyProgram(Sim, { ...input(), grid: { ...input().grid, cellSize: NaN } }, 'front'),
    ).toThrow('Invalid');
    expect(freed).toBe(1);
  });
});
