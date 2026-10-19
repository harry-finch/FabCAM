import type { OpKind } from './op_types';
import type { MachineSettings } from './project-types';

/** UI explanation only: the core still validates every request. */
export function operationCapability(kind: OpKind, post: string, machine: MachineSettings) {
  if (post !== 'cps') return null;
  if (kind === 'homing' || kind === 'probe') return 'uxa.cps_motion_blocked' as const;
  if (kind === 'cycle_marker') return 'uxa.cps_marker_skipped' as const;
  if (
    kind === 'gcode_include' &&
    machine.cpsPost?.source === 'bundled' &&
    machine.cpsPost.bundledId === 'shopbot'
  )
    return 'uxa.shopbot_raw_blocked' as const;
  return null;
}

/** Mirrors setup_resolver and dedicated drivers; never normalizes saved fields. */
export function depthFields(kind: OpKind) {
  const undercut = kind === 't_slot' || kind === 'dovetail';
  return {
    final: kind !== 'chamfer',
    start: kind !== 'chamfer',
    step: !undercut && kind !== 'thread' && kind !== 'drill',
    finishStep: true,
    through: !undercut && !['chamfer', 'vcarve', 'thread'].includes(kind),
    list: !undercut && !['chamfer', 'vcarve', 'thread', 'drill'].includes(kind),
    undercut,
  };
}
export function storedPassDepth(value: string): number | null | undefined {
  if (value.trim() === '') return null;
  const n = Number(value);
  return Number.isFinite(n) && n > 0 ? -n : undefined;
}
/** Disabled and incompatible are separate from generated/current. */
export function operationRunState(
  enabled: boolean,
  hasProgram: boolean,
  current: boolean,
  state: string,
) {
  if (!enabled) return 'disabled';
  if (state === 'running' || state === 'cancelling') return 'pending';
  if (!hasProgram) return 'not_generated';
  return current ? 'current' : 'stale';
}
