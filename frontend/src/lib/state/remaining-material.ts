import type { RemainingMaterial } from '../api/types';

// Unsupported and stale results are never interpreted as zero residual.
export function remainingView(
  results: readonly RemainingMaterial[] | undefined,
  stale: boolean,
  opId: number | null,
): RemainingMaterial | null {
  if (stale || !results?.length) return null;
  return (
    results.find((r) => r.op_id === opId) ?? results.find((r) => r.status === 'ready') ?? results[0]
  );
}
