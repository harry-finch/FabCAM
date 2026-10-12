import { describe, it, expect } from 'vitest';
import { scopeValue } from './feeds-speeds-explanation';
import { parseDatasetCatalog } from './feeds-speeds';
import source from '../../../../data/feeds-speeds/sienci-router-reference.v1.json';
const catalog = parseDatasetCatalog(JSON.stringify(source));
describe('native rejection value presentation', () => {
  it('keeps precise exact and ranged DOC values', () => {
    expect(scopeValue('{"min":2.8,"max":2.8}', catalog, 'Unknown')).toBe('2.8');
    expect(scopeValue('{"min":1.25,"max":2.8}', catalog, 'Unknown')).toBe('1.25–2.8');
    expect(scopeValue('1', catalog, 'Unknown')).toBe('1');
  });
  it('resolves identity and revision without fabricating equivalence', () => {
    const identity = source.machines[0].identity;
    expect(scopeValue(JSON.stringify({ kind: 'exact', identity }), catalog, 'Unknown')).toBe(
      source.machines[0].label,
    );
    expect(
      scopeValue(JSON.stringify({ ...identity, revision: 'other' }), catalog, 'Unknown'),
    ).toContain('(other)');
  });
  it('distinguishes unknown and full-width engagement', () => {
    expect(scopeValue('null', catalog, 'Unknown')).toBe('Unknown');
    expect(scopeValue('{"kind":"known_full_width"}', catalog, 'Unknown')).toBe('Full width');
    expect(scopeValue('{"kind":"nominal","ratio":0.4}', catalog, 'Unknown')).toBe(
      'Nominal ratio 0.4',
    );
  });
});
