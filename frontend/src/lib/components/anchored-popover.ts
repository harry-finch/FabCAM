import { handleModalKey } from './modal_behavior';

type Rect = { left: number; right: number; top: number; bottom: number };
export function placePopover(
  anchor: Rect,
  height: number,
  viewport: { width: number; height: number },
) {
  const margin = 8,
    gap = 4;
  const width = Math.max(1, Math.min(440, viewport.width - margin * 2));
  const below = Math.max(0, viewport.height - anchor.bottom - gap - margin);
  const above = Math.max(0, anchor.top - gap - margin);
  const side = below >= Math.min(height, 400) || below >= above ? 'below' : 'above';
  const room = side === 'below' ? below : above;
  const centered = room < 180;
  const maxHeight = Math.max(1, Math.min(480, centered ? viewport.height - margin * 2 : room));
  const actualHeight = Math.min(height, maxHeight);
  const top = centered
    ? (viewport.height - actualHeight) / 2
    : side === 'below'
      ? anchor.bottom + gap
      : anchor.top - gap - actualHeight;
  return {
    width,
    maxHeight,
    left: Math.max(margin, Math.min(anchor.right - width, viewport.width - width - margin)),
    top: Math.max(margin, Math.min(top, viewport.height - actualHeight - margin)),
    placement: centered ? 'center' : side,
  };
}

/** Top-layer positioning escapes inspector/drawer clipping while retaining modal DOM ownership. */
export function attachAnchoredPopover(
  node: HTMLElement,
  anchor: HTMLElement,
  onClose: (restoreFocus?: boolean) => void,
  layer = node,
) {
  const doc = node.ownerDocument;
  const view = doc.defaultView!;
  let frame = 0;
  const position = () => {
    const rect = anchor.getBoundingClientRect();
    if (
      !anchor.isConnected ||
      rect.right <= rect.left ||
      rect.bottom <= rect.top ||
      rect.bottom < 0 ||
      rect.top > view.innerHeight ||
      rect.right < 0 ||
      rect.left > view.innerWidth
    ) {
      onClose(false);
      return;
    }
    node.style.width = `${Math.max(1, Math.min(440, view.innerWidth - 16))}px`;
    node.style.maxHeight = `${Math.max(1, Math.min(480, view.innerHeight - 16))}px`;
    const p = placePopover(rect, node.getBoundingClientRect().height, {
      width: view.innerWidth,
      height: view.innerHeight,
    });
    Object.assign(node.style, {
      left: `${p.left}px`,
      top: `${p.top}px`,
      maxHeight: `${p.maxHeight}px`,
    });
    node.dataset.placement = p.placement;
  };
  const schedule = () => {
    view.cancelAnimationFrame(frame);
    frame = view.requestAnimationFrame(position);
  };
  const outside = (event: MouseEvent) => {
    const target = event.target as Node | null;
    if (target && !node.contains(target)) {
      // Consume dismissal so a click cannot also change Stock or close a parent dialog.
      event.preventDefault();
      event.stopPropagation();
      onClose(true);
    }
  };
  const popover = typeof layer.showPopover === 'function';
  if (popover) layer.showPopover();
  else (layer as HTMLDialogElement).showModal();
  position();
  node.querySelector<HTMLInputElement>('input:not([disabled])')?.focus();
  layer.addEventListener('click', outside);
  view.addEventListener('resize', schedule);
  doc.addEventListener('scroll', schedule, true);
  const observer = new ResizeObserver(schedule);
  observer.observe(anchor);
  observer.observe(node);
  return () => {
    view.cancelAnimationFrame(frame);
    observer.disconnect();
    layer.removeEventListener('click', outside);
    view.removeEventListener('resize', schedule);
    doc.removeEventListener('scroll', schedule, true);
    if (popover && layer.matches(':popover-open')) layer.hidePopover();
    else if (!popover && (layer as HTMLDialogElement).open) (layer as HTMLDialogElement).close();
  };
}

/** Wire through a Svelte handler, so delegated child handlers run before propagation stops. */
export function handlePopoverKey(
  event: KeyboardEvent,
  body: HTMLElement | undefined,
  onClose: (restoreFocus?: boolean) => void,
) {
  event.stopPropagation();
  if (event.key === 'Escape') event.preventDefault();
  handleModalKey(event, body ?? null, () => onClose(true));
}
