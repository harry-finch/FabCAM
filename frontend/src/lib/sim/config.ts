import type { ToolEntry } from '../state/project-types';
import type { AppSettings } from '../state/project-data.svelte';
import { toWireToolKind } from '../api/build-project';
export function toWireTool(t: ToolEntry): Record<string, unknown> {
  const generated = (t as import('./tool-provenance').SimulationTool).simulationDefinition;
  if (generated) return generated;
  return {
    id: t.id,
    name: t.name,
    kind: toWireToolKind(t.kind),
    diameter: t.diameter,
    ...(t.tipDiameter !== undefined ? { tip_diameter: t.tipDiameter } : {}),
    ...(t.tipAngleDeg !== undefined ? { tip_angle_deg: t.tipAngleDeg } : {}),
    ...(t.dragoff !== undefined ? { dragoff: t.dragoff } : {}),
    flutes: t.flutes,
    speed: t.speed,
    plunge_rate: t.plungeRate,
    feed_rate: t.feedRate,
    coolant: t.coolant,
    ...(t.fluteLengthMm !== undefined ? { flute_length_mm: t.fluteLengthMm } : {}),
    ...(t.lengthMm !== undefined ? { length_mm: t.lengthMm } : {}),
    ...(t.compressionTransitionMm !== undefined
      ? { compression_transition_mm: t.compressionTransitionMm }
      : {}),
    ...(t.threadPitchMm !== undefined ? { thread_pitch_mm: t.threadPitchMm } : {}),
    ...(t.shankDiameterMm !== undefined ? { shank_diameter_mm: t.shankDiameterMm } : {}),
    ...(t.holder !== undefined ? { holder: t.holder } : {}),
    // Per-kind sim/holder metadata so the heightfield simulator can
    // pick the right cutter profile and holder-check uses the right
    // shank dims. Bull-nose collapses to a fillet profile in the sim;
    // form-profile (incl. the folded-in T-slot) carves its (z, r)
    // sample list when ≥2 rows are present.
    ...(t.cornerRadiusMm !== undefined ? { corner_radius_mm: t.cornerRadiusMm } : {}),
    // Beam / torch cut width — the laser_beam and plasma_torch sim
    // profiles carve a kerf-radius spot instead of a tool radius.
    ...(t.kerfMm !== undefined && t.kerfMm > 0 ? { kerf_mm: t.kerfMm } : {}),
    // Wear compensation — the sim carves at the effective diameter.
    ...(t.wearOffsetMm !== undefined && t.wearOffsetMm !== 0
      ? { wear_offset_mm: t.wearOffsetMm }
      : {}),
    ...(t.kind === 'form_profile' && t.formProfileMm !== undefined && t.formProfileMm.length >= 2
      ? { form_profile_mm: t.formProfileMm.map((s) => ({ z_mm: s.zMm, r_mm: s.rMm })) }
      : {}),
    ...(t.zShiftMm !== undefined ? { z_shift_mm: t.zShiftMm } : {}),
  };
}

/// `computeFootprint` moved to the THREE-free `./footprint`
/// module so the API layer can resolve the stock box without importing
/// this THREE-heavy driver. Imported for this module's own `build()` use
/// and re-exported for the existing 3D-scene import sites.

/// The in-browser (`?api=wasm`) trial runs the sim single-threaded
/// on the main thread, so a too-fine heightfield makes `set_toolpath` /
/// `advance` / mesh upload stutter the UI. Cap the grid harder there.
/// This is the single fidelity knob: a lower-res carve PREVIEW is an
/// acceptable trial trade — far better than laggy scrubbing. Native /
/// server / Tauri keep the user's full `maxSimulationCells`. ~250k cells
/// ≈ a 500×500 grid over the footprint — smooth, still legible.
export const WASM_TRIAL_SIM_CELL_CAP = 250_000;

/// Effective sim cell cap for the active transport. In wasm-trial mode
/// it's the tighter of the user's setting and [`WASM_TRIAL_SIM_CELL_CAP`];
/// everywhere else it's the user's setting verbatim. Pure so it's
/// unit-tested without the wasm module.
export function effectiveSimCellCap(userMaxCells: number, isWasm: boolean): number {
  const userCap = Math.max(1, userMaxCells);
  return isWasm ? Math.min(userCap, WASM_TRIAL_SIM_CELL_CAP) : userCap;
}

/// Compute cell size from the active tool diameter when settings is in
/// 'auto' mode. Targets ~tool_diameter/15, clamped 0.05..2.0 mm.
export function computeCellSize(
  toolDiameter: number,
  settings: Pick<AppSettings, 'cellResolutionMode' | 'cellResolutionMm'>,
): number {
  if (settings.cellResolutionMode === 'manual') {
    return Math.max(0.01, settings.cellResolutionMm);
  }
  return Math.max(0.05, Math.min(2.0, toolDiameter / 15));
}

export function simulationGrid(
  fp: { minX: number; minY: number; maxX: number; maxY: number },
  diameter: number,
  settings: Pick<AppSettings, 'cellResolutionMode' | 'cellResolutionMm' | 'maxSimulationCells'>,
  isWasm: boolean,
) {
  const cellSize = computeCellSize(diameter, settings);
  const cellCount =
    (Math.ceil((fp.maxX - fp.minX) / cellSize) + 1) *
    (Math.ceil((fp.maxY - fp.minY) / cellSize) + 1);
  const cap = effectiveSimCellCap(settings.maxSimulationCells, isWasm);
  const effectiveCellSize = cellCount > cap ? cellSize * Math.sqrt(cellCount / cap) : cellSize;
  return { cellSize, effectiveCellSize, coarsened: effectiveCellSize !== cellSize };
}
