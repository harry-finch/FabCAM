import { describe, expect, it } from 'vitest';
import type { ImportResponse } from '../api/types';
import {
  UNIT_SCALES,
  choiceForScale,
  groupImportWarnings,
  importantWarningCount,
  needsReview,
  parseCustomScale,
  previewSize,
  unitChoicesFor,
  unitStatus,
  unitsAdjustable,
} from './import-units';

function source(patch: Partial<ImportResponse> = {}): ImportResponse {
  return {
    filename: 'part.svg',
    format: 'svg',
    segments: [],
    layers: [],
    objects: [],
    object_meta: [],
    bbox: { min_x: 0, min_y: 0, max_x: 96, max_y: 48 },
    unit_scale: 1,
    warnings: [],
    unit_status: 'assumed',
    unit_source: "width='96px', height='48px'",
    ...patch,
  };
}

describe('ADR-0026 unit interpretation', () => {
  it('offers pixel interpretations only for SVG', () => {
    expect(unitChoicesFor('svg')).toEqual(['mm', 'px96', 'px72', 'inch', 'custom']);
    expect(unitChoicesFor('dxf')).toEqual(['mm', 'inch', 'custom']);
    expect(UNIT_SCALES.px96 * 96).toBeCloseTo(25.4, 12);
    expect(UNIT_SCALES.px72 * 72).toBeCloseTo(25.4, 12);
  });

  it('previews the drawing size for each choice without a second conversion', () => {
    const r = source();
    expect(previewSize(r, UNIT_SCALES.mm)).toEqual({ width: 96, height: 48 });
    expect(previewSize(r, UNIT_SCALES.px96).width).toBeCloseTo(25.4, 9);
    expect(previewSize(r, UNIT_SCALES.inch).width).toBeCloseTo(96 * 25.4, 9);
    // Already imported at 25.4/96: switching back to 1:1 restores 96 mm.
    const explicit = source({
      unit_status: 'explicit',
      unit_scale: UNIT_SCALES.px96,
      bbox: { min_x: 0, min_y: 0, max_x: 25.4, max_y: 12.7 },
    });
    expect(previewSize(explicit, 1).width).toBeCloseTo(96, 9);
    expect(choiceForScale(explicit.unit_scale, 'svg')).toBe('px96');
    expect(choiceForScale(0.5, 'svg')).toBe('custom');
  });

  it('never reinterprets declared, physical or legacy units (no double conversion)', () => {
    expect(unitsAdjustable(source({ unit_status: 'physical' }))).toBe(false);
    expect(unitsAdjustable(source({ unit_status: 'declared', format: 'dxf' }))).toBe(false);
    expect(unitsAdjustable(source({ unit_status: 'unknown' }))).toBe(false);
    expect(unitsAdjustable(source({ unit_status: 'assumed' }))).toBe(true);
    expect(unitsAdjustable(source({ unit_status: 'explicit' }))).toBe(true);
    // A source saved before units existed reads as unknown.
    const legacy = source();
    delete (legacy as Partial<ImportResponse>).unit_status;
    expect(unitStatus(legacy)).toBe('unknown');
  });

  it('accepts only finite custom scales inside the core range', () => {
    expect(parseCustomScale('0.2645833')).toBeCloseTo(0.2645833, 9);
    expect(parseCustomScale('25,4')).toBe(25.4);
    for (const bad of ['', '0', '-1', 'NaN', 'Infinity', 'abc', '1e-7', '2e6']) {
      expect(parseCustomScale(bad), bad).toBeNull();
    }
    expect(parseCustomScale('1e-6')).toBe(1e-6);
    expect(parseCustomScale('1000000')).toBe(1e6);
  });

  it('asks for review only for assumed units or importer warnings', () => {
    expect(needsReview(source())).toBe(true);
    expect(needsReview(source({ unit_status: 'physical' }))).toBe(false);
    expect(needsReview(source({ unit_status: 'declared', warnings: ['x'] }))).toBe(true);
    expect(needsReview(source({ unit_status: 'explicit' }))).toBe(false);
  });
});

describe('ADR-0026 importer warning groups', () => {
  const warnings = [
    "unsupported entity SOLID on layer 'HATCHING'",
    "unsupported entity SOLID on layer 'HATCHING'",
    "unsupported entity SOLID on layer 'HATCHING'",
    "INSERT references missing block 'GHOST'",
    "SVG size is not in physical units (width='96px', height='48px'): 1 SVG user unit (px) was imported as 1 mm.",
    "SPLINE on layer 'CURVES': invalid degree 0; control polygon imported, not the spline",
    '_CAMCFG layer carries 12 bytes of CAM directives (parser stub)',
    'something new from the importer',
  ];

  it('collapses repeats, keeps every message and orders by importance', () => {
    const groups = groupImportWarnings(warnings);
    expect(groups.map((g) => g.kind)).toEqual([
      'units',
      'missing_block',
      'unsupported',
      'geometry',
      'other',
      'info',
    ]);
    const solid = groups.find((g) => g.kind === 'unsupported')!;
    expect(solid).toMatchObject({ count: 3, entity: 'SOLID', layer: 'HATCHING' });
    expect(groups.find((g) => g.kind === 'missing_block')!.block).toBe('GHOST');
    expect(groups.find((g) => g.kind === 'geometry')).toMatchObject({
      entity: 'SPLINE',
      layer: 'CURVES',
    });
    // Nothing is silently dropped.
    expect(groups.reduce((n, g) => n + g.count, 0)).toBe(warnings.length);
    // The informational CAM-directive note does not count as actionable.
    expect(importantWarningCount(warnings)).toBe(warnings.length - 1);
  });
});
