export interface ExportStatus {
  stale: boolean;
  fixture?: number;
  backFixture?: number;
  critical: number;
  backCritical: number;
  workArea: number;
  backWorkArea: number;
}
export function exportSafetyProblem(
  status: ExportStatus,
  side: 'front' | 'back',
  settings: { blockOnCriticalSimWarnings: boolean; blockOnWorkAreaViolation: boolean },
) {
  const fixture = (side === 'back' ? status.backFixture : status.fixture) ?? 0;
  if (fixture > 0) return { key: 'genbar.error.block_critical' as const, count: fixture };
  const critical = side === 'back' ? status.backCritical : status.critical;
  const area = side === 'back' ? status.backWorkArea : status.workArea;
  if (settings.blockOnCriticalSimWarnings && critical > 0)
    return { key: 'genbar.error.block_critical' as const, count: critical };
  if (settings.blockOnWorkAreaViolation && area > 0)
    return { key: 'genbar.error.block_work_area' as const, count: area };
  return undefined;
}
