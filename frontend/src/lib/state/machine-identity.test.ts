import { describe, expect, it } from 'vitest';
import { machineIdentity } from './machine-identity';
describe('persisted machine identity presentation', () => {
  it.each(['ShopBot', 'The very long full machine identity with no validation metadata'])(
    'preserves ordinary names without inventing status: %s',
    (name) => {
      expect(machineIdentity(name, 'Unnamed')).toEqual({
        fullName: name,
        shortName: name,
        limitation: '',
      });
    },
  );
  it('separates the literal software limitation from a fixture prefix', () => {
    expect(machineIdentity('FS3.9 SOFTWARE ONLY — ShopBot post', 'Unnamed')).toEqual({
      fullName: 'FS3.9 SOFTWARE ONLY — ShopBot post',
      shortName: 'ShopBot post',
      limitation: 'SOFTWARE ONLY',
    });
  });
  it.each([
    'ShopBot — unvalidated',
    'ShopBot — NOT FOR MACHINING',
    'CNC — non validée',
    'CNC — nicht validiert',
  ])('keeps the stated limitation visible: %s', (name) => {
    const view = machineIdentity(name, 'Unnamed');
    expect(view.fullName).toBe(name);
    expect(view.limitation.length).toBeGreaterThan(0);
    expect(machineIdentity(JSON.parse(JSON.stringify({ name })).name, 'Unnamed')).toEqual(view);
  });
  it('handles missing and limitation-only names without asserting validation', () => {
    expect(machineIdentity('', 'Unnamed').shortName).toBe('Unnamed');
    expect(machineIdentity('SOFTWARE ONLY', 'Unnamed').limitation).toBe('SOFTWARE ONLY');
  });
});
