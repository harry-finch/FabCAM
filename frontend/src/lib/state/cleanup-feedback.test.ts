import { expect, it } from 'vitest';
import { cleanupBlockReason, cleanupError } from './cleanup-feedback';
const state = {
  busy: false,
  stale: false,
  conflict: '',
  selected: 1,
  preview: false,
  failed: false,
};
it('distinguishes failed preview from stale analysis and selection', () => {
  expect(cleanupBlockReason(state, 'apply')).toBe('cleanup.preview_first');
  expect(cleanupBlockReason({ ...state, failed: true }, 'apply')).toBe('cleanup.preview_failed');
  expect(cleanupBlockReason({ ...state, failed: true, stale: true }, 'apply')).toBe(
    'cleanup.stale',
  );
  expect(cleanupBlockReason({ ...state, selected: 0 }, 'preview')).toBe('cleanup.select_first');
  expect(cleanupBlockReason({ ...state, preview: true }, 'apply')).toBe('');
});
it('shows structured error text and recovery without JSON', () => {
  expect(
    cleanupError(
      new Error(
        JSON.stringify({
          kind: 'bad_input',
          message: 'Crossing edges',
          recovery_hint: 'Review geometry',
        }),
      ),
    ),
  ).toBe('Crossing edges Review geometry');
  expect(cleanupError(new Error('Connection failed'))).toBe('Connection failed');
});

it('keeps protective precedence while allowing a preview retry', () => {
  expect(
    cleanupBlockReason({ ...state, conflict: 'tabs in Rough #3', selected: 0 }, 'preview'),
  ).toBe('cleanup.placement_block');
  expect(cleanupBlockReason({ ...state, busy: true, stale: true }, 'apply')).toBe('cleanup.busy');
  expect(cleanupBlockReason({ ...state, failed: true }, 'preview')).toBe('');
});
it('handles optional hints, object payloads, unexpected JSON and cancellation', () => {
  expect(
    cleanupError({ kind: 'bad_input', message: 'Operations Rough (#3), Finish (#5) blocked' }),
  ).toBe('Operations Rough (#3), Finish (#5) blocked');
  expect(cleanupError(new Error('{"unexpected":true}'))).not.toContain('{');
  expect(cleanupError({ mystery: 1 })).not.toContain('[object Object]');
  expect(cleanupError(new Error('Cancelled'))).toBe('Cancelled');
  expect(cleanupError(new Error('Connection lost\n at internal()'))).toBe('Connection lost');
});
