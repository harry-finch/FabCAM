/** Result-oriented presentation only; the existing boolean remains the saved identity. */
export function vcarveStrategy(fullMedialAxis?: boolean) {
  return fullMedialAxis
    ? ({
        value: 'medial',
        label: 'ops.vcarve.mode.medial_axis',
        description: 'ops.vcarve.description.medial',
        scope: 'ops.vcarve.scope.medial',
        depthLabel: 'ops.vcarve.depth.maximum',
        depthHelp: 'ops.vcarve.depth.medial.help',
      } as const)
    : ({
        value: 'perimeter',
        label: 'ops.vcarve.mode.perimeter',
        description: 'ops.vcarve.description.perimeter',
        scope: 'ops.vcarve.scope.perimeter',
        depthLabel: 'opprops.cut.final_depth',
        depthHelp: 'ops.vcarve.depth.perimeter.help',
      } as const);
}
/** Same patch as the former two buttons. No depth/pass/default normalization. */
export function vcarveStrategyPatch(value: 'perimeter' | 'medial') {
  return { fullMedialAxis: value === 'medial' ? true : undefined };
}

export function vcarveDepthPresentation(op: { kind: string; fullMedialAxis?: boolean } | null) {
  const strategy = op?.kind === 'vcarve' ? vcarveStrategy(op.fullMedialAxis) : null;
  return {
    label: strategy?.depthLabel ?? ('opprops.cut.final_depth' as const),
    help: strategy?.depthHelp,
    stepHelp: strategy ? ('ops.vcarve.step.help' as const) : undefined,
  };
}
