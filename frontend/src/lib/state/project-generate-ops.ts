// Generate / pipeline-lifecycle helpers extracted from the ProjectState
// god root. Most forward to the generated-state slice (`p.gen`); the
// handful that cross slices — `setGenerated` (gen + dirty + error +
// playhead), `beginGenerate` / `failGenerate` (error + gen) — live here
// too so the whole lifecycle reads from one place. ProjectState keeps
// thin one-line delegators so the component-facing `project.*` API is
// unchanged.

import type { ProjectState } from './project.svelte';
import type {
  GenerateResponse,
  SimDiagnostics,
  TwoSidedGenerateResponse,
  WiacError,
} from '../api/types';
import type { PipelineNoteEvent } from './generated.svelte';
import { notificationError } from '../api/error-notification';
import { inDrawingFrame } from './work-frame';
import type { WorkOffset } from './project-types';
import { toolpathArcLengths } from '../sim/playhead';

export function setGenerated(p: ProjectState, r: GenerateResponse) {
  p.gen.generated = r;
  // Single-program run clears any prior two-sided back program so a
  // single-sided regenerate can't leave a stale back behind.
  p.gen.generatedBack = null;
  p.gen.generatedVersion += 1;
  // Pre-compute cumulative arc length over the toolpath so playback can
  // advance by physical distance instead of segment count. See
  // `playheadToSegment` for the inverse lookup.
  const { cumLen, totalLen } = toolpathArcLengths(r.toolpath);
  p.gen.toolpathCumLen = cumLen;
  p.gen.toolpathTotalLen = totalLen;
  // A fresh toolpath invalidates the previous heightfield-sim run: its
  // warnings (collisions, rapid-through-material) described the OLD
  // program. Clear them so they don't linger against the new toolpath;
  // the 3D pane's sim re-runs (keyed on generatedVersion) and repopulates
  // when it's visible. Without this, a stale critical sim warning kept
  // showing in the warning chip after a fix-and-regenerate.
  p.gen.simDiagnostics = null;
  p.data.dirty = false;
  p.error = null;
  p.playhead = 1.0;
}

/// Store a two-sided (flip-stock) result: the FRONT program becomes the
/// primary `generated` (all existing consumers see it), and the BACK
/// program is held alongside for the Front/Back gcode tabs + dual-surface
/// preview. Delegates the front-program bookkeeping to `setGenerated`
/// (which first clears `generatedBack`), then sets the back.
export function setGeneratedTwoSided(p: ProjectState, r: TwoSidedGenerateResponse) {
  setGenerated(p, r.front);
  p.gen.generatedBack = r.back ?? null;
}

export function setSimDiagnostics(p: ProjectState, d: SimDiagnostics | null) {
  p.gen.simDiagnostics = d;
}

export function beginGenerate(p: ProjectState) {
  p.error = null;
  p.gen.beginGenerate();
}

export function notePipelineEvent(p: ProjectState, ev: PipelineNoteEvent) {
  p.gen.notePipelineEvent(ev);
}

export function finishGenerate(p: ProjectState) {
  p.gen.finishGenerate();
}

export function cancelGenerate(p: ProjectState) {
  p.gen.cancelGenerate();
}

/// Pipeline failure path. Routes the error through setError and snaps the
/// generate slice back to idle. Spans two slices, so it lives with the
/// lifecycle rather than on either slice.
export function failGenerate(p: ProjectState, err: string | WiacError) {
  if (typeof err !== 'string' && err.diagnostic_preview) {
    const dirty = p.data.dirty;
    // Adapt inspection-only data to the existing preview consumers. No program.
    setGenerated(p, {
      gcode: '',
      toolpath: err.diagnostic_preview.toolpath,
      warnings: err.diagnostic_preview.warnings,
      gcode_index: { lines_to_segment: [], segments_to_line: [] },
      stats: { object_count: 0, closed_object_count: 0, offset_count: 0 },
      time_estimate: {
        total_s: 0,
        arc_s: 0,
        cut_s: 0,
        dwell_s: 0,
        plunge_s: 0,
        rapid_s: 0,
        retract_s: 0,
        spindle_warmup_s: 0,
        toolchange_s: 0,
      },
    });
    p.data.dirty = dirty;
  }
  p.setError(err);
  p.gen.pipelineState = 'idle';
}

export function endGenerate(p: ProjectState) {
  p.gen.endGenerate();
}

/** Keep rejected paths in the same drawing frame as successful previews. */
export function frameGenerationError(input: unknown, origin: WorkOffset): string | WiacError {
  const error = notificationError(input);
  if (typeof error !== 'string' && error.diagnostic_preview) {
    error.diagnostic_preview = inDrawingFrame(error.diagnostic_preview, origin);
  }
  return error;
}
