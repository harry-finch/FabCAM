import type { components } from '../api/generated';
import type { PresetRow } from './preset-library';
import type { LoadedSource } from './feeds-speeds-sources';
export type SelectedRecord = components['schemas']['FsSelectedRecord'];
export function recordSelection(row: PresetRow): SelectedRecord | undefined {
  return row.record
    ? {
        pack: { ...row.pack.manifest.identity },
        record_id: row.record.source_record_id,
        profile: { ...row.profile.identity },
      }
    : undefined;
}
/** Eligibility presentation only. Applicability always comes from Rust. */
export function unavailableRecord(row: PresetRow): boolean {
  return (
    !row.enabled ||
    !row.milling ||
    !row.record ||
    row.pack.manifest.distribution === 'external_reference' ||
    !!(
      row.record.lab &&
      (row.record.lab.state === 'draft' || !row.record.lab.enabled || row.record.lab.deleted)
    )
  );
}
export function sourceFingerprint(rows: LoadedSource[]): string {
  return JSON.stringify(
    rows.map((r) => [r.id, r.contents]).sort((a, b) => a[0].localeCompare(b[0])),
  );
}
export function assertSameSources(preview: LoadedSource[], fresh: LoadedSource[]): void {
  if (sourceFingerprint(preview) !== sourceFingerprint(fresh))
    throw new Error(
      'Cutting sources changed since preview. Refresh and review the parameters again.',
    );
}

/** Shared preview/apply boundary. The commit callback is synchronous and owns Undo/Redo. */
export async function applyReviewedCutting<T>(options: {
  previewSources: LoadedSource[];
  readSources: () => Promise<LoadedSource[]>;
  isCurrent: () => boolean;
  requiresAcknowledgement: boolean;
  acknowledged: boolean;
  validate: () => Promise<T>;
  commit: (result: T) => void;
}): Promise<void> {
  const check = () => {
    if (!options.isCurrent())
      throw new Error('The operation or preview changed. Review again before applying.');
  };
  check();
  if (options.requiresAcknowledgement && !options.acknowledged)
    throw new Error('Confirm replacement of manual cutting values before applying.');
  assertSameSources(options.previewSources, await options.readSources());
  check();
  const result = await options.validate();
  assertSameSources(options.previewSources, await options.readSources());
  check();
  options.commit(result);
}
