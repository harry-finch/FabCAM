import { tryParseStructuredError } from './client';
import type { WiacError } from './types';

/** Whitelist user-facing fields; never call String() on an arbitrary object. */
export function notificationError(input: unknown): string | WiacError {
  const structured = tryParseStructuredError(input instanceof Error ? input.message : input);
  if (structured && !structured.message.includes('[object Object]')) return structured;
  const candidate = input instanceof Error ? input.message : input;
  if (typeof candidate === 'string' && candidate.trim() && !candidate.includes('[object Object]'))
    return candidate;
  if (candidate && typeof candidate === 'object') {
    const message = (candidate as { message?: unknown }).message;
    if (typeof message === 'string') return notificationError(message);
  }
  return 'Unable to complete this action. Review the setup and try again.';
}

export function notificationText(input: unknown): string {
  const error = notificationError(input);
  return typeof error === 'string' ? error : error.message;
}
