import type { DatasetCatalog } from './feeds-speeds';

/** Presentation only: matching and rejection criteria remain native authority. */
export function scopeValue(raw: string, catalog: DatasetCatalog, unknown: string): string {
  function format(value: unknown): string {
    if (value == null) return unknown;
    if (typeof value === 'number') return String(value);
    if (typeof value === 'string') return value.replaceAll('_', ' ');
    if (typeof value !== 'object') return String(value);
    const v = value as Record<string, unknown>;
    if (typeof v.id === 'string' && typeof v.revision === 'string') {
      return (
        [...catalog.machines, ...catalog.materials, ...catalog.tools].find(
          (c) => c.identity.id === v.id && c.identity.revision === v.revision,
        )?.label ?? `${v.id} (${v.revision})`
      );
    }
    if (typeof v.min === 'number' && typeof v.max === 'number')
      return v.min === v.max ? String(v.min) : `${v.min}–${v.max}`;
    if (v.identity) return format(v.identity);
    if (v.kind === 'family') return `Family: ${String(v.id)}`;
    if (v.kind === 'known_full_width' || v.kind === 'full_width') return 'Full width';
    if (v.ratio != null) return `Nominal ratio ${format(v.ratio)}`;
    return v.kind ? format(v.kind) : raw;
  }
  try {
    return format(JSON.parse(raw));
  } catch {
    return raw;
  }
}
