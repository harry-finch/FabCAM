import { expect, it } from 'vitest';
import { dimensionState, toolDimensions } from './tool-dimensions';
import { defaultToolForMode } from './tool_mode_defaults';

it('distinguishes unknown, invalid and valid zero free shank without filling source fields', () => {
  expect(dimensionState(undefined)).toBe('unknown');
  expect(dimensionState(null)).toBe('unknown');
  for (const v of [0, -1, NaN, Infinity]) expect(dimensionState(v)).toBe('invalid');
  expect(dimensionState(0, true)).toBe('known');
  const tool = defaultToolForMode('mill', 1);
  const before = JSON.stringify(tool);
  const diagram = toolDimensions(tool);
  expect(JSON.stringify(tool)).toBe(before);
  expect(diagram.holderLength).toBeUndefined();
  expect(Number.isFinite(diagram.overall)).toBe(true);
});
it('uses Rust free-shank semantics: holder starts above flutes plus free shank', () => {
  const base = {
    ...defaultToolForMode('mill', 1),
    fluteLengthMm: 20,
    stickoutLengthMm: 0,
    lengthMm: 40,
  };
  expect(toolDimensions(base).holderBase).toBe(toolDimensions(base).flute);
  const extended = toolDimensions({ ...base, stickoutLengthMm: 10 });
  expect(extended.holderBase / extended.flute).toBe(1.5);
  expect(extended.inconsistent).toBe(false);
  expect(toolDimensions({ ...base, stickoutLengthMm: 25 }).inconsistent).toBe(true);
});
it('handles holder shapes and rejects invalid parts even when their sum is positive', () => {
  const base = defaultToolForMode('mill', 1);
  const d = toolDimensions({
    ...base,
    holder: {
      kind: 'stepped',
      cylinder_diameter_mm: 20,
      cylinder_length_mm: 12,
      cone_top_diameter_mm: 35,
      cone_length_mm: 25,
    },
  });
  expect(d.holderLength).toBe(37);
  expect(d.holderDiameter).toBe(35);
  const invalid = toolDimensions({
    ...base,
    holder: { kind: 'cone', bottom_diameter_mm: -3, top_diameter_mm: 35, length_mm: 30 },
  });
  expect(invalid.holderValid).toBe(false);
  expect(invalid.holderDiameter).toBeNaN();
  expect(invalid.holderPath).not.toMatch(/NaN|Infinity/);
});
