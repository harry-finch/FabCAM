// NUMSAFE: the main-thread wasm fallback cannot re-instantiate a trapped
// instance, so after a trap every later call must refuse explicitly.
import { describe, expect, it, vi } from 'vitest';
import { isFatalWasmError, WasmFatalError } from './wasm-fatal';
import type { GenerateRequest } from './types';

const calls = vi.hoisted(() => ({ generate: 0, version: 0 }));
vi.mock('ivac-wasm', () => ({
  default: async () => undefined,
  generate: () => {
    calls.generate += 1;
    if (calls.generate === 1) throw new WebAssembly.RuntimeError('unreachable');
    return { gcode: 'G0 X0' };
  },
  version: () => {
    calls.version += 1;
    return { version: 'test' };
  },
  healthz: () => ({ ok: true }),
}));

describe('wasm fatal classification', () => {
  it('treats traps and stack overflows as fatal, API errors as recoverable', () => {
    expect(isFatalWasmError(new WebAssembly.RuntimeError('unreachable'))).toBe(true);
    expect(isFatalWasmError(new RangeError('Maximum call stack size exceeded'))).toBe(true);
    expect(isFatalWasmError(new Error('bad input'))).toBe(false);
    expect(isFatalWasmError({ code: 'bad_input', message: 'depth' })).toBe(false);
  });
});

describe('WasmWiacClient (main-thread fallback) after a trap', () => {
  it('refuses every later call instead of running on the trapped instance', async () => {
    const { WasmWiacClient } = await import('./wasm');
    const client = new WasmWiacClient();
    await expect(client.version()).resolves.toEqual({ version: 'test' });
    await expect(client.generate({} as GenerateRequest)).rejects.toBeInstanceOf(
      WebAssembly.RuntimeError,
    );
    await expect(client.generate({} as GenerateRequest)).rejects.toBeInstanceOf(WasmFatalError);
    await expect(client.version()).rejects.toThrow(/reload/);
    expect(calls).toEqual({ generate: 1, version: 1 });
  });
});
