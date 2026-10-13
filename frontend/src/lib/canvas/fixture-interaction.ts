import { FixtureGesture } from './fixture-gesture';
import type { ProjectState } from '../state/project.svelte';
import type { Fixture } from '../state/project-types';
import type { UserView } from './viewport';
export class FixtureInteraction {
  private gesture: FixtureGesture | null = null;
  private startPixel = { x: 0, y: 0 };
  private moving = false;
  private world = { x: 0, y: 0 };
  private units = { x: 1, y: -1 };
  constructor(
    private project: ProjectState,
    private canvas: () => HTMLCanvasElement,
    private toData: (x: number, y: number) => { x: number; y: number } | null,
    private activate: () => void,
  ) {}
  select(id: number, event: PointerEvent, x: number, y: number) {
    const selected = this.project.sel.selectedFixtureId === id;
    this.project.selectFixture(id);
    this.project.sel.selectedTextLayerId = null;
    this.activate();
    if (selected && event.pointerType !== 'touch') this.start(id, event, x, y);
  }
  start(id: number, event: PointerEvent, x: number, y: number) {
    const point = this.toData(x, y);
    if (!point) return;
    this.gesture = new FixtureGesture(
      id,
      event.pointerId,
      point,
      this.project.target(),
      this.project.history,
    );
    this.startPixel = { x: event.clientX, y: event.clientY };
    this.world = point;
    const next = this.toData(x + 1, y + 1)!;
    this.units = { x: next.x - point.x, y: next.y - point.y };
    this.moving = false;
    this.canvas().setPointerCapture(event.pointerId);
    this.canvas().style.cursor = 'grab';
  }
  move(event: PointerEvent): boolean {
    if (this.gesture?.pointerId !== event.pointerId) return false;
    const dx = event.clientX - this.startPixel.x,
      dy = event.clientY - this.startPixel.y;
    // Freeze the grab transform: stale-feedback/layout changes must not move XY.
    this.moving ||= Math.hypot(dx, dy) >= 3;
    const point = { x: this.world.x + dx * this.units.x, y: this.world.y + dy * this.units.y };
    if (this.moving && point) this.gesture.move(point);
    this.canvas().style.cursor = this.moving ? 'grabbing' : 'grab';
    return true;
  }
  end(event: PointerEvent): boolean {
    if (this.gesture?.pointerId !== event.pointerId) return false;
    this.finish(event.type === 'pointercancel' || event.type === 'lostpointercapture');
    return true;
  }
  cancel() {
    this.finish(true);
  }
  private finish(cancel: boolean) {
    const gesture = this.gesture;
    if (!gesture) return;
    this.gesture = null;
    gesture.finish(cancel);
    const canvas = this.canvas();
    canvas.style.cursor = 'default';
    if (canvas.hasPointerCapture(gesture.pointerId))
      canvas.releasePointerCapture(gesture.pointerId);
  }
}
/** Frame the fixture's declared bounds, not an invented collision point. */
export function frameFixture(
  f: Fixture,
  base: { scale: number; offX: number; offY: number },
  width: number,
  height: number,
): UserView {
  const xs =
    f.kind.shape === 'polygon'
      ? f.kind.vertices.map((p) => p[0])
      : f.kind.shape === 'box'
        ? [-f.kind.width / 2, f.kind.width / 2]
        : [-f.kind.radius, f.kind.radius];
  const ys =
    f.kind.shape === 'polygon'
      ? f.kind.vertices.map((p) => p[1])
      : f.kind.shape === 'box'
        ? [-f.kind.depth / 2, f.kind.depth / 2]
        : [-f.kind.radius, f.kind.radius];
  const minX = Math.min(...xs),
    maxX = Math.max(...xs),
    minY = Math.min(...ys),
    maxY = Math.max(...ys);
  const scale = Math.min(width, height) / Math.max(5 * Math.max(maxX - minX, maxY - minY), 100);
  return {
    zoom: scale / base.scale,
    panX: width / 2 - (f.origin[0] + (minX + maxX) / 2) * scale - base.offX,
    panY: height / 2 + (f.origin[1] + (minY + maxY) / 2) * scale - base.offY,
  };
}
