import type { TestedPreset } from './preset-collections';

const CONTEXT_MISMATCH =
  'Preset context differs: review machine, material, tool geometry and operation';

/** Match only the authoritative compatibility error, never other safety errors. */
export function isPresetContextMismatch(error: string): boolean {
  return error === CONTEXT_MISMATCH || error === `Error: ${CONTEXT_MISMATCH}`;
}

export interface RecordedContextField {
  path: string;
  value: string;
}
export type RecordedContextReview =
  | { available: true; fields: RecordedContextField[] }
  | { available: false; reason: 'invalid' | 'unsupported' | 'limit' };

function record(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

/** Display only. Never authorizes a load or writes/normalizes the stored signature. */
export function recordedPresetContext(preset: TestedPreset): RecordedContextReview {
  const source = preset.context.signature;
  if (new TextEncoder().encode(source).length > 65536) return { available: false, reason: 'limit' };
  let saved: unknown;
  try {
    saved = JSON.parse(source);
  } catch {
    return { available: false, reason: 'invalid' };
  }
  if (
    !record(saved) ||
    saved.machine !== preset.context.machine_id ||
    !record(saved.machine_setup) ||
    typeof saved.mode !== 'string' ||
    !record(saved.material) ||
    saved.material.id !== preset.context.material_id ||
    typeof saved.material.revision !== 'string' ||
    !record(saved.operation) ||
    typeof saved.operation.type !== 'string' ||
    !record(saved.plunge) ||
    !Array.isArray(saved.tools) ||
    !saved.tools.length ||
    !saved.tools.every(
      (tool) =>
        record(tool) &&
        typeof tool.role === 'string' &&
        typeof tool.physical_id === 'string' &&
        record(tool.geometry),
    )
  )
    return { available: false, reason: 'unsupported' };

  const fields: RecordedContextField[] = [];
  function visit(value: unknown, path: string, depth: number): boolean {
    if (depth > 16 || fields.length >= 2048) return false;
    if (value !== null && typeof value === 'object') {
      const entries = Object.entries(value);
      if (!entries.length) fields.push({ path, value: Array.isArray(value) ? '[]' : '{}' });
      for (const [key, item] of entries) {
        const label = Array.isArray(value) ? `[${Number(key) + 1}]` : key.replaceAll('_', ' ');
        if (!visit(item, path ? `${path} / ${label}` : label, depth + 1)) return false;
      }
      return true;
    }
    // Do not invent a precise recorded rating from an unsafe JS integer/overflow.
    if (
      typeof value === 'number' &&
      (!Number.isFinite(value) || (Number.isInteger(value) && !Number.isSafeInteger(value)))
    )
      return false;
    fields.push({ path, value: Object.is(value, -0) ? '-0' : String(value) });
    return true;
  }
  return visit(saved, '', 0) ? { available: true, fields } : { available: false, reason: 'limit' };
}
