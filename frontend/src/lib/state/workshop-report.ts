import type { ToolpathSegment } from '../api/types';
export function generatedMinimumZ(segments: ToolpathSegment[], opId: number): number | null {
  let min = Infinity;
  for (const s of segments)
    if (s.op_id === opId && s.kind !== 'rapid') min = Math.min(min, s.from.z, s.to.z);
  return Number.isFinite(min) ? min : null;
}

import { countCriticalPipelineWarnings, type PipelineWarning } from '../api/pipeline-warnings';
import { simWarningSeverity, simWarningSummary } from '../sim/warnings';
import type { SimWarning } from '../api/types';
/** A report never hides simulator failures behind an otherwise successful Generate. */
export function workshopWarnings(pipeline: PipelineWarning[], sim: SimWarning[]) {
  const bad =
    countCriticalPipelineWarnings(pipeline) +
    sim.filter((w) => simWarningSeverity(w) === 'critical').length;
  return {
    bad,
    warn: pipeline.length + sim.length - bad,
    top: [...sim.map((w) => ({ kind: w.kind, message: simWarningSummary(w) })), ...pipeline],
  };
}
