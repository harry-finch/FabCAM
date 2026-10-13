import { describe, it, expect } from 'vitest';
import { remainingView } from './remaining-material';
import type { RemainingMaterial } from '../api/types';
const ready: RemainingMaterial = {
  op_id: 2,
  status: 'ready',
  reason: 'resolved_path_at_depth',
  depth_mm: -2,
  area_mm2: 4,
  largest_area_mm2: 4,
  regions: [],
};
const skipped: RemainingMaterial = {
  ...ready,
  op_id: 1,
  status: 'unsupported',
  reason: 'intentional_tabs',
  area_mm2: 0,
};
describe('remaining analysis scope and freshness', () => {
  it('never presents stale or missing results as an empty machined region', () => {
    expect(remainingView([ready], true, 2)).toBeNull();
    expect(remainingView(undefined, false, 2)).toBeNull();
  });
  it('keeps explicit unsupported context instead of selecting a reassuring ready result', () => {
    expect(remainingView([skipped, ready], false, 1)?.status).toBe('unsupported');
    expect(remainingView([skipped, ready], false, null)?.op_id).toBe(2);
  });
  it('does not accumulate unrelated operations or manufacture support for unknown states', () => {
    expect(remainingView([{ ...skipped, status: 'unknown' }, ready], false, 1)?.status).toBe(
      'unknown',
    );
    expect(remainingView([ready], false, 2)?.area_mm2).toBe(4);
  });
});
