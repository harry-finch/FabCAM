import { describe, it, expect } from 'vitest';
import reference from '../../../../data/feeds-speeds/packs/sienci-longmill.v3.json';
import type { SourcePack } from './feeds-speeds-sources';
import { presetRows, filterPresets, emptyPresetFilters, rangeText } from './preset-library';
function pack(id = 'reference'): SourcePack {
  const p = structuredClone(reference) as unknown as SourcePack;
  p.manifest.identity.id = id;
  p.manifest.name = id === 'reference' ? 'Référence atelier' : id;
  p.dataset.profile_set.profiles = p.dataset.profile_set.profiles.slice(0, 2);
  p.records = p.records.filter((r) =>
    p.dataset.profile_set.profiles.some((v) => v.identity.id === r.profile.id),
  );
  return p;
}
const key = (p: SourcePack) => `${p.manifest.identity.id}@${p.manifest.identity.revision}`;
describe('read-only cutting presets library', () => {
  it('preserves identities, authored values and provenance without mutation or calculation', () => {
    const p = pack();
    p.dataset.profile_set.profiles[0].data = {
      kind: 'chipload_target',
      rpm: 18000,
      plunge_mm_min: 100,
      doc_mm: 1,
      chipload_mm_tooth: 0.025,
    };
    const before = JSON.stringify(p);
    const rows = presetRows([p], new Set([key(p)]), true);
    expect(rows).toHaveLength(2);
    expect(rows[0].profile).toBe(p.dataset.profile_set.profiles[0]);
    expect(rows[0].pack).toBe(p);
    expect(JSON.stringify(p)).toBe(before);
    expect(rows[0].profile.data.kind).toBe('chipload_target');
    expect('feed_mm_min' in rows[0].profile.data).toBe(false);
    expect(new Set(presetRows([p, pack('second')], new Set(), true).map((r) => r.key)).size).toBe(
      4,
    );
  });
  it('combines accent-insensitive search and pack, family, tool and evidence filters', () => {
    const p = pack();
    const first = p.dataset.profile_set.profiles[0];
    first.scope.material = { kind: 'family', id: 'workshop-sheet' };
    first.source.notes = 'Érable software notes';
    const rows = presetRows([p], new Set([key(p)]), true);
    const f = {
      ...emptyPresetFilters(),
      query: 'erable software',
      pack: key(p),
      materialFamily: 'workshop-sheet',
      toolFamily: first.scope.cutting_family,
      state: first.evidence,
    };
    expect(filterPresets(rows, f)).toEqual([rows[0]]);
    expect(filterPresets(rows, { ...f, pack: 'missing' })).toEqual([]);
    expect(filterPresets(rows, { ...f, query: 'not recorded' })).toEqual([]);
  });
  it('does not infer material equivalence and preserves unknown revisioned IDs', () => {
    const p = pack();
    const first = p.dataset.profile_set.profiles[0];
    first.scope.material = {
      kind: 'exact',
      identity: { id: 'unknown-material', revision: 'legacy-7' },
    };
    first.scope.tool = { id: 'unknown-tool', revision: '9' };
    p.dataset.profile_set.material_families = [
      { id: 'recorded-family', members: [{ id: 'unknown-material', revision: 'other' }] },
    ];
    let rows = presetRows([p], new Set([key(p)]), true);
    expect(rows[0].material).toBe('unknown-material @legacy-7');
    expect(rows[0].tool).toBe('unknown-tool @9');
    expect(rows[0].materialFamilies).toEqual([]);
    p.dataset.profile_set.material_families[0].members[0].revision = 'legacy-7';
    rows = presetRows([p], new Set([key(p)]), true);
    expect(rows[0].materialFamilies).toEqual(['recorded-family']);
  });
  it('browses disabled and non-milling sources without enabling them', () => {
    const p = pack();
    const before = JSON.stringify(p);
    const disabled = presetRows([p], new Set(), true);
    expect(filterPresets(disabled, emptyPresetFilters())).toEqual([]);
    expect(
      filterPresets(disabled, { ...emptyPresetFilters(), availability: 'disabled' }),
    ).toHaveLength(2);
    const nonMilling = presetRows([p], new Set([key(p)]), false);
    expect(filterPresets(nonMilling, emptyPresetFilters())).toEqual([]);
    expect(
      filterPresets(nonMilling, { ...emptyPresetFilters(), availability: 'all' }),
    ).toHaveLength(2);
    expect(JSON.stringify(p)).toBe(before);
    p.dataset.profile_set.profiles = [];
    expect(presetRows([p], new Set(), true)).toEqual([]);
  });
  it('retains the newest legacy Sorbonne head and optional fields without re-identification', () => {
    const old = pack('sorbonne-fablab');
    old.manifest.identity.revision = '0000000000000001';
    const latest = structuredClone(old);
    latest.manifest.identity.revision = '0000000000000002';
    latest.dataset.profile_set.profiles[0].scope.flute_count = null;
    latest.dataset.profile_set.profiles[0].scope.tool = null;
    const rows = presetRows([old, latest], new Set([key(latest)]), true);
    expect(rows).toHaveLength(2);
    expect(rows.every((r) => r.pack === latest)).toBe(true);
    expect(rows[0].profile.identity).toEqual(old.dataset.profile_set.profiles[0].identity);
    expect(rows[0].profile.scope.flute_count).toBeNull();
    expect(filterPresets(rows, { ...emptyPresetFilters(), query: 'flutes' })).toHaveLength(2);
  });
  it('keeps null and range dimensions explicit', () => {
    expect(rangeText(null)).toBe('—');
    expect(rangeText({ min: 3, max: 3 })).toBe('3');
    expect(rangeText({ min: 3, max: 6 })).toBe('3–6');
  });
});
