import type { VersionResponse } from './types';
export function assertCuttingCapability(version: VersionResponse): void {
  if (!version.capabilities.includes('cutting-parameters-v9'))
    throw new Error(
      'The running engine does not support explicit v9 cutting parameters. Restart it with the updated FabCAM build before generating.',
    );
}
