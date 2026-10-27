import { tryParseStructuredError } from '../api/client';

import { notificationError } from '../api/error-notification';

export function cleanupError(error: unknown): string {
  const input = error instanceof Error ? error.message : error;
  const detail = tryParseStructuredError(input);
  if (detail) return [detail.message, detail.recovery_hint].filter(Boolean).join(' ');
  const fallback = notificationError(error);
  const message = typeof fallback === 'string' ? fallback : fallback.message;
  if (message.trimStart().startsWith('{') || message.trimStart().startsWith('['))
    return 'Unable to complete drawing cleanup. Review technical details and retry.';
  return message.split('\n')[0];
}

export function cleanupTechnicalDetails(error: unknown): string {
  if (error instanceof Error) return error.message;
  if (typeof error === 'string') return error;
  try {
    return JSON.stringify(error) ?? '';
  } catch {
    return '';
  }
}

export function cleanupBlockReason(
  s: {
    busy: boolean;
    stale: boolean;
    conflict: string;
    selected: number;
    preview: boolean;
    failed: boolean;
  },
  action: 'preview' | 'apply',
): string {
  if (s.busy) return 'cleanup.busy';
  if (s.stale) return 'cleanup.stale';
  if (s.conflict) return 'cleanup.placement_block';
  if (!s.selected) return 'cleanup.select_first';
  if (action === 'apply' && !s.preview)
    return s.failed ? 'cleanup.preview_failed' : 'cleanup.preview_first';
  return '';
}
