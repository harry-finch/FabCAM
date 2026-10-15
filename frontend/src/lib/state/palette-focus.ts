import { tick } from 'svelte';
/** Let the owner's dialog autofocus first; navigation focuses a visible destination. */
export async function focusPaletteDestination() {
  await tick();
  if (document.querySelector('[role="dialog"][aria-modal="true"]')) return;
  const visible = (element: HTMLElement) =>
    element.getClientRects().length > 0 && !element.closest('.tab-hidden');
  const active = [...document.querySelectorAll<HTMLElement>('[role=region].active')].find(visible);
  if (active) {
    active.focus();
    return;
  }
  const region = [...document.querySelectorAll<HTMLElement>('main')].find(visible);
  const focus = region?.querySelector<HTMLElement>(
    'button:not(:disabled),input:not(:disabled),a[href],[tabindex="0"]',
  );
  if (focus) {
    focus.focus();
    return;
  }
  // Navigation buttons persist while a lazy workspace finishes loading.
  const tab = document.querySelector<HTMLElement>('.main-tabs .main-tab.active');
  if (tab && visible(tab)) {
    tab.focus();
    return;
  }
  if (region) {
    region.tabIndex = -1;
    region.focus();
  }
}
