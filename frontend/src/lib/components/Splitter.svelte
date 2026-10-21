<script lang="ts">
  /// Drag handle for resizing two adjacent panes.
  ///
  /// Sits *between* the panes in a CSS grid / flex column. The handle is
  /// a 4 px-wide hover-targeted gutter; clicking and dragging fires
  /// `onResize(delta)` with the cursor delta in the active axis. The
  /// parent owns the pane size (typically a `--var-px` CSS custom prop)
  /// and is responsible for clamping. We only report cursor motion in
  /// the same client-space the size lives in, so the parent's math is
  /// trivial: `size += delta` (or `size -= delta` when the splitter sits
  /// to the LEFT/ABOVE of the resizable pane).
  ///
  /// Pointer capture is taken on pointerdown so the drag continues past
  /// the gutter (cursor can leave by 50+ px on fast moves without losing
  /// the grab). Double-click invokes `onReset` so the user can recover
  /// the default layout without hunting through settings.

  interface Props {
    direction: 'horizontal' | 'vertical';
    onResize: (delta: number) => void;
    onReset?: () => void;
    title?: string;
    /// Optional controlled position enables keyboard resizing and accessible bounds.
    value?: number;
    min?: number;
    max?: number;
  }
  let { direction, onResize, onReset, title, value, min = 0, max = 100 }: Props = $props();

  let dragging = $state(false);
  let last = 0;

  function onPointerDown(e: PointerEvent) {
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    last = direction === 'horizontal' ? e.clientX : e.clientY;
    dragging = true;
    e.preventDefault();
  }
  function onPointerMove(e: PointerEvent) {
    if (!dragging) return;
    const cur = direction === 'horizontal' ? e.clientX : e.clientY;
    const delta = cur - last;
    if (delta !== 0) {
      last = cur;
      onResize(delta);
    }
  }
  function onPointerUp(e: PointerEvent) {
    if (!dragging) return;
    dragging = false;
    try {
      (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId);
    } catch {}
  }
  function onKeyDown(e: KeyboardEvent) {
    if (value === undefined) return;
    const decrease = direction === 'horizontal' ? 'ArrowLeft' : 'ArrowUp';
    const increase = direction === 'horizontal' ? 'ArrowRight' : 'ArrowDown';
    const step = e.shiftKey ? 40 : 10;
    const next =
      e.key === decrease
        ? value - step
        : e.key === increase
          ? value + step
          : e.key === 'Home'
            ? min
            : e.key === 'End'
              ? max
              : undefined;
    if (next === undefined) return;
    e.preventDefault();
    e.stopPropagation();
    onResize(Math.max(min, Math.min(max, next)) - value);
  }
  function onDblClick() {
    onReset?.();
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions (A controlled WAI-ARIA window separator is intentionally focusable and arrow-key adjustable.) -->
<div
  class="splitter"
  class:horizontal={direction === 'horizontal'}
  class:vertical={direction === 'vertical'}
  class:dragging
  role="separator"
  tabindex={value === undefined ? undefined : 0}
  aria-label={title}
  aria-valuenow={value === undefined ? undefined : Math.round(value)}
  aria-valuemin={value === undefined ? undefined : Math.round(min)}
  aria-valuemax={value === undefined ? undefined : Math.round(max)}
  aria-valuetext={value === undefined ? undefined : `${Math.round(value)} px`}
  onkeydown={onKeyDown}
  aria-orientation={direction === 'horizontal' ? 'vertical' : 'horizontal'}
  title={title ?? (onReset ? 'Drag to resize · double-click to reset' : 'Drag to resize')}
  onpointerdown={onPointerDown}
  onpointermove={onPointerMove}
  onpointerup={onPointerUp}
  onpointercancel={onPointerUp}
  ondblclick={onDblClick}
></div>

<style>
  /* Visible 2-px seam between two panes with a 10-px transparent overlay
     widening the hit-target to ~24 px effective grab area. Brightens to
     accent on hover/drag so the affordance is obvious. The old 4-px slab
     was a hostile target on trackpads and touch (WCAG recommends ≥24×24). */
  .splitter {
    background: var(--border);
    transition: background 80ms;
    flex-shrink: 0;
    user-select: none;
    touch-action: none;
    position: relative;
  }
  .splitter::after {
    /* Invisible overlay extending the pointer-hit area without bloating
       the visual seam. inset chosen so the hit zone is ~10 px wide
       (5 px on each side of the 2-px band), within the column-resize
       cursor's tolerance. */
    content: '';
    position: absolute;
    inset: 0;
  }
  .splitter.horizontal {
    width: 2px;
    cursor: col-resize;
  }
  .splitter.horizontal::after {
    left: -5px;
    right: -5px;
  }
  .splitter.vertical {
    height: 2px;
    cursor: row-resize;
  }
  .splitter.vertical::after {
    top: -5px;
    bottom: -5px;
  }
  .splitter:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 1px;
    z-index: 2;
  }
  .splitter:hover,
  .splitter.dragging {
    background: var(--accent);
  }
</style>
