import type { ImportEntry } from './project-types';
import { hexToCss, resolveAci } from '../canvas/aci-color';

export interface DrawingLayer {
  name: string;
  color: number;
  segment_count: number;
  sources: { id: number; filename: string }[];
}

// Presentation of the existing LayerList union: exact names, positive counts,
// first-seen order/color. Provenance describes that union; it never resolves CAM.
export function drawingLayers(imports: readonly ImportEntry[]): DrawingLayer[] {
  const byName = new Map<string, DrawingLayer>();
  for (const entry of imports) {
    for (const layer of entry.source.layers) {
      if (layer.segment_count <= 0) continue;
      let item = byName.get(layer.name);
      if (item) item.segment_count += layer.segment_count;
      else {
        item = { ...layer, sources: [] };
        byName.set(layer.name, item);
      }
      if (!item.sources.some((source) => source.id === entry.id))
        item.sources.push({ id: entry.id, filename: entry.source.filename });
    }
  }
  return [...byName.values()];
}

// Reuse the renderer's ACI policy; colors here are read-only source metadata.
export function layerSwatch(color: number): string {
  const resolved = resolveAci(color);
  return resolved.kind === 'fixed' ? hexToCss(resolved.hex) : `var(${resolved.token})`;
}
