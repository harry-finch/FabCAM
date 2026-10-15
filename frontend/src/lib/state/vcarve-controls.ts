import type { VCarveOp } from './op_types';

/** Presentation only. Never normalizes an authored operation on read. */
export function incompatibleVCarveSettings(op: VCarveOp) {
  const settings = [
    {
      field: 'startDepth',
      label: 'opprops.cut.start_depth',
      value: op.startDepth,
      incompatible: op.startDepth !== 0,
      reset: 0,
    },
    {
      field: 'throughDepth',
      label: 'opprops.cut.through_depth',
      value: op.throughDepth,
      incompatible: (op.throughDepth ?? 0) !== 0,
      reset: undefined,
    },
    {
      field: 'depthList',
      label: 'opprops.cut.depth_list',
      value: op.depthList?.join(', '),
      incompatible: !!op.depthList?.length,
      reset: undefined,
    },
    {
      field: 'finishStep',
      label: 'opprops.cut.finish_step',
      value: op.finishStep,
      incompatible: op.finishStep != null,
      reset: undefined,
    },
    {
      field: 'multiPassRefine',
      label: 'ops.vcarve.refine_pass.label',
      value: String(op.multiPassRefine),
      incompatible: !!op.multiPassRefine,
      reset: undefined,
    },
  ] as const;
  return settings.filter((s) => s.incompatible);
}

/** Keep structured V-Carve refusals readable until explicitly dismissed. */
export function isVCarveRefusal(
  error: string | { kind: string; params?: Record<string, unknown> | null },
  operations: readonly { id: number; kind: string }[],
) {
  if (typeof error === 'string' || error.kind !== 'misconfigured') return false;
  const id = Number(error.params?.op_id);
  return operations.some((op) => op.kind === 'vcarve' && op.id === id);
}
