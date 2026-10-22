import type { PresetCollection, TestedPreset } from './preset-collections';
import type { MaterialSource } from './material-presentation';
import { normalizeMaterialSearch } from './material-presentation';
/** Display-only projection. Never used to authorize a preset load. */
export function presetDisplay(p: TestedPreset) {
  try {
    const s = JSON.parse(p.context.signature);
    const operation =
      typeof s.operation?.type === 'string' ? s.operation.type : 'Unknown operation';
    const tools: string[] = Array.isArray(s.tools)
      ? s.tools.map((t: { physical_id?: string }) => String(t.physical_id ?? 'Unknown tool'))
      : [];
    return {
      operation,
      tools: [...new Set(tools)].join(', '),
      material: s.material as { id?: unknown; revision?: unknown } | undefined,
    };
  } catch {
    return { operation: 'Unknown operation', tools: 'Unknown tool', material: undefined };
  }
}
export function collectionMaterials(collections: PresetCollection[]): MaterialSource[] {
  return collections.map((c) => ({
    id: c.id,
    label: c.name,
    catalog: {
      materials: c.presets.flatMap((p) => {
        const material = presetDisplay(p).material;
        return material &&
          typeof material.id === 'string' &&
          typeof material.revision === 'string' &&
          material.id === p.context.material_id
          ? [
              {
                identity: { id: material.id, revision: material.revision },
                label: p.context.material_label,
              },
            ]
          : [];
      }),
    },
  }));
}
export interface TestedRow {
  key: string;
  collection: PresetCollection;
  preset: TestedPreset;
  operation: string;
  tools: string;
}
export interface PresetFilters {
  query: string;
  collection: string;
  machine: string;
  tool: string;
  material: string;
  operation: string;
}
export const emptyFilters = (): PresetFilters => ({
  query: '',
  collection: '',
  machine: '',
  tool: '',
  material: '',
  operation: '',
});
export function testedRows(collections: PresetCollection[]): TestedRow[] {
  return collections.flatMap((collection) =>
    collection.presets.map((preset) => ({
      key: JSON.stringify([collection.id, preset.id]),
      collection,
      preset,
      ...presetDisplay(preset),
    })),
  );
}
export function filterTestedRows(rows: TestedRow[], f: PresetFilters): TestedRow[] {
  const words = normalizeMaterialSearch(f.query).split(/\s+/).filter(Boolean);
  return rows.filter(
    (r) =>
      (!f.collection || r.collection.id === f.collection) &&
      (!f.machine || r.preset.context.machine_id === f.machine) &&
      (!f.material || r.preset.context.material_id === f.material) &&
      (!f.operation || r.operation === f.operation) &&
      (!f.tool || r.tools === f.tool) &&
      words.every((word) =>
        normalizeMaterialSearch(
          [
            r.collection.name,
            r.preset.name,
            r.preset.context.machine_id,
            r.preset.context.machine_label,
            r.preset.context.material_id,
            r.preset.context.material_label,
            r.tools,
            r.operation,
          ].join(' '),
        ).includes(word),
      ),
  );
}
export function attestationKey(p: TestedPreset): string {
  return JSON.stringify([p.context, p.parameters, p.test_date]);
}
/** Metadata edits retain the source claim. Changes to values/context/date need a fresh claim. */
export function editTestedPreset(
  original: TestedPreset | null,
  edited: TestedPreset,
  attested: string,
): TestedPreset {
  if (
    (!original || attestationKey(original) !== attestationKey(edited)) &&
    attested !== attestationKey(edited)
  )
    throw new Error('Confirm workshop testing of the edited configuration and values');
  return { ...structuredClone(edited), tested: true };
}
export function importChoice(
  incoming: PresetCollection,
  existing: { data: PresetCollection; token: string } | undefined,
  choice: 'replace' | 'copy' | 'new',
) {
  if (choice === 'replace' && !existing) throw new Error('Collection no longer exists; refresh');
  if (choice === 'new' && existing) throw new Error('Choose Replace or Import independent copy');
  const data = structuredClone(incoming);
  if (choice === 'copy') {
    data.id = crypto.randomUUID();
    data.name = `${data.name.slice(0, 145)} (copy)`;
  }
  return { data, expected: choice === 'replace' ? existing!.token : null };
}

export function parameterSummary(p: TestedPreset['parameters']): string[] {
  const depth =
    p.schedule.kind === 'explicit'
      ? `Depths: ${p.schedule.depths.join(', ')} mm`
      : `Depth per pass: ${p.schedule.step == null ? 'not used' : `${Math.abs(p.schedule.step)} mm`}${p.schedule.finish_step == null ? '' : `; finish ${Math.abs(p.schedule.finish_step)} mm`}`;
  return [
    depth,
    ...(p.pocket_overlap == null ? [] : [`Pocket overlap: ${p.pocket_overlap * 100}%`]),
    ...(p.planar_stepover_mm == null ? [] : [`Surface stepover: ${p.planar_stepover_mm} mm`]),
  ];
}
