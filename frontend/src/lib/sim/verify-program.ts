import type { SimDiagnostics } from '../api/types';
import type { VerificationInput } from './qualification';
import { simWarningKey } from './warnings';

/** Same Rust Simulator as playback, without a renderer or playback checkpoints. */
export interface VerificationSimulator {
  set_fixtures(fixtures: unknown): void;
  set_toolpath(segments: unknown): number;
  advance(tool: unknown, from: number, to: number): unknown;
  take_diagnostics(): SimDiagnostics;
  push_cell_size_coarsened(original: number, effective: number, reason: string): void;
  free(): void;
}
export type SimulatorConstructor = new (...args: number[]) => VerificationSimulator;
export function verifyProgram(
  Simulator: SimulatorConstructor,
  input: VerificationInput,
  side: 'front' | 'back',
) {
  const segments = input[side];
  if (!segments?.length) throw new Error('No complete toolpath to verify');
  const g = input.grid;
  const values = Object.values(g);
  if (
    !values.every(Number.isFinite) ||
    g.cellSize <= 0 ||
    g.maxX <= g.minX ||
    g.maxY <= g.minY ||
    g.bottomZ >= g.topZ
  )
    throw new Error('Invalid simulation stock or resolution');
  const sim = new Simulator(g.minX, g.minY, g.maxX, g.maxY, g.cellSize, g.topZ, g.bottomZ);
  try {
    sim.set_fixtures(input.fixtures);
    if (sim.set_toolpath(segments) !== segments.length)
      throw new Error('Incomplete simulator input');
    if (g.cellSize > g.originalCellSize)
      sim.push_cell_size_coarsened(g.originalCellSize, g.cellSize, 'qualification_cell_budget');
    const warnings = new Map<string, SimDiagnostics['warnings'][number]>();
    const tools = side === 'back' ? input.backTools : input.tools;
    for (let from = 0; from < segments.length; ) {
      const reference = segments[from].tool_ref;
      const tool = reference == null ? undefined : tools?.[reference];
      if (!tool) throw new Error('Movement has unknown tool provenance');
      let to = from + 1;
      // Bounded calls also let diagnostics be collected per tool/run: advance
      // clears the Rust diagnostics each call. Never retain just the last run.
      while (to < segments.length && to - from < 256 && segments[to].tool_ref === reference) to++;
      sim.advance(tool, from, to);
      for (const warning of sim.take_diagnostics().warnings)
        warnings.set(simWarningKey(warning), warning);
      from = to;
    }
    return { warnings: [...warnings.values()] };
  } finally {
    sim.free();
  }
}
