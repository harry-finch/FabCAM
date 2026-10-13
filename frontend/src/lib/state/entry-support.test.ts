import { describe, it, expect } from 'vitest';
import { shopbotEntrySupport } from './entry-support';
import { buildOpEntry } from './op_defaults';
import type { MachineSettings } from './project-types';
const machine = {
  gcodeDialect: 'cps',
  cpsPost: { source: 'bundled', bundledId: 'shopbot', properties: {} },
} as MachineSettings;
const makeOp = () =>
  buildOpEntry('profile', {
    nextId: 1,
    tools: [],
    reliefSources: [],
    selectionIds: [],
    objectMeta: [],
  });
describe('ShopBot entry advisory preserves authored data', () => {
  it('only classifies the active bundled ShopBot post', () => {
    const op = makeOp();
    expect(shopbotEntrySupport(op, machine, 'linuxcnc')).toBe('direct');
    expect(shopbotEntrySupport(op, { ...machine, gcodeDialect: 'grbl' }, 'cps')).toBeNull();
    expect(
      shopbotEntrySupport(
        op,
        { ...machine, cpsPost: { source: 'file', filename: 'shopbot.cps', properties: {} } },
        'cps',
      ),
    ).toBeNull();
  });
  it('exposes unqualified combinations without changing values', () => {
    const op = makeOp();
    if (op.kind !== 'profile') throw new Error('fixture');
    op.plunge = { kind: 'ramp', angleDeg: 3 };
    expect(shopbotEntrySupport(op, machine, '')).toBe('ramp');
    op.tabMode = { kind: 'auto', count: 2 };
    expect(shopbotEntrySupport(op, machine, '')).toBe('tabs');
    op.tabMode = { kind: 'off' };
    op.leadInKind = 'arc';
    expect(shopbotEntrySupport(op, machine, '')).toBe('leads');
    op.plunge = { kind: 'helix', angleDeg: 3, radiusMm: 5 };
    const before = JSON.stringify(op);
    expect(shopbotEntrySupport(op, machine, '')).toBe('helix');
    expect(JSON.stringify(op)).toBe(before);
  });
  it('explains translated origins and unsupported work systems', () => {
    const op = makeOp();
    expect(shopbotEntrySupport(op, machine, '', { x_mm: 5, y_mm: 0, z_mm: 0, wcs: 'G54' })).toBe(
      'direct',
    );
    expect(shopbotEntrySupport(op, machine, '', { x_mm: 0, y_mm: 0, z_mm: 0, wcs: 'G55' })).toBe(
      'origin',
    );
  });
});
