// Advisory UI policy for the bundled ShopBot post. The post remains authoritative.
import type { OpEntry } from './op_types';
import type { MachineSettings, WorkOffset } from './project-types';
export type EntrySupport = 'direct' | 'ramp' | 'helix' | 'tabs' | 'leads' | 'origin';
export function shopbotEntrySupport(
  op: OpEntry,
  machine: MachineSettings,
  fallbackPost: string,
  origin?: WorkOffset,
): EntrySupport | null {
  if (
    (machine.gcodeDialect ?? fallbackPost) !== 'cps' ||
    machine.cpsPost?.source !== 'bundled' ||
    machine.cpsPost.bundledId !== 'shopbot'
  )
    return null;
  if (origin && (origin.wcs !== 'G54' || origin.z_mm !== 0)) return 'origin';
  if (op.kind !== 'profile' && op.kind !== 'pocket') return null;
  const entry = op.plunge?.kind ?? 'direct';
  if (entry === 'helix' || (op.kind === 'pocket' && op.pocketStrategy === 'trochoidal'))
    return 'helix';
  if (entry === 'ramp') {
    if (op.tabsActive || (op.tabMode && op.tabMode.kind !== 'off')) return 'tabs';
    if ((op.leadInKind ?? 'off') !== 'off' || (op.leadOutKind ?? 'off') !== 'off') return 'leads';
    return 'ramp';
  }
  return 'direct';
}
