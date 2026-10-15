import type { OpEntry } from './op_types';
import type { MachineSettings, ToolEntry } from './project-types';
import type { MachineProfile } from './workspace';
import { profilePayload } from './machine_profiles';
import { deepEqual } from '../components/dialog-draft';

/** Review exactly the migrated payload used by applyMachineProfile. Never remap IDs. */
export function assignmentReview(
  profile: MachineProfile,
  currentTools: readonly ToolEntry[],
  operations: readonly Pick<OpEntry, 'toolId' | 'name'>[],
) {
  const { tools } = profilePayload(profile);
  return operations.flatMap<{ name: string; toolId: number; reason: 'missing' | 'changed' }>(
    (op) => {
      const next = tools.find((t) => t.id === op.toolId);
      const previous = currentTools.find((t) => t.id === op.toolId);
      if (!next) return [{ name: op.name, toolId: op.toolId, reason: 'missing' as const }];
      if (!deepEqual(previous, next))
        return [{ name: op.name, toolId: op.toolId, reason: 'changed' as const }];
      return [];
    },
  );
}

/** Staged first profile uses existing machine fields; no asserted measurements. */
export function newMachineDraft(): MachineSettings {
  return {
    name: '',
    mode: 'mill',
    unit: 'mm',
    comments: true,
    arcs: false,
    toolchangeStrategy: 'manual_m0_pause',
    fastMoveZ: 5,
    workArea: { x: 200, y: 300, z: 50 },
    gcodeDialect: 'linuxcnc',
  };
}
