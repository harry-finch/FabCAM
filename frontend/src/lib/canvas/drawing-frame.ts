import { computeFootprint } from '../sim/footprint';
import type { ProjectState } from '../state/project.svelte';
import type { BBox } from '../api/types';
import type { ImportEntry } from '../state/project-types';

/** Preserve the camera through placement changes: otherwise auto-fit follows
 * the drawing and makes fixed stock appear to move. Explicit Fit resets it. */
export class DrawingFrame {
  private epoch = -1;
  private sourceBoundsKey = '';
  private box: BBox | null = null;
  reset() {
    this.box = null;
  }
  fit(epoch: number, entries: readonly ImportEntry[], box: BBox, stock?: BBox): BBox {
    // History clones imports. Compare stable source descriptors, not object
    // references; transforms must not cause a new auto-fit after Undo/Redo.
    const key = JSON.stringify(
      entries.map((e) => [e.id, e.source.filename, e.source.bbox, e.source.segments?.length]),
    );
    if (!this.box || epoch !== this.epoch || key !== this.sourceBoundsKey) {
      this.epoch = epoch;
      this.sourceBoundsKey = key;
      this.box = stock
        ? {
            min_x: Math.min(box.min_x, stock.min_x),
            min_y: Math.min(box.min_y, stock.min_y),
            max_x: Math.max(box.max_x, stock.max_x),
            max_y: Math.max(box.max_y, stock.max_y),
          }
        : box;
    }
    return this.box;
  }
}

export function drawingFit(frame: DrawingFrame, p: ProjectState, box: BBox) {
  const fp = computeFootprint(p.stockSizingImport, p.data.stock, p.data.machine.workArea);
  const stock =
    p.data.stock.mode === 'manual'
      ? { min_x: fp.minX, min_y: fp.minY, max_x: fp.maxX, max_y: fp.maxY }
      : undefined;
  return frame.fit(p.documentEpoch, p.data.imports, box, stock);
}
