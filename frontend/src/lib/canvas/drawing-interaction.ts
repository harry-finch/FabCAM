import type { ProjectState } from '../state/project.svelte';
import type { Segment } from '../api/types';
import { applyFileTransform } from '../state/file-transform';
import { drawSegment } from './render/segment';
import type { ProjectFn } from './render/types';
import { DrawingDrag, importAtSegment } from './drawing-drag';

/** Owns transient file movement. Rendering is an overlay, not mutable project
 * geometry. Commit uses the existing stock/marker/history transform command. */
export class DrawingInteraction {
  private gesture: DrawingDrag | null = null;
  private preview: Segment[] = [];
  private imports: ProjectState['data']['imports'] | null = null;
  private stock: ProjectState['data']['stock'] | null = null;
  constructor(
    private project: ProjectState,
    private canvas: () => HTMLCanvasElement,
    private repaint: () => void,
  ) {}
  get pending() {
    return this.gesture !== null;
  }
  cancel(pointerId?: number) {
    const id = this.gesture?.pointerId;
    if (pointerId !== undefined && id !== pointerId) return;
    this.gesture = null;
    this.preview = [];
    this.imports = null;
    this.stock = null;
    const canvas = this.canvas();
    if (!canvas) return;
    canvas.style.cursor = 'default';
    if (id !== undefined && canvas.hasPointerCapture(id)) canvas.releasePointerCapture(id);
    this.repaint();
  }
  start(
    e: PointerEvent,
    index: number | null,
    cx: number,
    cy: number,
    scale: number,
    additive: boolean,
  ) {
    const p = this.project;
    if (
      e.pointerType !== 'mouse' ||
      e.button !== 0 ||
      e.shiftKey ||
      e.ctrlKey ||
      e.metaKey ||
      additive ||
      p.data.drawing ||
      p.data.cornerRelief ||
      index === null ||
      !Number.isFinite(scale) ||
      scale <= 0
    )
      return false;
    const object = p.geometryView?.objects?.[index] ?? 0;
    const entry = importAtSegment(p.data.imports, index);
    if (!entry || !object || !p.sel.selectedObjects.has(object)) return false;
    this.gesture = new DrawingDrag(e.pointerId, entry, cx, cy, scale);
    this.imports = p.data.imports;
    this.stock = p.data.stock;
    this.preview = applyFileTransform(entry.source, entry.fileTransform).segments;
    this.canvas().setPointerCapture(e.pointerId);
    this.canvas().style.cursor = 'grab';
    e.preventDefault();
    return true;
  }
  move(e: PointerEvent, cx: number, cy: number) {
    if (this.gesture?.pointerId !== e.pointerId) return false;
    this.gesture.move(cx, cy);
    this.canvas().style.cursor = this.gesture.active ? 'grabbing' : 'grab';
    this.repaint();
    return true;
  }
  end(e: PointerEvent): { tap: boolean; x: number; y: number } | null {
    if (this.gesture?.pointerId !== e.pointerId) return null;
    const g = this.gesture,
      p = this.project;
    const unchanged =
      p.data.imports === this.imports &&
      p.data.stock === this.stock &&
      !p.data.drawing &&
      !p.data.cornerRelief;
    const rect = this.canvas().getBoundingClientRect();
    g.move(e.clientX - rect.left, e.clientY - rect.top);
    this.cancel();
    if (e.type !== 'pointercancel' && unchanged && g.active && (g.dx || g.dy))
      p.patchFileTransformForImport(g.entry.id, { translate: g.translation });
    return { tap: e.type !== 'pointercancel' && unchanged && !g.active, x: g.x, y: g.y };
  }
  paint(ctx: CanvasRenderingContext2D, project: ProjectFn, accent: string) {
    if (!this.gesture?.active) return;
    const { dx, dy } = this.gesture;
    ctx.save();
    ctx.strokeStyle = accent;
    ctx.lineWidth = 2;
    ctx.setLineDash([6, 3]);
    for (const seg of this.preview)
      if (this.project.data.visibleLayers.has(seg.layer))
        drawSegment(ctx, seg, (x, y) => project(x + dx, y + dy));
    ctx.restore();
  }
}
