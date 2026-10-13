import { expect, it } from 'vitest';
import { cuttingOriginPresentation } from './cutting-origin-presentation';
it('collapses identical recommended origins without repeated badges', () => {
  const view = cuttingOriginPresentation(Array(5).fill('recommendation'));
  expect(view.shared).toBe('recommendation');
  expect(view.show('recommendation')).toBe(false);
});
it('marks manual exceptions without calling them recommended', () => {
  const view = cuttingOriginPresentation(['recommendation', 'manual', 'recommendation']);
  expect(view.shared).toBe('mixed');
  expect(view.show('manual')).toBe(true);
  expect(view.show('recommendation')).toBe(false);
});
it('does not broaden recommendation origin to missing or legacy fields', () => {
  for (const other of [undefined, null, 'legacy_snapshot']) {
    const view = cuttingOriginPresentation(['recommendation', other]);
    expect(view.shared).toBeNull();
    expect(view.show('recommendation')).toBe(true);
    expect(view.show(other)).toBe(false);
  }
});
it('groups fully manual values and handles empty contexts neutrally', () => {
  expect(cuttingOriginPresentation(['manual', 'manual']).shared).toBe('manual');
  expect(cuttingOriginPresentation([]).shared).toBeNull();
});
