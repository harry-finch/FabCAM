import { it, expect } from 'vitest';
import { exportSafetyProblem } from './generate-palette-policy';
const settings = { blockOnCriticalSimWarnings: true, blockOnWorkAreaViolation: true };
const clean = { stale: false, critical: 0, backCritical: 0, workArea: 0, backWorkArea: 0 };
it.each(['front', 'back'] as const)('blocks critical and envelope warnings on %s', (side) => {
  const status = {
    ...clean,
    ...(side === 'front' ? { critical: 3, workArea: 2 } : { backCritical: 3, backWorkArea: 2 }),
  };
  expect(exportSafetyProblem(status, side, settings)).toEqual({
    key: 'genbar.error.block_critical',
    count: 3,
  });
  expect(
    exportSafetyProblem(status, side, { ...settings, blockOnCriticalSimWarnings: false }),
  ).toEqual({ key: 'genbar.error.block_work_area', count: 2 });
});
it('keeps front aggregate sim+pipeline independent of the back program', () => {
  expect(exportSafetyProblem({ ...clean, critical: 2 }, 'back', settings)).toBeUndefined();
  expect(exportSafetyProblem({ ...clean, critical: 2 }, 'front', settings)?.count).toBe(2);
});
it('honours existing explicit safety preferences', () =>
  expect(
    exportSafetyProblem({ ...clean, critical: 2, workArea: 1 }, 'front', {
      blockOnCriticalSimWarnings: false,
      blockOnWorkAreaViolation: false,
    }),
  ).toBeUndefined());

it.each(['front', 'back'] as const)('fixture refusal cannot be disabled on %s', (side) => {
  expect(
    exportSafetyProblem({ ...clean, fixture: 1, backFixture: 2 }, side, {
      blockOnCriticalSimWarnings: false,
      blockOnWorkAreaViolation: false,
    }),
  ).toEqual({ key: 'genbar.error.block_critical', count: side === 'front' ? 1 : 2 });
});
