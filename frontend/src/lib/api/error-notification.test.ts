import { describe, expect, it } from 'vitest';
import { notificationError, notificationText } from './error-notification';

describe('notification errors', () => {
  it('preserves direct backend objects, encoded errors and actionable metadata', () => {
    const error = {
      kind: 'misconfigured',
      message: 'Invalid fixture dimensions',
      recovery_hint: 'Edit Clamp',
      code: 'tool_too_large',
      params: { tool_id: 2 },
    };
    expect(notificationError(error)).toEqual(error);
    expect(notificationError(new Error(JSON.stringify(error)))).toEqual(error);
    expect(notificationText(error)).toBe(error.message);
  });
  it('reads only safe message fields and never renders arbitrary object coercion', () => {
    for (const input of [
      { kind: 'misconfigured', message: '[object Object]' },
      {},
      { message: { fixture: 4 } },
      '[object Object]',
      undefined,
      42,
    ]) {
      expect(notificationText(input)).not.toContain('[object Object]');
      expect(notificationText(input)).toContain('Unable to complete');
    }
    expect(notificationText({ message: 'Export is blocked.' })).toBe('Export is blocked.');
    expect(notificationText(new Error('Readable error'))).toBe('Readable error');
  });
});
