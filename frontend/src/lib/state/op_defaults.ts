// Single source of truth for the default field set of a newly-added
// operation, keyed by `OpKind`. Extracted out of `ProjectState.addOperation`
// (which was a ~200-line copy-pasted if-chain) so the defaults are a PURE,
// rune-free function the unit tests can exercise for every kind. The rune
// method is now a thin wrapper: gather the context, call `buildOpEntry`,
// push it through the command bus.

import { seedCutting, cuttingRoles } from './cutting-parameters';
import type { MachineMode, OpEntry, OpKind } from './op_types';
import { prettyOpKind, type DrillCycle, type ReliefSource, type ToolEntry } from './project-types';
import { partitionToolsForModes, pickBestToolForOp } from './tool_picker';

/// The imported-object metadata `pickBestToolForOp` consults — derived from
/// its signature so this module doesn't re-import the generated wire type.
type ObjectMeta = Parameters<typeof pickBestToolForOp>[2];

/// Everything `buildOpEntry` needs from the live project to synthesize a new
/// op. The rune `addOperation` fills this from `this`; tests pass a literal.
export interface OpDefaultsCtx {
  /// Id for the new op (max existing id + 1).
  nextId: number;
  surfaceZone?: { minX: number; minY: number; maxX: number; maxY: number };
  tools: readonly ToolEntry[];
  reliefSources: readonly ReliefSource[];
  /// Canvas-selected object ids — geometry kinds pin the op to this set.
  selectionIds: number[];
  /// Imported-object metadata, for the drill tool-diameter heuristic.
  objectMeta: ObjectMeta;
  /// The machine's effective mode set (`effectiveModes(machine)`),
  /// when known — the default tool pick prefers tools the machine can
  /// actually run (a new Profile on a plasma machine grabs the torch,
  /// not the first endmill in the library).
  modes?: readonly MachineMode[];
}

