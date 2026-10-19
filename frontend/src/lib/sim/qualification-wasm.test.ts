import { readFileSync } from 'node:fs';
import { expect, it } from 'vitest';
import { verifyProgram, type SimulatorConstructor } from './verify-program';
import { toWireTool } from './config';
import type { VerificationInput } from './qualification';
import type { ToolpathSegment } from '../api/types';

it('real Rust/WASM simulator distinguishes clean and colliding complete programs on both faces', async () => {
  const url = new URL('../../../../crates/ivac-wasm/pkg/ivac_wasm.js', import.meta.url);
  const wasm = await import(/* @vite-ignore */ url.href);
  const initialized = await wasm.default({
    module_or_path: readFileSync(new URL('./ivac_wasm_bg.wasm', url)),
  });
  const baselineHeapBytes = initialized.memory.buffer.byteLength;
  const started = performance.now();
  const tool = toWireTool({
    id: 1,
    name: 'test',
    kind: 'endmill',
    diameter: 1,
    flutes: 2,
    speed: 18000,
    feedRate: 600,
    plungeRate: 100,
    coolant: 'off',
  });
  const segment = (z: number): ToolpathSegment =>
    ({
      op_id: 1,
      tool_ref: 1,
      gcode_line: 1,
      kind: 'rapid',
      from: { x: 2, y: 2, z },
      to: { x: 4, y: 2, z },
    }) as ToolpathSegment;
  const job: VerificationInput = {
    key: 'wasm',
    grid: {
      minX: 0,
      minY: 0,
      maxX: 10,
      maxY: 10,
      cellSize: 0.1,
      originalCellSize: 0.1,
      topZ: 0,
      bottomZ: -5,
    },
    tools: { 1: tool },
    backTools: { 1: tool },
    fixtures: [],
    front: [segment(2)],
    back: [segment(-1)],
  };
  expect(verifyProgram(wasm.Simulator as SimulatorConstructor, job, 'front').warnings).toEqual([]);
  expect(
    verifyProgram(wasm.Simulator as SimulatorConstructor, job, 'back').warnings.some(
      (w) => w.kind === 'rapid_through_material',
    ),
  ).toBe(true);
  // Earlier warnings cannot vanish at the next chunk; repeat with a new pristine grid.
  job.front = [segment(-1), ...Array.from({ length: 300 }, () => segment(2))];
  expect(
    verifyProgram(wasm.Simulator as SimulatorConstructor, job, 'front').warnings.some(
      (w) => w.kind === 'rapid_through_material' && w.segment_idx === 0,
    ),
  ).toBe(true);
  console.info(
    'WASM verification measurement',
    JSON.stringify({
      baselineHeapBytes,
      finalHeapBytes: initialized.memory.buffer.byteLength,
      elapsedMs: performance.now() - started,
      scope: 'direct Node/WASM; not whole-browser RSS or native installation',
    }),
  );
});

it('real WASM keeps the finishing width and detects its short-body collision within the same op', async () => {
  const url = new URL('../../../../crates/ivac-wasm/pkg/ivac_wasm.js', import.meta.url);
  const wasm = await import(/* @vite-ignore */ url.href);
  const initialized = await wasm.default({
    module_or_path: readFileSync(new URL('./ivac_wasm_bg.wasm', url)),
  });
  const rough = toWireTool({
    id: 41,
    name: 'rough',
    kind: 'endmill',
    diameter: 8,
    flutes: 2,
    speed: 18000,
    feedRate: 600,
    plungeRate: 100,
    coolant: 'off',
    fluteLengthMm: 20,
    shankDiameterMm: 8,
  });
  const finish = {
    ...rough,
    id: 7,
    name: 'finish',
    diameter: 2,
    flute_length_mm: 5,
    shank_diameter_mm: 6,
  };
  const path: ToolpathSegment[] = [
    {
      op_id: 99,
      tool_ref: 1,
      gcode_line: 1,
      kind: 'cut',
      from: { x: 5, y: 2, z: -8 },
      to: { x: 15, y: 2, z: -8 },
    },
    {
      op_id: 99,
      tool_ref: 2,
      gcode_line: 2,
      kind: 'cut',
      from: { x: 5, y: 12, z: -8 },
      to: { x: 15, y: 12, z: -8 },
    },
  ];
  const job: VerificationInput = {
    key: 'bodies',
    grid: {
      minX: 0,
      minY: 0,
      maxX: 20,
      maxY: 20,
      cellSize: 0.1,
      originalCellSize: 0.1,
      topZ: 0,
      bottomZ: -15,
    },
    fixtures: [],
    tools: { 1: rough, 2: finish },
    front: path,
  };
  const actual = verifyProgram(wasm.Simulator, job, 'front');
  expect(actual.warnings.some((w) => w.kind === 'holder_collision' && w.segment_idx === 1)).toBe(
    true,
  );
  const wrong = verifyProgram(wasm.Simulator, { ...job, tools: { 1: rough, 2: rough } }, 'front');
  expect(wrong.warnings.some((w) => w.kind === 'holder_collision')).toBe(false);
  const sim = new wasm.Simulator(0, 0, 20, 20, 0.1, 0, -15);
  try {
    sim.set_toolpath(path);
    sim.advance(rough, 0, 1);
    sim.advance(finish, 1, 2);
    const field = new Float32Array(
      initialized.memory.buffer,
      sim.data_ptr(),
      sim.cols() * sim.rows(),
    );
    const sample = (x: number, y: number) =>
      field[
        Math.floor((y - sim.origin_y()) / sim.cell_size()) * sim.cols() +
          Math.floor((x - sim.origin_x()) / sim.cell_size())
      ];
    expect(sample(10, 12)).toBeCloseTo(-8);
    expect(sample(10, 14)).toBe(0); // Ø2 finish has not carved the Ø8 rough footprint.
  } finally {
    sim.free();
  }
});
