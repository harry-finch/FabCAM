// Source-unit interpretation and importer diagnostics (ADR-0026).
//
// One authoritative scale per source: the importer's `unit_scale`, recorded
// with `unit_status` in the stored ImportResponse. A user choice is an
// explicit re-import with that scale (status `explicit`), so save/reopen and
// source reload reproduce it; nothing here rescales stored geometry.
import type { ImportResponse } from '../api/types';

export type UnitStatus = ImportResponse['unit_status'];
export type UnitChoice = 'mm' | 'px96' | 'px72' | 'inch' | 'custom';

/// mm per source unit for each named interpretation.
export const UNIT_SCALES: Record<Exclude<UnitChoice, 'custom'>, number> = {
  mm: 1,
  px96: 25.4 / 96,
  px72: 25.4 / 72,
  inch: 25.4,
};

/// Same bounds as the core's `EXPLICIT_SCALE_RANGE`.
export const SCALE_MIN = 1e-6;
export const SCALE_MAX = 1e6;

/// Legacy sources (saved before units were recorded) read as `unknown`.
export function unitStatus(r: ImportResponse): UnitStatus {
  return r.unit_status ?? 'unknown';
}

/// Only a size without reliable units may be reinterpreted: declared DXF
/// units and physical SVG sizes are already exact (a factor would convert
/// twice), and a legacy source cannot be re-read with confidence.
export function unitsAdjustable(r: ImportResponse): boolean {
  const s = unitStatus(r);
  return s === 'assumed' || s === 'explicit';
}

/// Choices that make sense for the format (pixels only exist in SVG).
export function unitChoicesFor(format: string): UnitChoice[] {
  return format === 'svg' ? ['mm', 'px96', 'px72', 'inch', 'custom'] : ['mm', 'inch', 'custom'];
}

export function choiceForScale(scale: number, format: string): UnitChoice {
  for (const c of unitChoicesFor(format)) {
    if (c !== 'custom' && Math.abs(UNIT_SCALES[c] - scale) <= 1e-12 * UNIT_SCALES[c]) return c;
  }
  return 'custom';
}

/// Parse a custom "mm per source unit" entry; null when not a finite
/// value inside the accepted range (zero, negative, NaN, ∞, typos).
export function parseCustomScale(text: string): number | null {
  const v = Number(text.trim().replace(',', '.'));
  if (text.trim() === '' || !Number.isFinite(v) || v < SCALE_MIN || v > SCALE_MAX) return null;
  return v;
}

/// Drawing size (mm) the source would have with `scale` mm per unit. Exact
/// for a uniform factor; the confirmed import is still re-run with that
/// scale so curve sampling keeps its 0.01 mm budget.
export function previewSize(r: ImportResponse, scale: number): { width: number; height: number } {
  const k = r.unit_scale > 0 ? scale / r.unit_scale : 1;
  return {
    width: Math.max(0, r.bbox.max_x - r.bbox.min_x) * k,
    height: Math.max(0, r.bbox.max_y - r.bbox.min_y) * k,
  };
}

/// Open the import summary before committing when the size is an
/// assumption or the importer reported anything.
export function needsReview(r: ImportResponse): boolean {
  return unitStatus(r) === 'assumed' || (r.warnings?.length ?? 0) > 0;
}

export type WarningKind = 'units' | 'missing_block' | 'unsupported' | 'geometry' | 'other' | 'info';

export interface WarningGroup {
  kind: WarningKind;
  /// Raw importer text (kept for details and support).
  message: string;
  count: number;
  entity?: string;
  layer?: string;
  block?: string;
}

const ORDER: WarningKind[] = ['units', 'missing_block', 'unsupported', 'geometry', 'other', 'info'];

function classify(message: string): Omit<WarningGroup, 'message' | 'count'> {
  let m: RegExpMatchArray | null;
  if (/^(DXF drawing units are not set|SVG size is not in physical units)/.test(message))
    return { kind: 'units' };
  if ((m = message.match(/^INSERT references missing block '(.*)'$/)))
    return { kind: 'missing_block', block: m[1] };
  if ((m = message.match(/^unsupported entity (\S+) on layer '(.*)'$/)))
    return { kind: 'unsupported', entity: m[1], layer: m[2] };
  if ((m = message.match(/^(SPLINE|MLINE) on layer '([^']*)'/)))
    return { kind: 'geometry', entity: m[1], layer: m[2] };
  if (/^INSERT depth limit/.test(message)) return { kind: 'geometry', entity: 'INSERT' };
  if (/^_CAMCFG layer/.test(message)) return { kind: 'info' };
  return { kind: 'other' };
}

/// Collapse identical importer messages (e.g. one per ignored entity) into
/// counted groups, most important kinds first. Nothing is dropped.
export function groupImportWarnings(warnings: readonly string[]): WarningGroup[] {
  const groups = new Map<string, WarningGroup>();
  for (const message of warnings) {
    const g = groups.get(message);
    if (g) g.count += 1;
    else groups.set(message, { message, count: 1, ...classify(message) });
  }
  return [...groups.values()].sort((a, b) => ORDER.indexOf(a.kind) - ORDER.indexOf(b.kind));
}

/// Warnings the user should act on (everything except informational notes).
export function importantWarningCount(warnings: readonly string[]): number {
  return groupImportWarnings(warnings)
    .filter((g) => g.kind !== 'info')
    .reduce((n, g) => n + g.count, 0);
}