/// Build the default `OpEntry` for `kind`. Pure: no `$state`, no command bus,
/// no selection side effects — the caller owns those. Mirrors the historical
/// `addOperation` branches field-for-field (locked by `op_defaults.test.ts`).
function buildUnseededOpEntry(kind: OpKind, ctx: OpDefaultsCtx): OpEntry {
  const { nextId, reliefSources, selectionIds, objectMeta, modes } = ctx;
  // Mode-compatible tools first so every "first / best library tool"
  // fallback below lands on something the machine can run. Library
  // order is preserved within each half; no modes keeps the list as-is.
  const parts = modes && modes.length > 0 ? partitionToolsForModes(ctx.tools, modes) : null;
  const tools = parts ? [...parts.compatible, ...parts.incompatible] : ctx.tools;
  // Shared skeleton. Program-only kinds (pause/homing/…) keep `toolId: 0`
  // and carry no geometry/Z-schedule fields; cutter kinds override toolId.
  const base = {
    id: nextId,
    name: prettyOpKind(kind),
    enabled: true,
    toolId: 0,
    sourceCombine: 'auto' as const,
    sourceLayers: null,
  };

  switch (kind) {
    // Program-only building blocks — no tool, no source, no geometry —
    // each carries its own kind-specific fields.
    case 'pause':
      return { ...base, kind: 'pause', message: '' } as OpEntry;
    case 'homing':
      return { ...base, kind: 'homing', retractToSafeZ: true } as OpEntry;
    case 'probe':
      return { ...base, kind: 'probe', axis: 'z', distanceMm: -10, feedMmMin: 100 } as OpEntry;
    case 'cycle_marker':
      return { ...base, kind: 'cycle_marker', label: '' } as OpEntry;
    // External G-code include — path/content default empty; the
    // OpPropertiesPanel file picker fills them.
    case 'gcode_include':
      return {
        ...base,
        kind: 'gcode_include',
        path: '',
        content: '',
        // Per-line unsim fan-out is opt-in; the counted summary fires
        // regardless.
        verboseUnsimWarnings: false,
      } as OpEntry;
    // Relief surfacing follows a target Z-surface, not source geometry.
    // Prefer a ball-/bull-nose tool; bind to the first loaded relief source.
    case 'planar_surface': {
      const tool = tools.find((t) => t.kind === 'endmill' || t.kind === 'compression');
      const zone = ctx.surfaceZone;
      return {
        ...base,
        kind,
        toolId: tool?.id ?? 0,
        depth: -0.5,
        startDepth: 0,
        step: -0.5,
        surface: {
          origin: zone ? [zone.minX, zone.minY] : [20, 20],
          width_mm: zone ? zone.maxX - zone.minX : 100,
          height_mm: zone ? zone.maxY - zone.minY : 100,
          margin_mm: 0,
          overtravel_mm: 1,
          stepover_mm: Math.max(0.01, (tool?.diameter ?? 6) * 0.4),
          direction: 'along_x',
        },
      };
    }
    case 'relief_mill': {
      const ball = tools.find((t) => t.kind === 'ball_nose' || t.kind === 'bull_nose') ?? tools[0];
      return {
        ...base,
        kind: 'relief_mill',
        toolId: ball?.id ?? tools[0]?.id ?? 1,
        depth: -2,
        startDepth: 0,
        step: -1,
        sourceId: reliefSources[0]?.id ?? 0,
        zMinMm: -2,
        zMaxMm: 0,
        invert: false,
        scallopHeightMm: 0.05,
        stepoverMm: null,
        scanDirection: 'along_x',
        alongStepMm: 0.5,
      } as OpEntry;
    }
    // Waterline roughing slices an STL source level-by-level. Prefer a
    // flat / bull-nose endmill (it hogs material like a pocket); bind to
    // the first loaded relief source.
    case 'waterline_rough': {
      const mill = tools.find((t) => t.kind === 'endmill' || t.kind === 'bull_nose') ?? tools[0];
      return {
        ...base,
        kind: 'waterline_rough',
        toolId: mill?.id ?? tools[0]?.id ?? 1,
        depth: -2,
        startDepth: 0,
        step: -1,
        sourceId: reliefSources[0]?.id ?? 0,
        zStepMm: 1,
        stepoverMm: 2,
        floorZMm: 0,
      } as OpEntry;
    }
    // Laser raster engrave follows an image-derived power field.
    // Prefer a laser tool; linear S0..S1000 ramp is the GRBL-agnostic default.
    case 'raster_engrave': {
      const laser = tools.find((t) => t.kind === 'laser_beam') ?? tools[0];
      return {
        ...base,
        kind: 'raster_engrave',
        toolId: laser?.id ?? tools[0]?.id ?? 1,
        depth: 0,
        startDepth: 0,
        step: null,
        sourceId: reliefSources[0]?.id ?? 0,
        resolutionMm: 0.1,
        powerCurve: { kind: 'linear', min: 0, max: 1000 },
        scanDirection: 'along_x',
        link: 'lift_between',
        overscanFactor: 0,
      } as OpEntry;
    }
    // Geometry kinds (profile/pocket/drill/engrave/drag_knife/t_slot/
    // dovetail/vcarve/thread/chamfer). Pin to the canvas selection when one
    // exists; pick the best library tool (drill matches the hole diameter).
    default: {
      const presetSources = selectionIds.length > 0 ? { sourceObjects: selectionIds } : {};
      const tool = pickBestToolForOp(kind, selectionIds, objectMeta, tools);
      return {
        ...base,
        kind,
        toolId: tool?.id ?? 1,
        ...presetSources,
        depth: -2,
        startDepth: 0,
        step: -1,
        offset:
          kind === 'engrave' || kind === 'drag_knife' || kind === 't_slot' || kind === 'dovetail'
            ? 'on'
            : 'outside',
        pocketStrategy: kind === 'pocket' ? 'cascade' : null,
        ...(kind === 'drill' ? { drillCycle: { kind: 'simple', dwellSec: 0 } as DrillCycle } : {}),
        ...(kind === 'profile' || kind === 'pocket' ? { tabWidthMode: 'material_v1' } : {}),
        cutDirection: 'conventional',
        finishCutDirection: 'conventional',
        plunge: { kind: 'direct' },
        xyOverlap: 0.5,
        ...(kind === 'vcarve' ? { multiPassRefine: false } : {}),
      } as OpEntry;
    }
  }
}

export function buildOpEntry(kind: OpKind, ctx: OpDefaultsCtx): OpEntry {
  const op = buildUnseededOpEntry(kind, ctx);
  if ((!ctx.modes || ctx.modes.includes('mill')) && cuttingRoles(op).length) {
    op.cutting = seedCutting(op, [...ctx.tools]);
    const tool = ctx.tools.find((t) => t.id === op.toolId);
    if (op.step === null && tool?.defaultStep !== undefined) op.step = tool.defaultStep;
  }
  return op;
}
