import type { VersionResponse } from './types';
export function assertCuttingCapability(
  version: VersionResponse,
  project?: { operations?: readonly { params?: { stages?: { contract?: string } | null } }[] },
): void {
  if (!version.capabilities.includes('cutting-parameters-v10'))
    throw new Error(
      'The running engine does not support explicit v10 cutting parameters. Restart it with the updated FabCAM build before generating.',
    );
  if (
    project?.operations?.some((op) => op.params?.stages?.contract === 'pocket_finishing_v2') &&
    !version.capabilities.includes('pocket-finishing-v2')
  )
    throw new Error(
      'The running engine does not support explicit Pocket finishing scopes. Restart it with the updated FabCAM build; no fallback has been applied.',
    );
}
