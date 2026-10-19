import { summarizeWarnings } from './warnings-summary';
import type { ProjectState } from './project.svelte';
import { generationContextKey } from './generation-context';
import { programTools } from '../sim/tool-provenance';
import { computeFootprint } from '../sim/footprint';
import { simulationGrid, toWireTool } from '../sim/config';
import { isWasmTransport } from '../api/transport-mode';
import {
  QUALIFICATION_REVISION,
  type VerificationInput,
  type QualificationPhase,
} from '../sim/qualification';
import { countCriticalPipelineWarnings, type PipelineWarning } from '../api/pipeline-warnings';
import { simWarningSeverity } from '../sim/warnings';

/** Recomputed at every export, including after a Save dialog. No effect timing dependency. */
export function qualificationKey(p: ProjectState, post: string): string | null {
  if (
    !p.gen.generated?.gcode.trim() ||
    p.gen.pipelineState !== 'idle' ||
    p.loading ||
    !p.gen.inputContextKey ||
    p.gen.inputContextKey !== generationContextKey(p, post)
  )
    return null;
  const s = p.data.settings;
  return JSON.stringify([
    QUALIFICATION_REVISION,
    p.documentEpoch,
    p.gen.generatedVersion,
    p.gen.inputContextKey,
    p.gen.generated.tool_provenance,
    p.gen.generatedBack?.tool_provenance,
    s.cellResolutionMode,
    s.cellResolutionMm,
    s.maxSimulationCells,
    isWasmTransport(),
  ]);
}
export function qualificationPhase(p: ProjectState, post: string): QualificationPhase {
  const q = p.gen.qualification;
  const key = qualificationKey(p, post);
  if (!key) return p.gen.generated ? 'stale' : 'unavailable';
  if (!q) return 'pending';
  if (q.key !== key) return 'stale';
  if (q.phase === 'current' && (!q.front || (p.gen.generatedBack && !q.back))) return 'failed';
  return q.phase;
}
export function qualifiedDiagnostics(
  p: ProjectState,
  post: string,
  side: 'front' | 'back' = 'front',
) {
  return qualificationPhase(p, post) === 'current' ? (p.gen.qualification?.[side] ?? null) : null;
}
export function verificationInput(p: ProjectState, key: string): VerificationInput {
  if (p.data.machine.mode !== 'mill')
    throw new Error('Full-program milling simulation is unavailable for this process');
  if (!p.stockSizingImport || !(p.data.stock.thickness > 0))
    throw new Error('A finite stock is required for full-program simulation');
  if (p.gen.generatedBack && p.data.fixtures.length)
    throw new Error('Back-face fixture placement is not qualified');
  if (p.data.stock.flip?.dowels)
    throw new Error('Two-sided registration remains outside this qualification');
  const front = programTools(p.gen.generated!, true);
  const back = p.gen.generatedBack ? programTools(p.gen.generatedBack, true) : undefined;
  const wireTools = (resolved: ReturnType<typeof programTools>) =>
    Object.fromEntries([...resolved.tools].map(([ref, tool]) => [ref, toWireTool(tool)]));
  const tools = wireTools(front);
  const fp = computeFootprint(p.stockSizingImport, p.data.stock);
  const grid = simulationGrid(
    fp,
    Math.min(front.diameter, back?.diameter ?? Infinity),
    p.data.settings,
    isWasmTransport(),
  );
  const topZ = p.data.stock.offsetZ ?? 0;
  return {
    key,
    grid: {
      ...fp,
      cellSize: grid.effectiveCellSize,
      originalCellSize: grid.cellSize,
      topZ,
      bottomZ: topZ - p.data.stock.thickness,
    },
    fixtures: p.data.fixtures,
    tools,
    backTools: back ? wireTools(back) : undefined,
    front: p.gen.generated!.toolpath,
    back: p.gen.generatedBack?.toolpath,
  };
}
export function programExportStatus(p: ProjectState, post: string) {
  const front = (p.gen.generated?.warnings ?? []) as PipelineWarning[];
  const back = (p.gen.generatedBack?.warnings ?? []) as PipelineWarning[];
  const critical = (side: 'front' | 'back') =>
    (qualifiedDiagnostics(p, post, side)?.warnings ?? []).filter(
      (w) => simWarningSeverity(w) === 'critical',
    ).length;
  const phase = qualificationPhase(p, post);
  return {
    stale: !p.gen.inputContextKey || p.gen.inputContextKey !== generationContextKey(p, post),
    qualification: phase,
    critical: countCriticalPipelineWarnings(front) + critical('front'),
    backCritical: countCriticalPipelineWarnings(back) + critical('back'),
    fixture: front.filter((w) => w.kind === 'fixture_path_collision').length,
    backFixture: back.filter((w) => w.kind === 'fixture_path_collision').length,
    workArea: front.filter((w) => w.kind === 'out_of_work_area').length,
    backWorkArea: back.filter((w) => w.kind === 'out_of_work_area').length,
  };
}

/** Both warning surfaces describe the complete front/back programs. */
export function programWarningSummary(p: ProjectState, post: string) {
  const phase = qualificationPhase(p, post);
  const q = phase === 'current' ? p.gen.qualification : null;
  return summarizeWarnings({
    verification: phase,
    simWarnings: [...(q?.front?.warnings ?? []), ...(q?.back?.warnings ?? [])],
    pipelineWarnings: [
      ...(p.gen.generated?.warnings ?? []),
      ...(p.gen.generatedBack?.warnings ?? []),
    ],
    hasGenerated: p.gen.generated != null,
    hasSimDiagnostics: phase === 'current',
    dirty: p.gen.generated != null && p.gen.inputContextKey !== generationContextKey(p, post),
  });
}
