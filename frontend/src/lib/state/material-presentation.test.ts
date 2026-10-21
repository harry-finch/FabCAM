import { describe, it, expect } from 'vitest';
import {
  materialOptions,
  filterMaterials,
  materialPresentation,
  materialDisplay,
  materialIdentityKey,
} from './material-presentation';
import type { LoadedSource } from './feeds-speeds-sources';
function source(
  id: string,
  materials: { identity: { id: string; revision: string }; label: string }[],
): LoadedSource {
  return {
    id,
    label: `Workshop ${id}`,
    contents: '',
    catalog: { description: 'Synthetic test catalog', materials, machines: [], tools: [] },
  };
}
const item = (id: string, label = 'Same name', revision = '1') => ({
  identity: { id, revision },
  label,
});
describe('MAT-1 presentation does not establish equivalence', () => {
  it('retains all normalized identities and distinguishes grade/condition revisions', () => {
    const options = materialOptions([]);
    expect(options).toHaveLength(19);
    expect(new Set(options.map((o) => o.key)).size).toBe(19);
    expect(
      filterMaterials(options, 'acrylique')
        .map((o) => o.identity.id)
        .sort(),
    ).toEqual([
      'material/plastic.pmma',
      'material/plastic.pmma.cast',
      'material/plastic.pmma.extruded',
    ]);
    expect(filterMaterials(options, 'sperrholz')).toHaveLength(3);
    expect(filterMaterials(options, 'bouleau')).toHaveLength(1);
    expect(materialPresentation({ id: 'material/wood.mdf', revision: '2' }).family).toBe('unknown');
    expect(materialPresentation({ id: 'material/wood.fiction', revision: '1' }).family).toBe(
      'unknown',
    );
  });
  it('preserves broad source categories, without fabricating constituent identities', () => {
    const broad = item(
      'sienci-softwood-soft-plywood-mdf',
      'Source softwood / soft plywood / MDF category',
    );
    const options = materialOptions([source('public@1', [broad])]);
    expect(options).toHaveLength(20);
    expect(filterMaterials(options, 'medium')).toHaveLength(2);
    expect(filterMaterials(options, 'mdf', '', 'public@1')[0].identity).toEqual(broad.identity);
    expect(filterMaterials(options, 'mdf', '', 'public@1')[0].broad).toBe(true);
  });
  it('keeps same-name private identities separate and source-order selection deterministic', () => {
    const a = source('a@1', [item('private-a'), item('shared', 'Z name')]);
    const b = source('b@7', [item('private-b'), item('shared', 'A name')]);
    const before = JSON.stringify([a, b]);
    const options = materialOptions([a, b]);
    expect(options).toEqual(materialOptions([b, a]));
    expect(filterMaterials(options, 'same name', 'unknown')).toHaveLength(2);
    expect(filterMaterials(options, 'private-b', '', 'b@7')[0].identity.id).toBe('private-b');
    expect(options.find((o) => o.identity.id === 'shared')?.sources).toHaveLength(2);
    expect(JSON.stringify([a, b])).toBe(before);
    expect(materialIdentityKey({ id: 'a', revision: 'b@c' })).not.toBe(
      materialIdentityKey({ id: 'a@b', revision: 'c' }),
    );
  });
  it('searches original labels and IDs without changing unknown selections or label bytes', () => {
    const current = { id: 'Échantillon/異なる', revision: 'legacy-7', label: 'Pièce spéciale' };
    const options = materialOptions([], current);
    expect(filterMaterials(options, 'piece speciale', 'unknown')[0].identity).toEqual({
      id: current.id,
      revision: current.revision,
    });
    expect(filterMaterials(options, 'echantillon')[0].identity.id).toBe(current.id);
    expect(filterMaterials(options, 'no such result')).toEqual([]);
    expect(JSON.parse(JSON.stringify(current))).toEqual(current);
    expect(materialDisplay(current, current.label, (k) => k)).toBe(current.label);
  });
  it('handles large imported catalogs and long labels without dropping source occurrences', () => {
    const entries = Array.from({ length: 1500 }, (_, i) =>
      item(`fixture-${i}`, `Long label ${'x'.repeat(200)} ${i}`),
    );
    const options = materialOptions([source('large@1', entries)]);
    expect(filterMaterials(options, '', 'unknown', 'large@1')).toHaveLength(1500);
    expect(filterMaterials(options, 'fixture-1499', '', 'large@1')[0].identity.id).toBe(
      'fixture-1499',
    );
  });
});
