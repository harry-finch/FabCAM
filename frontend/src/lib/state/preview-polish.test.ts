import { describe, expect, it } from 'vitest';
import { programLabel } from './program-label';
import { formatDuration } from '../components/generate-time';
describe('preview presentation', () => {
  it('uses emitted extension and remains neutral when absent', () => {
    expect(programLabel('sbp', 'Program')).toBe('OpenSBP');
    expect(programLabel('nc', 'Program')).toBe('Program (.nc)');
    expect(programLabel(undefined, 'Program')).toBe('Program');
    expect(programLabel('invalid/name', 'Program')).toBe('Program');
  });
  it('formats duration boundaries with explicit units', () => {
    expect(formatDuration(119.9)).toBe('2 min 0 s');
    expect(formatDuration(3599.9)).toBe('1 h 0 min 0 s');
    expect(formatDuration(0)).toBe('0 s');
    expect(formatDuration(NaN)).toBe('—');
  });
});
