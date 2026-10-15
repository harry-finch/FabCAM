import type { OpEntry, SourceCombine } from './op_types';
import type { ImportEntry } from './project-types';

export type GeometrySourceMode = 'all' | 'layers' | 'objects';
export const sourceModeLabels = {
  all: 'opprops.source.mode.all',
  layers: 'opprops.source.mode.layer',
  objects: 'opprops.source.mode.objects',
} as const;
type SourceFields = Pick<OpEntry, 'sourceLayers' | 'sourceObjects'>;
export type SourceMemory = { layers?: string[]; objects?: number[] };

// Mirrors buildSource's precedence, including its legacy empty-array fallback.
// This describes the authored selector, not an independently resolved geometry set.
export function effectiveSourceMode(op: SourceFields): GeometrySourceMode {
  if (op.sourceObjects?.length) return 'objects';
  return op.sourceLayers?.length ? 'layers' : 'all';
}

export function sourceSignature(op: SourceFields): string {
  return JSON.stringify([op.sourceLayers, op.sourceObjects]);
}

// A mode without a binding is an editor draft. Do not write an empty selector
// (which buildSource interprets as All) or erase the current authored binding.
export function sourceModePatch(
  mode: GeometrySourceMode,
  memory: SourceMemory,
): SourceFields | null {
  if (mode === 'all') return { sourceLayers: null, sourceObjects: undefined };
  if (mode === 'layers' && memory.layers?.length)
    return { sourceLayers: [...memory.layers], sourceObjects: undefined };
  if (mode === 'objects' && memory.objects?.length)
    return { sourceLayers: null, sourceObjects: [...memory.objects] };
  return null;
}

export const combineChoices = [
  'auto',
  'union',
  'difference',
  'intersection',
  'xor',
  'none',
] as const;
export const combineHelp = {
  auto: 'opprops.combine.auto.help',
  union: 'opprops.combine.union.help',
  difference: 'opprops.combine.difference.help',
  intersection: 'opprops.combine.intersection.help',
  xor: 'opprops.combine.xor.help',
  none: 'opprops.combine.none.help',
} as const satisfies Record<SourceCombine, string>;

export function contourHelp(op: OpEntry) {
  if (op.kind === 'pocket' && op.frameShape) return 'opprops.geometry.frame.help';
  if (op.kind === 'drill') return 'opprops.geometry.drill.help';
  if (op.kind !== 'pocket' && op.kind !== 'vcarve') return 'opprops.geometry.paths.help';
  const mode = effectiveSourceMode(op) === 'all' ? 'auto' : (op.sourceCombine ?? 'auto');
  if (mode === 'auto' && op.kind === 'pocket' && op.pocketStrategy !== 'halfpipe')
    return 'opprops.geometry.pocket_auto.help';
  if (mode === 'difference')
    return effectiveSourceMode(op) === 'objects'
      ? 'opprops.geometry.difference_objects.help'
      : 'opprops.geometry.difference_layers.help';
  return combineHelp[mode];
}

export function canInterpretContours(op: OpEntry): boolean {
  return (
    (op.kind === 'vcarve' || (op.kind === 'pocket' && !op.frameShape)) &&
    effectiveSourceMode(op) !== 'all'
  );
}

// Provenance only: same-named layers are merged by combineImports, never
// scoped by the filename displayed here. SVG groups are not imported layers.
export function layerProvenance(name: string, imports: readonly ImportEntry[]) {
  const drawings = imports
    .filter((entry) =>
      entry.source.layers.some((layer) => layer.name === name && layer.segment_count > 0),
    )
    .map((entry) => ({ filename: entry.source.filename, format: entry.source.format }));
  return {
    name,
    drawings,
    syntheticSvg:
      name === 'svg' && drawings.length > 0 && drawings.every((d) => d.format === 'svg'),
  };
}
