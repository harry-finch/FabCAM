import { describe, expect, it } from 'vitest';
import { errorMessage, errorHint } from './error-display';
import { lookup } from '../i18n/i18n-core';
import en from '../i18n/messages/en.json';
import de from '../i18n/messages/de.json';
import type { WiacError } from '../api/types';

describe('motion guard diagnostics', () => {
  for (const locale of ['en', 'de'] as const) {
    it(`renders actionable ${locale} refusals using stable backend metadata`, () => {
      for (const reason of [
        'helical_entry',
        'legacy_overcut',
        'back_text',
        'back_approach',
        'back_source',
        'back_pattern',
        'two_sided_depth',
        'two_sided_holding',
      ]) {
        const e: WiacError = {
          kind: 'misconfigured',
          message: 'backend fallback',
          recovery_hint: 'backend hint',
          params: { safety_guard: reason },
        };
        const t = (key: Parameters<typeof lookup>[2]) => lookup({ en, de }, locale, key);
        expect(errorMessage(e, t)).not.toBe(e.message);
        expect(errorMessage(e, t)).not.toContain('error.safety.');
        expect(errorHint(e, t)).not.toBe(e.recovery_hint);
      }
    });
  }
});
