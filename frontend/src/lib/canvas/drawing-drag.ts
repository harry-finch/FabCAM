import type { ImportEntry } from '../state/project-types';

/// Combined imports concatenate segments in source order. Never map synthetic
/// stock or evaluated CAD geometry through this raw-import index.
export function importAtSegment(entries: readonly ImportEntry[], index: number) {
  if (index < 0) return null;
  for (const entry of entries) {
    if (index < entry.source.segments.length) return entry;
    index -= entry.source.segments.length;
  }
  return null;
}

/// Transient gesture: no project mutation until pointer-up. Threshold is screen
/// pixels, translation is mm. Freeze scale at grab so view changes cannot jump.
export class DrawingDrag {
  dx = 0;
  dy = 0;
  active = false;
  constructor(
    readonly pointerId: number,
    readonly entry: ImportEntry,
    readonly x: number,
    readonly y: number,
    readonly pixelsPerMm: number,
  ) {}
  move(x: number, y: number) {
    if (!Number.isFinite(this.pixelsPerMm) || this.pixelsPerMm <= 0) return;
    if (!this.active && Math.hypot(x - this.x, y - this.y) < 4) return;
    this.active = true;
    this.dx = (x - this.x) / this.pixelsPerMm;
    this.dy = -(y - this.y) / this.pixelsPerMm;
  }
  get translation() {
    return {
      x: this.entry.fileTransform.translate.x + this.dx,
      y: this.entry.fileTransform.translate.y + this.dy,
    };
  }
}
