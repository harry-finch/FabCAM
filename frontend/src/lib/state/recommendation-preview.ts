import type { components } from '../api/generated';
type S = components['schemas'];
/** Transient request ownership; cancellation also invalidates pending apply responses. */
export class PreviewRequests {
  private revision = 0;
  start() {
    return ++this.revision;
  }
  cancel() {
    ++this.revision;
  }
  owns(revision: number) {
    return revision === this.revision;
  }
}
/** Classify only facts supplied by derivation; never infer geometry from formatted labels. */
export function adaptationKind(d?: S['FsDerivation'] | null) {
  if (!d) return 'unknown';
  if (d.interpolation_weight != null) return 'interpolation';
  if (d.source_diameters_mm.some((v) => Math.abs(v - d.requested_diameter_mm) > 1e-9))
    return 'diameter';
  if (d.slot_basis_for_pocket) return 'slot_to_pocket';
  if (d.material_transfer || d.machine_transfer) return 'context';
  return 'unchanged';
}
export function displayNumber(value: number, digits = 0) {
  return new Intl.NumberFormat('en-GB', { maximumFractionDigits: digits }).format(value);
}
/** Nominal engagement is still available in details; only identical displayed ratios are deduplicated. */
export function distinctEngagement(requested: number, proposed?: number | null) {
  return proposed == null || Math.abs(requested - proposed) > 1e-9;
}
