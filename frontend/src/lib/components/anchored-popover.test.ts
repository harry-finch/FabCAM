import { afterEach, describe, expect, it, vi } from 'vitest';
import { attachAnchoredPopover, placePopover, handlePopoverKey } from './anchored-popover';
const viewport = { width: 1280, height: 720 };

describe('anchored popup viewport bounds', () => {
  it('anchors below an inspector trigger and expands left beyond the inspector', () => {
    expect(
      placePopover({ left: 1100, right: 1270, top: 100, bottom: 128 }, 420, viewport),
    ).toMatchObject({ left: 830, top: 132, width: 440, placement: 'below' });
  });
  it('flips above a trigger near the bottom', () => {
    expect(
      placePopover({ left: 1100, right: 1270, top: 630, bottom: 660 }, 420, viewport),
    ).toMatchObject({ left: 830, top: 206, placement: 'above' });
  });
  it.each([
    { width: 680, height: 800 },
    { width: 320, height: 480 },
    { width: 320, height: 240 },
  ])('stays bounded at $width × $height with long/expanded content', (view) => {
    const result = placePopover(
      { left: 0, right: 100, top: view.height / 2, bottom: view.height / 2 + 28 },
      800,
      view,
    );
    expect(result.left).toBeGreaterThanOrEqual(8);
    expect(result.top).toBeGreaterThanOrEqual(8);
    expect(result.left + result.width).toBeLessThanOrEqual(view.width - 8);
    expect(result.top + Math.min(800, result.maxHeight)).toBeLessThanOrEqual(view.height - 8);
  });
  it('uses a compact centered position when neither anchored side is usable', () => {
    expect(
      placePopover({ left: 100, right: 200, top: 100, bottom: 128 }, 420, {
        width: 320,
        height: 240,
      }).placement,
    ).toBe('center');
  });
});

function setup(popover = true) {
  const keys: Record<string, (event: KeyboardEvent) => void> = {};
  const clicks: Record<string, (event: MouseEvent) => void> = {};
  const disconnected = vi.fn();
  vi.stubGlobal(
    'ResizeObserver',
    class {
      observe = vi.fn();
      disconnect = disconnected;
    },
  );
  const view = {
    innerWidth: 1280,
    innerHeight: 720,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    requestAnimationFrame: vi.fn((_callback: FrameRequestCallback) => 1),
    cancelAnimationFrame: vi.fn(),
  };
  const doc = {
    defaultView: view,
    activeElement: null as unknown,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
  };
  const first = {
    focus: vi.fn(() => {
      doc.activeElement = first;
    }),
  };
  const last = { focus: vi.fn() };
  const anchor = {
    isConnected: true,
    getBoundingClientRect: () => ({ left: 1100, right: 1270, top: 100, bottom: 128 }),
  };
  const node = {
    ownerDocument: doc,
    style: {},
    dataset: {},
    getBoundingClientRect: () => ({ height: 420 }),
    querySelector: () => first,
    querySelectorAll: () => [first, last],
    contains: (value: unknown) => value === first || value === last,
    addEventListener: vi.fn((key, callback) => {
      keys[key] = callback;
    }),
    removeEventListener: vi.fn(),
  };
  const layer = {
    showPopover: popover ? vi.fn() : undefined,
    showModal: vi.fn(),
    open: true,
    close: vi.fn(),
    hidePopover: vi.fn(),
    matches: () => true,
    addEventListener: vi.fn((key, callback) => {
      clicks[key] = callback;
    }),
    removeEventListener: vi.fn(),
  };
  const close = vi.fn();
  const cleanup = attachAnchoredPopover(
    node as unknown as HTMLElement,
    anchor as unknown as HTMLElement,
    close,
    layer as unknown as HTMLElement,
  );
  keys.keydown = (event) => handlePopoverKey(event, node as unknown as HTMLElement, close);
  return {
    keys,
    clicks,
    disconnected,
    view,
    doc,
    first,
    last,
    anchor,
    node,
    layer,
    close,
    cleanup,
  };
}
afterEach(() => vi.unstubAllGlobals());
describe('top-layer popup lifecycle and nested dialog ownership', () => {
  it('opens in the top layer, positions and focuses search, then releases listeners/observers', () => {
    const f = setup();
    expect(f.layer.showPopover).toHaveBeenCalledOnce();
    expect(f.node.style).toMatchObject({ left: '830px', top: '132px', width: '440px' });
    expect(f.first.focus).toHaveBeenCalledOnce();
    expect(f.close).not.toHaveBeenCalled();
    f.cleanup();
    expect(f.disconnected).toHaveBeenCalledOnce();
    expect(f.node.addEventListener).not.toHaveBeenCalledWith('keydown', expect.any(Function));
    expect(f.view.removeEventListener).toHaveBeenCalledWith('resize', expect.any(Function));
    expect(f.layer.hidePopover).toHaveBeenCalledOnce();
  });
  it('uses a native dialog top layer when Popover is unavailable', () => {
    const f = setup(false);
    expect(f.layer.showModal).toHaveBeenCalledOnce();
    expect(f.first.focus).toHaveBeenCalledOnce();
    f.cleanup();
    expect(f.layer.close).toHaveBeenCalledOnce();
  });
  it('dismisses a hidden trigger after navigation instead of leaving an orphaned layer', () => {
    const f = setup();
    f.anchor.getBoundingClientRect = () => ({ left: 0, right: 0, top: 0, bottom: 0 });
    const resize = f.view.addEventListener.mock.calls.find(([name]) => name === 'resize')![1];
    resize();
    f.view.requestAnimationFrame.mock.calls[0][0](0);
    expect(f.close).toHaveBeenCalledWith(false);
    f.cleanup();
  });
  it('Escape requests focus restoration and cannot bubble to the adaptation dialog', () => {
    const f = setup();
    const event = { key: 'Escape', stopPropagation: vi.fn(), preventDefault: vi.fn() };
    f.keys.keydown(event as unknown as KeyboardEvent);
    expect(f.close).toHaveBeenCalledWith(true);
    expect(event.stopPropagation).toHaveBeenCalled();
    expect(event.preventDefault).toHaveBeenCalled();
    f.cleanup();
  });
  it('keeps Tab in the popup, independently of the parent modal trap', () => {
    const f = setup();
    f.doc.activeElement = f.last;
    const event = {
      key: 'Tab',
      shiftKey: false,
      stopPropagation: vi.fn(),
      preventDefault: vi.fn(),
    };
    f.keys.keydown(event as unknown as KeyboardEvent);
    expect(event.preventDefault).toHaveBeenCalledOnce();
    expect(f.first.focus).toHaveBeenCalledTimes(2);
    expect(f.close).not.toHaveBeenCalled();
    f.cleanup();
  });
  it('consumes an outside click without changing material or activating background controls', () => {
    const f = setup();
    const inside = { target: f.first, preventDefault: vi.fn(), stopPropagation: vi.fn() };
    f.clicks.click(inside as unknown as MouseEvent);
    expect(f.close).not.toHaveBeenCalled();
    const outside = { target: f.layer, preventDefault: vi.fn(), stopPropagation: vi.fn() };
    f.clicks.click(outside as unknown as MouseEvent);
    expect(f.close).toHaveBeenCalledWith(true);
    expect(outside.preventDefault).toHaveBeenCalledOnce();
    expect(outside.stopPropagation).toHaveBeenCalledOnce();
    f.cleanup();
  });
});
