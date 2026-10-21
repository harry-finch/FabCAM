import { t } from '../i18n';
import type { components } from '../api/generated';
type S = components['schemas'];
export function rejectionReason(reason: S['FsUnavailableReason']) {
  switch (reason.kind) {
    case 'invalid_request':
      return t('fs36.reason.invalid_request');
    case 'dataset_mismatch':
      return t('fs36.reason.dataset_mismatch');
    case 'missing_material':
      return t('fs36.reason.missing_material');
    case 'unsupported_tool':
      return t('fs36.reason.unsupported_tool');
    case 'unsupported_operation':
      return t('fs36.reason.unsupported_operation');
    case 'missing_cutting_data':
      return t('fs36.reason.missing_cutting_data');
    case 'ambiguous_profiles':
      return t('fs36.reason.ambiguous_profiles');
    case 'unrepresentable_output':
      return t('fs36.reason.unrepresentable_output');
  }
}
export function scopeField(field: string) {
  switch (field) {
    case 'mode':
      return t('fs.mode');
    case 'machine':
      return t('fs35.machine');
    case 'material':
      return t('fs.material');
    case 'tool':
      return t('fs35.tool');
    case 'operation':
      return t('fs35.operation');
    case 'doc_mm':
      return t('fs36.pass_depth');
    case 'thickness_mm':
      return t('stock.thickness');
    case 'diameter_proximity':
      return t('fs39.relative_gap');
    case 'cutting_family':
      return t('fs36.scope.cutting_family');
    case 'diameter_mm':
      return t('fs36.scope.diameter_mm');
    case 'flute_count':
      return t('fs36.scope.flute_count');
    case 'entry':
      return t('fs36.scope.entry');
    case 'engagement':
      return t('fs36.scope.engagement');
    default:
      return field;
  }
}
export function evidence(value: S['FsEvidence']) {
  return t(
    value === 'reference_data'
      ? 'fs35.reference'
      : value === 'tested'
        ? 'fs35.tested'
        : value === 'validated'
          ? 'fs35.validated'
          : 'fs35.theoretical',
  );
}
