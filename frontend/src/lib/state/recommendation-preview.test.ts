import { describe, expect, it } from 'vitest';
import type { components } from '../api/generated';
import {
  PreviewRequests,
  adaptationKind,
  displayNumber,
  distinctEngagement,
} from './recommendation-preview';
type Derivation = components['schemas']['FsDerivation'];
const derivation = (patch: Partial<Derivation> = {}): Derivation => ({
  policy: 'test',
  quality: 'close',
  method: 'bounded_normalized_reference',
  requested_diameter_mm: 6.35,
  source_diameters_mm: [6.35],
  source_profiles: [],
  relative_delta: 0,
  chipload_factor: 1,
  doc_factor: 1,
  machine_transfer: false,
  material_transfer: false,
  slot_basis_for_pocket: false,
  ...patch,
});
describe('recommendation presentation facts', () => {
  it('does not call an unchanged diameter adapted, even for a Close reference', () => {
    expect(adaptationKind(derivation())).toBe('unchanged');
    expect(adaptationKind(derivation({ requested_diameter_mm: 6 }))).toBe('diameter');
    expect(adaptationKind(derivation({ slot_basis_for_pocket: true }))).toBe('slot_to_pocket');
    expect(adaptationKind(derivation({ machine_transfer: true }))).toBe('context');
    expect(adaptationKind()).toBe('unknown');
  });
  it('distinguishes interpolation and real differences before display rounding', () => {
    expect(
      adaptationKind(derivation({ source_diameters_mm: [3.175, 6.35], interpolation_weight: 0.5 })),
    ).toBe('interpolation');
    expect(adaptationKind(derivation({ requested_diameter_mm: 6.3499 }))).toBe('diameter');
  });
  it('formats display only and preserves distinctions between engagement and stepover', () => {
    const value = 0.095294117;
    expect(displayNumber(value, 3)).toBe('0.095');
    expect(value).toBe(0.095294117);
    expect(displayNumber(17000)).toBe('17,000');
    expect(distinctEngagement(0.5, 0.5)).toBe(false);
    expect(distinctEngagement(0.5, 0.4)).toBe(true);
    expect(distinctEngagement(0.5, null)).toBe(true);
  });
});
describe('unapplied asynchronous preview ownership', () => {
  it('keeps the newest mode result when responses arrive out of order', async () => {
    const requests = new PreviewRequests();
    let visible = '',
      busy = false;
    const operation = { feed: 1200, origin: 'manual' };
    const resolvers: (() => void)[] = [];
    async function calculate(mode: string) {
      const id = requests.start();
      busy = true;
      await new Promise<void>((resolve) => resolvers.push(resolve));
      if (requests.owns(id)) {
        visible = mode;
        busy = false;
      }
    }
    const prudent = calculate('prudent');
    const normal = calculate('normal');
    const rapid = calculate('rapid');
    resolvers[1]();
    await normal;
    expect(busy).toBe(true);
    expect(visible).toBe('');
    resolvers[2]();
    await rapid;
    resolvers[0]();
    await prudent;
    expect(visible).toBe('rapid');
    expect(busy).toBe(false);
    expect(operation).toEqual({ feed: 1200, origin: 'manual' });
  });
  it('invalidates preview and apply responses on cancellation, including reopen', () => {
    const requests = new PreviewRequests();
    const pending = requests.start();
    requests.cancel();
    expect(requests.owns(pending)).toBe(false);
    const reopened = requests.start();
    expect(requests.owns(pending)).toBe(false);
    expect(requests.owns(reopened)).toBe(true);
  });
});
