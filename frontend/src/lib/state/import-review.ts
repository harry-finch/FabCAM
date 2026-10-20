// Import summary gate (ADR-0026). The open-file flows await `reviewImport`
// between the importer call and the project commit, so a cancelled review
// changes nothing and a unit choice is applied by re-importing the same
// source with an explicit scale. Fatal importer errors never reach here.
//
// Plain TypeScript with the Svelte store contract (`subscribe`), so the
// services importing it stay rune-free and testable in node.
import type { ImportResponse } from '../api/types';
import { needsReview } from './import-units';

export interface ImportReviewRequest {
  result: ImportResponse;
  /// Re-run the importer on the same bytes/path with an explicit scale.
  /// Absent in view mode (already committed source).
  rescale?: (scale: number) => Promise<ImportResponse>;
}

type Listener = (request: ImportReviewRequest | null) => void;

class ImportReviewStore {
  private current: ImportReviewRequest | null = null;
  private resolver: ((r: ImportResponse | null) => void) | null = null;
  private listeners = new Set<Listener>();

  get pending(): ImportReviewRequest | null {
    return this.current;
  }

  subscribe(listener: Listener): () => void {
    this.listeners.add(listener);
    listener(this.current);
    return () => {
      this.listeners.delete(listener);
    };
  }

  private set(request: ImportReviewRequest | null) {
    this.current = request;
    for (const l of this.listeners) l(request);
  }

  /// Confirm-mode review; resolves with the result to commit or null.
  review(request: ImportReviewRequest): Promise<ImportResponse | null> {
    this.finish(null);
    this.set(request);
    return new Promise((resolve) => {
      this.resolver = resolve;
    });
  }

  /// Read-only summary of an already imported source.
  show(result: ImportResponse) {
    this.finish(null);
    this.set({ result });
  }

  finish(result: ImportResponse | null) {
    const resolve = this.resolver;
    this.resolver = null;
    this.set(null);
    resolve?.(result);
  }
}

export const importReview = new ImportReviewStore();

/// Gate used by every open/add-drawing flow. Clean imports with reliable
/// units pass straight through; otherwise the user reviews warnings and
/// confirms (or chooses) the source units. `null` = cancelled.
export async function reviewImport(
  result: ImportResponse,
  rescale: (scale: number) => Promise<ImportResponse>,
): Promise<ImportResponse | null> {
  if (!needsReview(result)) return result;
  return importReview.review({ result, rescale });
}
