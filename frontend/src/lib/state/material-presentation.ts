/** Presentation only. Nothing in this module establishes cutting compatibility. */
import type { MsgKey } from '../i18n';
import { NORMALIZED_MATERIALS, type LoadedSource } from './feeds-speeds-sources';
type Identity = { id: string; revision: string };
export type MaterialFamily = 'wood' | 'plastic' | 'metal' | 'unknown';
export const materialFamilyKeys: Record<MaterialFamily, MsgKey> = {
  wood: 'mat.wood',
  plastic: 'mat.plastic',
  metal: 'mat.metal',
  unknown: 'mat.unknown',
};
const concepts: Record<string, { key: MsgKey; aliases: string }> = {
  wood: { key: 'mat.wood', aliases: 'wood products bois holz holzwerkstoffe' },
  'wood.mdf': { key: 'mat.mdf', aliases: 'mdf medium mitteldichte faserplatte' },
  'wood.plywood': { key: 'mat.plywood', aliases: 'plywood contreplaqué sperrholz' },
  'wood.plywood.poplar': { key: 'mat.poplar', aliases: 'poplar peuplier pappel' },
  'wood.plywood.birch': { key: 'mat.birch', aliases: 'birch bouleau birke' },
  'wood.hardwood': { key: 'mat.hardwood', aliases: 'hardwood bois dur hartholz' },
  'wood.softwood': { key: 'mat.softwood', aliases: 'softwood bois tendre weichholz' },
  plastic: { key: 'mat.plastic', aliases: 'plastic plastics plastique kunststoff kunststoffe' },
  'plastic.pmma': { key: 'mat.pmma', aliases: 'pmma acrylic acrylique acryl' },
  'plastic.pmma.cast': { key: 'mat.cast', aliases: 'cast coulé gegossen' },
  'plastic.pmma.extruded': { key: 'mat.extruded', aliases: 'extruded extrudé extrudiert' },
  'plastic.pom': { key: 'mat.pom', aliases: 'pom' },
  'plastic.hdpe': { key: 'mat.hdpe', aliases: 'hdpe pehd' },
  'plastic.polycarbonate': { key: 'mat.pc', aliases: 'polycarbonate polycarbonat pc' },
  'plastic.pvc': { key: 'mat.pvc', aliases: 'pvc' },
  metal: { key: 'mat.metal', aliases: 'metal metals métal metall metalle' },
  'metal.aluminium': { key: 'mat.aluminium', aliases: 'aluminium aluminum' },
  'metal.aluminium.6061': { key: 'mat.6061', aliases: '6061' },
  'metal.aluminium.7075': { key: 'mat.7075', aliases: '7075' },
};
export const materialIdentityKey = (id: Identity) => JSON.stringify([id.id, id.revision]);
export function normalizeMaterialSearch(text: string): string {
  return text
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
    .trim();
}
export function materialPresentation(id: Identity) {
  const taxon = id.revision === '1' && id.id.startsWith('material/') ? id.id.slice(9) : '';
  const known = Object.hasOwn(concepts, taxon);
  const broad = id.id === 'sienci-softwood-soft-plywood-mdf' && id.revision === '1';
  const path = known ? taxon.split('.').map((_, i, parts) => parts.slice(0, i + 1).join('.')) : [];
  return {
    family: (known ? taxon.split('.')[0] : broad ? 'wood' : 'unknown') as MaterialFamily,
    broad,
    keys: path.map((p) => concepts[p].key),
    aliases: broad
      ? 'wood products bois holz holzwerkstoffe softwood plywood contreplaqué sperrholz mdf medium'
      : known
        ? path.map((p) => concepts[p].aliases).join(' ')
        : 'unclassified nicht klassifiziert',
  };
}
export function materialDisplay(
  id: Identity,
  fallback: string,
  translate: (key: MsgKey) => string,
): string {
  const { keys } = materialPresentation(id);
  return keys.length ? keys.map(translate).join(' / ') : fallback;
}
export type MaterialOption = {
  key: string;
  identity: Identity;
  labels: string[];
  sources: { key: string; name: string }[];
  family: MaterialFamily;
  broad: boolean;
  search: string;
};
const compare = (a: string, b: string) => (a < b ? -1 : a > b ? 1 : 0);
export function materialOptions(
  sources: LoadedSource[],
  current?: Identity & { label?: string },
): MaterialOption[] {
  const entries = new Map<string, MaterialOption>();
  function add(identity: Identity, label: string, source?: { key: string; name: string }) {
    const key = materialIdentityKey(identity);
    let entry = entries.get(key);
    if (!entry) {
      const p = materialPresentation(identity);
      entry = {
        key,
        identity: { id: identity.id, revision: identity.revision },
        labels: [],
        sources: [],
        family: p.family,
        broad: p.broad,
        search: '',
      };
      entries.set(key, entry);
    }
    if (!entry.labels.includes(label)) entry.labels.push(label);
    if (source && !entry.sources.some((s) => s.key === source.key)) entry.sources.push(source);
  }
  for (const m of NORMALIZED_MATERIALS) add(m.identity, m.label);
  for (const source of sources)
    for (const m of source.catalog.materials)
      add(m.identity, m.label, { key: source.id, name: source.label });
  if (current && !entries.has(materialIdentityKey(current)))
    add(current, current.label || current.id);
  return [...entries.values()]
    .map((e) => {
      e.labels.sort(compare);
      e.sources.sort((a, b) => compare(a.key, b.key));
      e.search = normalizeMaterialSearch(
        [
          e.identity.id,
          e.identity.revision,
          ...e.labels,
          ...e.sources.flatMap((s) => [s.key, s.name]),
          materialPresentation(e.identity).aliases,
        ].join(' '),
      );
      return e;
    })
    .sort((a, b) => compare(a.labels[0], b.labels[0]) || compare(a.key, b.key));
}
export function filterMaterials(
  options: MaterialOption[],
  query: string,
  family = '',
  source = '',
) {
  const words = normalizeMaterialSearch(query).split(/\s+/).filter(Boolean);
  return options.filter(
    (e) =>
      (!family || e.family === family) &&
      (!source || e.sources.some((s) => s.key === source)) &&
      words.every((w) => e.search.includes(w)),
  );
}
